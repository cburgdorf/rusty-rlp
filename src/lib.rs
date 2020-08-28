use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyTuple};
use pyo3::wrap_pyfunction;

use rlp::{PayloadInfo, Prototype, Rlp};

mod errors;

use crate::errors::{DecodingError, EncodingError, RlpDecoderError};

// We use this to abstract between both types to not have to rely on calling to_object(py) to achieve that.
enum ListOrBytes<'a> {
    List(&'a PyList),
    Bytes(&'a PyBytes),
}

impl ToPyObject for ListOrBytes<'_> {
    fn to_object(&self, py: Python) -> PyObject {
        match *self {
            ListOrBytes::List(ref val) => val.to_object(py),
            ListOrBytes::Bytes(ref val) => val.to_object(py),
        }
    }
}

fn _has_trailing_bytes(payload_info: &PayloadInfo, item_len: usize, rlp: &Rlp) -> bool {
    payload_info.header_len + item_len < rlp.as_raw().len()
}

fn _wrap_as_list_if_some(val: Option<&PyList>) -> Option<ListOrBytes> {
    match val {
        Some(val) => Some(ListOrBytes::List(val)),
        None => None,
    }
}

fn _decode_raw<'a>(
    strict: bool,
    preserve_cache_info: bool,
    rlp_val: rlp::Rlp,
    py: pyo3::Python<'a>,
) -> Result<(ListOrBytes<'a>, Option<ListOrBytes<'a>>), PyErr> {
    match rlp_val.prototype() {
        Ok(Prototype::Null) => errors::construct_invariant_error(),
        Ok(Prototype::Data(len)) => {
            if strict {
                let payload_info = rlp_val.payload_info().map_err(RlpDecoderError)?;
                if _has_trailing_bytes(&payload_info, len, &rlp_val) {
                    return errors::construct_trailing_bytes_error(&payload_info);
                }

                if let [prefix, val] = rlp_val.as_raw() {
                    if prefix == &129 && (val >= &0 && val <= &127) {
                        return errors::construct_short_string_error(val);
                    }
                }
            }

            let decoded_val =
                ListOrBytes::Bytes(PyBytes::new(py, rlp_val.data().map_err(RlpDecoderError)?));

            let rlp_val = if preserve_cache_info {
                Some(ListOrBytes::List(PyList::new(
                    py,
                    vec![PyBytes::new(py, rlp_val.as_raw())],
                )))
            } else {
                // We don't want to allocate any unnecessary Python values if we don't have to preserve cache info
                None
            };

            Ok((decoded_val, rlp_val))
        }
        Ok(Prototype::List(len)) => {
            let payload_info = rlp_val.payload_info().map_err(RlpDecoderError)?;
            if strict && len == 0 && _has_trailing_bytes(&payload_info, len, &rlp_val) {
                return errors::construct_trailing_bytes_error(&payload_info);
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
                let item = rlp_val.at(i).map_err(RlpDecoderError)?;
                if strict
                    && rlp_val.as_raw().len() > (payload_info.header_len + payload_info.value_len)
                {
                    return errors::construct_trailing_bytes_error(&payload_info);
                // TODO: Investigate if that is the correct way to decide about termination of non-strict decoding
                } else if !strict && item.as_raw() == [0] {
                    return Ok((ListOrBytes::List(current), _wrap_as_list_if_some(rlp_info)));
                }

                match _decode_raw(strict, preserve_cache_info, item, py) {
                    Ok(decoded_and_info) => {
                        // Handle the decoded item itself
                        if let (ListOrBytes::List(thing), _) = decoded_and_info {
                            current.append(thing)?
                        } else if let (ListOrBytes::Bytes(thing), _) = decoded_and_info {
                            current.append(thing)?
                        }
                        // Handle the preserved info if we need to
                        if let Some(_rlp_info) = rlp_info {
                            if let (_, Some(ListOrBytes::List(info))) = decoded_and_info {
                                _rlp_info.append(info)?
                            } else if let (_, Some(ListOrBytes::Bytes(info))) = decoded_and_info {
                                _rlp_info.append(info)?
                            }
                        }
                    }
                    _ => return errors::construct_invariant_error(),
                }
            }
            Ok((ListOrBytes::List(current), _wrap_as_list_if_some(rlp_info)))
        }
        Err(e) => Err(DecodingError::py_err(format!("{:?}", e))),
    }
}

fn _encode_raw<'a>(
    stream: &'a mut rlp::RlpStream,
    val: &PyAny,
    py: pyo3::Python,
) -> Result<&'a mut rlp::RlpStream, pyo3::PyErr> {
    if let Ok(list_item) = val.downcast::<PyList>() {
        stream.begin_unbounded_list();
        for item in list_item {
            _encode_raw(stream, item, py)?;
        }
        stream.finalize_unbounded_list();
        Ok(stream)
    } else if let Ok(list_item) = val.downcast::<PyTuple>() {
        stream.begin_unbounded_list();
        for item in list_item {
            _encode_raw(stream, item, py)?;
        }
        stream.finalize_unbounded_list();
        Ok(stream)
    } else if let Ok(bytes_item) = val.downcast::<PyBytes>() {
        stream.append(&bytes_item.as_bytes());
        Ok(stream)
    } else {
        Err(EncodingError::py_err(format!(
            "Can not encode value {:?}",
            val
        )))
    }
}

#[pyfunction]
fn encode_raw(val: PyObject, py: pyo3::Python) -> PyResult<PyObject> {
    let mut rlp_stream = rlp::RlpStream::new();
    match _encode_raw(&mut rlp_stream, &val.cast_as(py).unwrap(), py) {
        Ok(_) => Ok(PyBytes::new(py, &rlp_stream.out()).to_object(py)),
        Err(e) => Err(e),
    }
}

#[pyfunction]
fn decode_raw(
    rlp_val: Vec<u8>,
    strict: bool,
    preserve_cache_info: bool,
    py: pyo3::Python,
) -> PyResult<PyObject> {
    _decode_raw(strict, preserve_cache_info, rlp::Rlp::new(&rlp_val), py).map(|result| {
        match result {
            (decoded, None) => (decoded, PyList::empty(py).to_object(py)),
            (decoded, Some(val)) => (decoded, val.to_object(py)),
        }
        .to_object(py)
    })
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
