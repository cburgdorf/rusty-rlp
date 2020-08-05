#![feature(test)]

use rlp;
use rstest::rstest;

#[rstest(input, expected,
    case("dog", "[83, 64, 6f, 67]"),
    case("hello world", "[8b, 68, 65, 6c, 6c, 6f, 20, 77, 6f, 72, 6c, 64]"),
    case(
        "Hello there, I am a very very long string, and I am going get encoded in RLP!",
        "[b8, 4d, 48, 65, 6c, 6c, 6f, 20, 74, 68, 65, 72, 65, 2c, 20, 49, 20, 61, 6d, 20, 61, 20, 76, 65, 72, 79, 20, 76, 65, 72, 79, 20, 6c, 6f, 6e, 67, 20, 73, 74, 72, 69, 6e, 67, 2c, 20, 61, 6e, 64, 20, 49, 20, 61, 6d, 20, 67, 6f, 69, 6e, 67, 20, 67, 65, 74, 20, 65, 6e, 63, 6f, 64, 65, 64, 20, 69, 6e, 20, 52, 4c, 50, 21]"
    )
)]
fn encode_strings(input: &str, expected: &str) {
    let mut stream = rlp::RlpStream::new();
    stream.append(&input);
    assert_eq!(format!("{:x?}", stream.out()), expected);
}
