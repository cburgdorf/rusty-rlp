use pyo3::create_exception;
use pyo3::exceptions::Exception;
use pyo3::prelude::*;
use rlp::PayloadInfo;

create_exception!(rusty_rlp, EncodingError, Exception);
create_exception!(rusty_rlp, DecodingError, Exception);


pub trait ToDecodingError<T> {
  fn map_decoder_error(self) -> Result<T, PyErr>;
}

impl<T> ToDecodingError<T> for Result<T, rlp::DecoderError> {
  fn map_decoder_error(self) -> Result<T, PyErr> {
      self.map_err(|val | DecodingError::py_err(val.to_string()))
  }
}

pub fn construct_short_string_error<T>(val: &u8) -> Result<T, PyErr> {
    Err(DecodingError::py_err(format!(
        "Encoded {} as short string although single byte was possible",
        val
    )))
}

pub fn construct_invariant_error<T>() -> Result<T, PyErr> {
    Err(DecodingError::py_err("Invariant"))
}

pub fn construct_trailing_bytes_error<T>(payload_info: &PayloadInfo) -> Result<T, PyErr> {
    Err(DecodingError::py_err(format!(
        "Trailing bytes. Payload Info {:?}",
        payload_info
    )))
}
