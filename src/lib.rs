use pyo3::exceptions::Exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList};
use pyo3::{create_exception, wrap_pyfunction};

use rlp::Prototype;

create_exception!(rusty_rlp, EncodingError, Exception);
create_exception!(rusty_rlp, DecodingError, Exception);

// We can not implement From for rlp::DecoderError as it is in a foreign crate. Hence, we use
// map_err and implement From on _DecoderError instead.
struct _DecoderError(rlp::DecoderError);
impl std::convert::From<_DecoderError> for PyErr {
    fn from(err: _DecoderError) -> PyErr {
        DecodingError::py_err(err.0.to_string())
    }
}

fn _decode_raw(strict: bool, r: rlp::Rlp, py: pyo3::Python) -> Result<PyObject, PyErr> {
    match r.prototype() {
        Ok(Prototype::Null) => Err(DecodingError::py_err("Invariant")),
        Ok(Prototype::Data(len)) => {
            if strict {
                let payload_info = r.payload_info().map_err(_DecoderError)?;
                if payload_info.header_len + len < r.as_raw().len() {
                    return Err(DecodingError::py_err("Trailing bytes"));
                }
            }

            Ok(PyBytes::new(py, r.data().map_err(_DecoderError)?).to_object(py))
        }
        Ok(Prototype::List(len)) => {
            let payload_info = r.payload_info().map_err(_DecoderError)?;
            if strict && len == 0 && payload_info.header_len + len < r.as_raw().len() {
                return Err(DecodingError::py_err("Trailing bytes"));
            }
            let current = PyList::empty(py);
            for i in 0..len {
                let (item, offset) = r.at_with_offset(i).map_err(_DecoderError)?;
                if offset > payload_info.value_len {
                    if strict {
                        return Err(DecodingError::py_err("Trailing bytes"));
                    } else {
                        continue;
                    }
                }
                match item.prototype() {
                    Ok(Prototype::Data(_)) => current.append(
                        PyBytes::new(py, item.data().map_err(_DecoderError)?).to_object(py),
                    )?,
                    Ok(Prototype::List(_)) => current.append(_decode_raw(strict, item, py)?)?,
                    Err(e) => return Err(DecodingError::py_err(format!("{:?}", e))),
                    _ => return Err(DecodingError::py_err("Invariant")),
                }
            }
            Ok(current.to_object(py))
        }
        Err(e) => Err(DecodingError::py_err(format!("{:?}", e))),
    }
}

fn _encode_raw<'a>(
    stream: &'a mut rlp::RlpStream,
    val: &PyAny,
    py: pyo3::Python,
) -> Result<&'a mut rlp::RlpStream, pyo3::PyErr> {
    // TODO: Support any sequence or iterable here
    if let Ok(list_item) = val.downcast::<PyList>() {
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
    let mut r = rlp::RlpStream::new();
    match _encode_raw(&mut r, &val.cast_as(py).unwrap(), py) {
        Ok(_) => Ok(PyBytes::new(py, &r.out()).to_object(py)),
        Err(e) => Err(e),
    }
}

#[pyfunction]
fn decode_raw(rlp_bytes: Vec<u8>, strict: bool, py: pyo3::Python) -> PyResult<PyObject> {
    _decode_raw(strict, rlp::Rlp::new(&rlp_bytes), py)
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
