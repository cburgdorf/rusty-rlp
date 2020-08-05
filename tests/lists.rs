#![feature(test)]

use rlp;
use rstest::rstest;

enum Data {
    Str(String),
    Int(u16),
}


#[test]
fn encode_list_with_strings_and_ints() {
    let sample = vec![Data::Str("dog".to_string()), Data::Str("mouse".to_string()), Data::Str("tigers".to_string()), Data::Int(127)];
    let mut stream = rlp::RlpStream::new();
    stream.begin_list(4);
    for item in sample.iter() {
        match item {
            Data::Str(text) => stream.append(&&text[..]),
            Data::Int(val) => stream.append(val)
        };
    }

    assert_eq!(stream.out(), vec![0xd2, 0x83, 0x64, 0x6f, 0x67, 0x85, 0x6d, 0x6f, 0x75, 0x73, 0x65, 0x86, 0x74, 0x69, 0x67, 0x65, 0x72, 0x73, 0x7f]);
}
