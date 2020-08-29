use ethereum_types::H160;
use hex_literal::hex;
use rstest::rstest;

// are we testing anything in rusty-rlp here?

// This is encoded as a string. Wouldn't an address be formatted as a pure binary instead?
// https://github.com/paritytech/parity-common/blob/0431acb4f34751af44c664b0b0a6f36b0cd147b3/rlp/tests/tests.rs#L222-L231
#[rstest(input, expected,
    case(H160::from(hex!("ef2d6d194084c2de36e0dabfce45d046b37d1106")),
    vec![
        0x94, 0xef, 0x2d, 0x6d, 0x19, 0x40, 0x84, 0xc2, 0xde, 0x36, 0xe0, 0xda, 0xbf, 0xce, 0x45, 0xd0, 0x46, 0xb3,
        0x7d, 0x11, 0x06,
    ]),
)]
fn encode_bytes(input: H160, expected: Vec<u8>) {
    let mut stream = rlp::RlpStream::new();
    stream.append(&input);

    assert_eq!(stream.out(), expected);
}
