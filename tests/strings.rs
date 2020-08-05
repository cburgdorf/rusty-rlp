#![feature(test)]

use rlp;
use rstest::rstest;

#[rstest(input, expected,
    case("dog", "[83, 64, 6f, 67]"),
    case("hello world", "[8b, 68, 65, 6c, 6c, 6f, 20, 77, 6f, 72, 6c, 64]"),
)]
fn encode_strings(input: &str, expected: &str) {
    let mut stream = rlp::RlpStream::new();
    stream.append(&input);
    assert_eq!(format!("{:x?}", stream.out()), expected);
}
