use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyTuple};
use pyo3::wrap_pyfunction;

use rlp::{PayloadInfo, Prototype, Rlp};

mod errors;

use crate::errors::{DecodingError, EncodingError, ToPyErr};

// We use this to abstract between both types to not have to rely on calling to_object(py) to achieve that.
enum ListOrBytes<'a> {
    List(&'a PyList),
    Bytes(&'a PyBytes),
}

struct DecodingInfo<'a>{
    decoded_val: ListOrBytes<'a>,
    cache_info: Option<ListOrBytes<'a>>
}

impl ToPyObject for ListOrBytes<'_> {
    fn to_object(&self, py: Python) -> PyObject {
        match *self {
            ListOrBytes::List(ref val) => val.to_object(py),
            ListOrBytes::Bytes(ref val) => val.to_object(py),
        }
    }
}

impl ToPyObject for DecodingInfo<'_> {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            DecodingInfo { decoded_val: val, cache_info: None } => {
                (val, PyList::empty(py).to_object(py))
            },
            DecodingInfo { decoded_val: val, cache_info: Some(info) } => {
                (val, info.to_object(py))
            },
        }.to_object(py)
    }
}

fn _has_trailing_bytes(payload_info: &PayloadInfo, item_len: usize, rlp: &Rlp) -> bool {
    payload_info.header_len + item_len < rlp.as_raw().len()
}

fn _decode_raw<'a>(
    strict: bool,
    preserve_cache_info: bool,
    rlp_val: rlp::Rlp,
    py: pyo3::Python<'a>,
) -> PyResult<DecodingInfo<'a>> {
    match rlp_val.prototype() {
        Ok(Prototype::Null) => Err(errors::construct_invariant_error()),
        Ok(Prototype::Data(len)) => {
            if strict {
                let payload_info = rlp_val.payload_info().map_err(|err| err.to_py_err())?;
                if _has_trailing_bytes(&payload_info, len, &rlp_val) {
                    return Err(errors::construct_trailing_bytes_error(&payload_info));
                }

                if let [prefix, val] = rlp_val.as_raw() {
                    if prefix == &129 && (val >= &0 && val <= &127) {
                        return Err(errors::construct_short_string_error(val));
                    }
                }
            }

            let decoded_val =
                ListOrBytes::Bytes(PyBytes::new(py, rlp_val.data().map_err(|err| err.to_py_err())?));

            let cache_info = if preserve_cache_info {
                Some(ListOrBytes::List(PyList::new(
                    py,
                    vec![PyBytes::new(py, rlp_val.as_raw())],
                )))
            } else {
                // We don't want to allocate any unnecessary Python values if we don't have to preserve cache info
                None
            };

            Ok(DecodingInfo { decoded_val, cache_info })
        }
        Ok(Prototype::List(len)) => {
            let payload_info = rlp_val.payload_info().map_err(|err| err.to_py_err())?;
            if strict && len == 0 && _has_trailing_bytes(&payload_info, len, &rlp_val) {
                return Err(errors::construct_trailing_bytes_error(&payload_info));
            }
            // TODO: Instead of creating an empty list early and then appending each item, we could instead
            // use an iterator and build up lists lazily, leveraging PyList::new(py, iterator) which would
            // be slightly more performant on the Python side.
            let current = PyList::empty(py);

            let rlp_info = if preserve_cache_info {
                Some(PyList::new(py, vec![PyBytes::new(py, rlp_val.as_raw())]))
            } else {
                None
            };

            for i in 0..len {
                let item = rlp_val.at(i).map_err(|err| err.to_py_err())?;
                if strict
                    && rlp_val.as_raw().len() > (payload_info.header_len + payload_info.value_len)
                {
                    return Err(errors::construct_trailing_bytes_error(&payload_info));
                // TODO: Investigate if that is the correct way to decide about termination of non-strict decoding
                } else if !strict && item.as_raw() == [0] {
                    return Ok(DecodingInfo {
                        decoded_val: ListOrBytes::List(current),
                        cache_info: rlp_info.map(|i| ListOrBytes::List(i))
                    })
                }

                match _decode_raw(strict, preserve_cache_info, item, py) {
                    Ok(decoding_info) => {
                        // Handle the decoded item itself
                        if let ListOrBytes::List(thing) = decoding_info.decoded_val {
                            current.append(thing)?
                        } else if let ListOrBytes::Bytes(thing) = decoding_info.decoded_val {
                            current.append(thing)?
                        }
                        // Handle the preserved info if we need to
                        if let Some(_rlp_info) = rlp_info {
                            if let Some(ListOrBytes::List(info)) = decoding_info.cache_info {
                                _rlp_info.append(info)?
                            } else if let Some(ListOrBytes::Bytes(info)) = decoding_info.cache_info {
                                _rlp_info.append(info)?
                            }
                        }
                    }
                    _ => return Err(errors::construct_invariant_error()),
                }
            }
           Ok(DecodingInfo {
                decoded_val: ListOrBytes::List(current),
                cache_info: rlp_info.map(|i| ListOrBytes::List(i))
           })
        }
        Err(e) => Err(DecodingError::py_err(format!("{:?}", e))),
    }
}

fn _encode_raw(
    stream: &mut rlp::RlpStream,
    val: &PyAny,
    py: pyo3::Python,
) -> PyResult<()> {
    if let Ok(list_item) = val.downcast::<PyList>() {
        return _encode_list(stream, list_item.iter().collect(), py);
    } else if let Ok(list_item) = val.downcast::<PyTuple>() {
        return _encode_list(stream, list_item.iter().collect(), py);
    } else if let Ok(bytes_item) = val.downcast::<PyBytes>() {
        stream.append(&bytes_item.as_bytes());
        return Ok(());
    }

    Err(EncodingError::py_err(format!(
        "Can not encode value {:?}",
        val
    )))
}

fn _encode_list(
    stream: &mut rlp::RlpStream,
    items: Vec<&PyAny>,
    py: pyo3::Python,
) -> PyResult<()> {
    stream.begin_unbounded_list();
    for item in items {
        _encode_raw(stream, item, py)?;
    }
    stream.finalize_unbounded_list();
    Ok(())
}

#[pyfunction]
fn encode_raw(val: PyObject, py: pyo3::Python) -> PyResult<PyObject> {
    let mut rlp_stream = rlp::RlpStream::new();
    _encode_raw(&mut rlp_stream, &val.cast_as(py).unwrap(), py)?;
    Ok(PyBytes::new(py, &rlp_stream.out()).to_object(py))
}

#[pyfunction]
fn decode_raw(
    rlp_val: Vec<u8>,
    strict: bool,
    preserve_cache_info: bool,
    py: pyo3::Python,
) -> PyResult<PyObject> {
    _decode_raw(strict, preserve_cache_info, rlp::Rlp::new(&rlp_val), py)
        .map(|result| result.to_object(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_rlp(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(decode_raw))?;
    module.add_wrapped(wrap_pyfunction!(encode_raw))?;
    module.add("DecodingError", _py.get_type::<DecodingError>())?;
    module.add("EncodingError", _py.get_type::<EncodingError>())?;

    Ok(())
}
