use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use pyo3::types::{PyBytes, PyList};

use rlp::Prototype;


struct UpstreamRLP<'a> {
  _rlp: rlp::Rlp<'a>,
}


fn to_py(r: rlp::Rlp, py: pyo3::Python) -> PyObject {
  match r.prototype() {
      // FIXME: Get rid of the panics
      // Fixme: Get rid of all the unwraps
      Ok(Prototype::Null) => panic!("null"),
      Ok(Prototype::Data(_)) => PyBytes::new(py, r.data().unwrap()).to_object(py),
      Ok(Prototype::List(_)) => {
          let current = PyList::empty(py);
          for item in r.iter() {
              match item.prototype() {
                Ok(Prototype::Data(_)) => current.append(PyBytes::new(py, item.data().unwrap()).to_object(py)).unwrap(),
                Ok(Prototype::List(_)) => current.append(to_py(item, py)).unwrap(),
                  _ => panic!("meh")
              };
          }
          current.to_object(py)
      }
      _ => panic!("woot"),
  }
}

// TODO: We currently do not use this because I couldn't figure out the lifetime error that
// we get in decode_raw if we rely on auto-conversion. Probably fixable, so leaving it in as a reminder.
impl pyo3::IntoPy<PyObject> for UpstreamRLP<'_>{
  fn into_py(self, py: pyo3::Python) -> PyObject {
    to_py(self._rlp, py)
  }
}


fn enc<'a>(stream: &'a mut rlp::RlpStream, val: &PyAny, py: pyo3::Python) -> &'a mut rlp::RlpStream {
  if py.is_instance::<PyList, _>(val).unwrap() {
    let current_list: &PyList = val.downcast().unwrap();
    stream.begin_unbounded_list();
    for item in current_list {
      enc(stream, item, py);
    }
    stream.finalize_unbounded_list();
    stream
  } else {
    let item_bytes: &PyBytes = val.downcast().unwrap();
    stream.append(&item_bytes.as_bytes());
    stream
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
  Ok(to_py(rlp::Rlp::new(&rlp_bytes), py))
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