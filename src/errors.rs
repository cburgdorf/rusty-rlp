use pyo3::create_exception;
use pyo3::exceptions::Exception;
use pyo3::prelude::*;
use rlp::{PayloadInfo, DecoderError};

create_exception!(rusty_rlp, EncodingError, Exception);
create_exception!(rusty_rlp, DecodingError, Exception);


pub trait ToPyErr {
  fn to_py_err(&self) -> PyErr;
}

impl ToPyErr for DecoderError {
    fn to_py_err(&self) -> PyErr {
        DecodingError::py_err(self.to_string())
    }
}

pub fn construct_short_string_error(val: &u8) -> PyErr {
    DecodingError::py_err(format!(
        "Encoded {} as short string although single byte was possible",
        val
    ))
}

pub fn construct_invariant_error() -> PyErr {
    DecodingError::py_err("Invariant")
}

pub fn construct_trailing_bytes_error(payload_info: &PayloadInfo) -> PyErr {
    DecodingError::py_err(format!(
        "Trailing bytes. Payload Info {:?}",
        payload_info
    ))
}
