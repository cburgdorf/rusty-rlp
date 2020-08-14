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

fn to_py(strict: bool, r: rlp::Rlp, py: pyo3::Python) -> Result<PyObject, PyErr> {
    match r.prototype() {
        Ok(Prototype::Null) => Err(DecodingError::py_err("Invariant")),
        Ok(Prototype::Data(len)) => {
            let payload_info = r.payload_info().map_err(_DecoderError)?;
            if strict && payload_info.header_len + len < r.as_raw().len() {
                return Err(DecodingError::py_err("Trailing bytes"));
            }
            Ok(PyBytes::new(py, r.data().map_err(_DecoderError)?).to_object(py))
        }
        Ok(Prototype::List(len)) => {
            let payload_info = rlp::PayloadInfo::from(r.as_raw()).unwrap();
            let current = PyList::empty(py);
            if strict && len == 0 && payload_info.header_len + len < r.as_raw().len() {
                return Err(DecodingError::py_err("Trailing bytes"));
            }
            for i in 0..len {
                let (item, offset) = r.at_with_offset(i).unwrap();
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
                    Ok(Prototype::List(_)) => current.append(to_py(strict, item, py)?)?,
                    Err(e) => return Err(DecodingError::py_err(format!("{:?}", e))),
                    _ => return Err(DecodingError::py_err("Invariant")),
                }
            }
            Ok(current.to_object(py))
        }
        Err(e) => Err(DecodingError::py_err(format!("{:?}", e))),
    }
}

fn enc<'a>(
    stream: &'a mut rlp::RlpStream,
    val: &PyAny,
    py: pyo3::Python,
) -> Result<&'a mut rlp::RlpStream, pyo3::PyErr> {
    // TODO: Support any sequence or iterable here
    if let Ok(list_item) = val.downcast::<PyList>() {
        stream.begin_unbounded_list();
        for item in list_item {
            enc(stream, item, py)?;
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
    match enc(&mut r, &val.cast_as(py).unwrap(), py) {
        Ok(_) => Ok(PyBytes::new(py, &r.out()).to_object(py)),
        Err(e) => Err(e),
    }
}

#[pyfunction]
fn decode_raw(rlp_bytes: Vec<u8>, strict: bool, py: pyo3::Python) -> PyResult<PyObject> {
    to_py(strict, rlp::Rlp::new(&rlp_bytes), py)
}

#[pyfunction]
fn decode_fictive_type(rlp_bytes: Vec<u8>) -> PyResult<(u64, u64, u64, u64)> {
    let rlp = rlp::Rlp::new(&rlp_bytes);

    Ok((
        rlp.val_at::<u64>(0).unwrap(),
        rlp.val_at::<u64>(1).unwrap(),
        rlp.val_at::<u64>(2).unwrap(),
        rlp.val_at::<u64>(3).unwrap(),
    ))
}

#[pyfunction]
fn encode_fictive_type(fictive_type: (u64, u64, u64, u64)) -> PyResult<Vec<u8>> {
    let mut stream = rlp::RlpStream::new();
    // //stream.begin_unbounded_list();
    let (val1, val2, val3, val4) = fictive_type;
    stream
        .begin_list(4)
        .append(&val1)
        .append(&val2)
        .append(&val3)
        .append(&val4);

    Ok(stream.out())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_rlp(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(decode_fictive_type))?;
    module.add_wrapped(wrap_pyfunction!(encode_fictive_type))?;
    module.add_wrapped(wrap_pyfunction!(decode_raw))?;
    module.add_wrapped(wrap_pyfunction!(encode_raw))?;
    module.add("DecodingError", _py.get_type::<DecodingError>())?;
    module.add("EncodingError", _py.get_type::<EncodingError>())?;

    Ok(())
}
