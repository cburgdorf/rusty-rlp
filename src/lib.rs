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

#[pyfunction]
fn decode_raw(rlp_bytes: Vec<u8>, py: pyo3::Python) -> PyResult<PyObject> {
  Ok(to_py(rlp::Rlp::new(&rlp_bytes), py))
}


//call with rusty_rlp.decode_fictive_type(b"\xf8M\x05\x89\x01\x0e\xfb\xefg\x94\x1fy\xb2\xa0V\xe8\x1f\x17\x1b\xccU\xa6\xff\x83E\xe6\x92\xc0\xf8n[H\xe0\x1b\x99l\xad\xc0\x01b/\xb5\xe3c\xb4!\xa0\xc5\xd2F\x01\x86\xf7#<\x92~}\xb2\xdc\xc7\x03\xc0\xe5\x00\xb6S\xca\x82';{\xfa\xd8\x04]\x85\xa4p")
// TODO: Return actual Python bytes: https://users.rust-lang.org/t/pyo3-best-way-to-return-bytes-from-function-call/46577/2
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

    Ok(())
}