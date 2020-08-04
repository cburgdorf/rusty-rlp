use bytes::{Bytes, BytesMut};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use hex_literal::hex;

#[pyfunction]
fn do_foo() -> PyResult<String> {
  let data = hex!("f84d0589010efbef67941f79b2a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
  let rlp = rlp::Rlp::new(&data);
  Ok(format!("{}", rlp))
}

#[pyfunction]
fn debug_me(rlp_bytes: Vec<u8>) -> PyResult<String> {
  Ok(format!("{:?}", rlp_bytes))
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

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_rlp(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(do_foo))?;
    m.add_wrapped(wrap_pyfunction!(decode_fictive_type))?;
    m.add_wrapped(wrap_pyfunction!(encode_fictive_type))?;
    m.add_wrapped(wrap_pyfunction!(debug_me))?;

    Ok(())
}