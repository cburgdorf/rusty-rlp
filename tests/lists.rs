#![feature(test)]

use std::str;
use rlp;
use rlp::Prototype;
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

// Test case from Py-RLP: https://github.com/ethereum/pyrlp/blob/37396698aeb949932e70a53fa10f3046b7915bf3/tests/test_raw_sedes.py#L6-L15
#[test]
fn encode_nested_list_with_strings() {
    //let sample = vec![b"fdsa", [b"dfs", [b"jfdkl"]]];
    let mut stream = rlp::RlpStream::new();
    stream.begin_list(2)
          .append(&"fdsa")
          .begin_list(2)
          .append(&"dfs")
          .begin_list(1)
          .append(&"jfdkl");

    assert_eq!(stream.out(), vec![209, 132, 102, 100, 115, 97, 203, 131, 100, 102, 115, 198, 133, 106, 102, 100, 107, 108]);
}

// def swalk(rlp):
//     if not isinstance(rlp, tuple):
//         return rlp

//     current = "["
//     for item in rlp:
//         if isinstance(item, tuple):
//             current += swalk(item)
//         else:
//             current +=  str(item)
    
//     current += "]"

//     return current


#[test]
fn decode_foo() {
    let rlp = rlp::Rlp::new(&[209, 132, 102, 100, 115, 97, 203, 131, 100, 102, 115, 198, 133, 106, 102, 100, 107, 108]);

    fn swalk(r: rlp::Rlp) -> String {
        match r.prototype() {
			Ok(Prototype::Null) => format!("{}",""),
			Ok(Prototype::Data(_)) => format!("{:x?}", String::from_utf8_lossy(r.data().unwrap())),
			Ok(Prototype::List(_)) => {
                let mut current = "[".to_string();
                for item in r.iter() {
                    match item.prototype() {
                        Ok(Prototype::Data(_)) => current.push_str(&format!("{:x?}", String::from_utf8_lossy(r.data().unwrap()))),
                        Ok(Prototype::List(_)) => current.push_str(&swalk(item)),
                        _ => panic!("meh")
                    }
                }
                current.push_str("]");
                current
            }
            _ => panic!("woot"),
        }
    }

    let x = swalk(rlp);
    println!("{}", x);

}