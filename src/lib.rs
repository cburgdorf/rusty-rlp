use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use pyo3::types::{PyBytes, PyList};

use rlp::{DecoderError, Prototype};


struct UpstreamRLP<'a> {
  _rlp: rlp::Rlp<'a>,
}


fn to_py(r: rlp::Rlp, py: pyo3::Python) -> Result<PyObject, DecoderError> {
  match r.prototype() {
      Ok(Prototype::Null) => Err(DecoderError::Custom("Invariant")),
      Ok(Prototype::Data(_)) => Ok(PyBytes::new(py, r.data().unwrap()).to_object(py)),
      Ok(Prototype::List(_)) => {
          let current = PyList::empty(py);
          for item in r.iter() {
              match item.prototype() {
                Ok(Prototype::Data(_)) => current.append(PyBytes::new(py, item.data().unwrap()).to_object(py)).unwrap(),
                Ok(Prototype::List(_)) => current.append(to_py(item, py).unwrap()).unwrap(),
                Err(e) => return Err(e),
                _ => return Err(DecoderError::Custom("Invariant")),
              };
          }
          Ok(current.to_object(py))
      }
      Err(e) => Err(e),
  }
}

// TODO: We currently do not use this because I couldn't figure out the lifetime error that
// we get in decode_raw if we rely on auto-conversion. Probably fixable, so leaving it in as a reminder.
impl pyo3::IntoPy<PyObject> for UpstreamRLP<'_>{
  fn into_py(self, py: pyo3::Python) -> PyObject {
    to_py(self._rlp, py).unwrap()
  }
}


fn enc<'a>(stream: &'a mut rlp::RlpStream, val: &PyAny, py: pyo3::Python) -> &'a mut rlp::RlpStream {

  // TODO: Support any sequence or iterable here
  if let Ok(list_item) = val.downcast::<PyList>() {
      stream.begin_unbounded_list();
      for item in list_item {
        enc(stream, item, py);
      }
      stream.finalize_unbounded_list();
      stream
  } else if let Ok(bytes_item) = val.downcast::<PyBytes>() {
    stream.append(&bytes_item.as_bytes());
    stream
  } else {
    panic!("Failed to encode object")
  }
}

#[pyfunction]
fn encode_raw(val: PyObject, py: pyo3::Python) -> PyResult<PyObject> {
  let mut r = rlp::RlpStream::new();
  enc(&mut r, &val.cast_as(py).unwrap(), py);

  Ok(PyBytes::new(py, &r.out()).to_object(py))

}

#[pyfunction]
fn decode_raw(rlp_bytes: Vec<u8>, py: pyo3::Python) -> PyResult<PyObject> {
  Ok(to_py(rlp::Rlp::new(&rlp_bytes), py).unwrap())
}


#[pyfunction]
fn decode_fictive_type(rlp_bytes: Vec<u8>) -> PyResult<(u64, u64, u64, u64)> {
  let rlp = rlp::Rlp::new(&rlp_bytes);

  Ok((
    rlp.val_at::<u64>(0).unwrap(),
    rlp.val_at::<u64>(1).unwrap(),
    rlp.val_at::<u64>(2).unwrap(),
    rlp.val_at::<u64>(3).unwrap()
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

    Ok(())
}