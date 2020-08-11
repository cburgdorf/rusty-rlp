#![feature(test)]

use rlp;
use rstest::rstest;

#[rstest(input, expected,
    case(vec![101, 102], vec![101, 102]),
    case(b"asdf".to_vec(), b"asdf".to_vec()),
)]
fn encode_bytes(input: Vec<u8>, expected: Vec<u8>) {
    let mut stream = rlp::RlpStream::new();
    // We need to feed the values one by one because adding continues u8 at once is treated
    // as adding a string and hence result in a different encoding
    for i in input {
        stream.append(&i);
    }

    assert_eq!(stream.out(), expected);
}

#[rstest(input, expected,
    case(vec![101, 102], vec![101, 102]),
    case(b"asdf".to_vec(), b"asdf".to_vec()),
)]
fn encode_bytes2(input: Vec<u8>, expected: Vec<u8>) {
    let mut stream = rlp::RlpStream::new();
    // Alternatively instead, we can use append_raw
    stream.append_raw(&input, 1);

    assert_eq!(stream.out(), expected);
}
