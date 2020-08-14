#![feature(test)]
use hex_literal::hex;
use rlp;
use rstest::rstest;

#[rstest(input, expected,
    //case(vec![101, 102], "e"), //should't this blow up, it doesn't?
    case(vec![0x83, 0x64, 0x6f, 0x67], "dog"),
    case(
        vec![0xb8, 0x4d, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x2c, 0x20, 0x49, 0x20, 0x61, 0x6d, 0x20, 0x61, 0x20, 0x76, 0x65, 0x72, 0x79, 0x20, 0x76, 0x65, 0x72, 0x79, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x49, 0x20, 0x61, 0x6d, 0x20, 0x67, 0x6f, 0x69, 0x6e, 0x67, 0x20, 0x67, 0x65, 0x74, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x52, 0x4c, 0x50, 0x21],
        "Hello there, I am a very very long string, and I am going get encoded in RLP!",
    )
)]
fn decode_strings(input: Vec<u8>, expected: &str) {
    let out: String = rlp::decode(&input).unwrap();
    assert_eq!(&out, expected);
}

#[test]
fn decode_block_header() {
    // Block Header example from
    // https://ethereum.stackexchange.com/a/67333
    // https://etherscan.io/block/400000
    let block_header = hex!("f90213a01e77d8f1267348b516ebc4f4da1e2aa59f85f0cbd853949500ffac8bfc38ba14a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347942a65aca4d5fc5b5c859090a6c34d164135398226a00b5e4386680f43c224c5c037efc0b645c8e1c3f6b30da0eec07272b4e6f8cd89a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b901000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000086057a418a7c3e83061a80832fefd880845622efdc96d583010202844765746885676f312e35856c696e7578a03fbea7af642a4e20cd93a945a1f5e23bd72fc5261153e09102cf718980aeff38886af23caae95692ef");
    let rlp = rlp::Rlp::new(&block_header);
    let parent_hash: Vec<u8> = rlp.val_at(0).unwrap();
    let uncle_hash: Vec<u8> = rlp.val_at(1).unwrap();
    let coinbase: Vec<u8> = rlp.val_at(2).unwrap();
    let state_root: Vec<u8> = rlp.val_at(3).unwrap();
    let tx_root: Vec<u8> = rlp.val_at(4).unwrap();
    let receipt_root: Vec<u8> = rlp.val_at(5).unwrap();
    let bloom: Vec<u8> = rlp.val_at(6).unwrap();
    let difficulty: Vec<u8> = rlp.val_at(7).unwrap();
    let block_number: Vec<u8> = rlp.val_at(8).unwrap();
    let gas_limit: Vec<u8> = rlp.val_at(9).unwrap();
    let gas_used: Vec<u8> = rlp.val_at(10).unwrap();
    let block_time: Vec<u8> = rlp.val_at(11).unwrap();
    let extra: Vec<u8> = rlp.val_at(12).unwrap();
    let mix_digest: Vec<u8> = rlp.val_at(13).unwrap();
    let nonce: Vec<u8> = rlp.val_at(14).unwrap();

    assert_eq!(
        hex::encode(parent_hash),
        "1e77d8f1267348b516ebc4f4da1e2aa59f85f0cbd853949500ffac8bfc38ba14"
    );
    assert_eq!(
        hex::encode(uncle_hash),
        "1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
    );
    assert_eq!(
        hex::encode(coinbase),
        "2a65aca4d5fc5b5c859090a6c34d164135398226"
    );
    assert_eq!(
        hex::encode(state_root),
        "0b5e4386680f43c224c5c037efc0b645c8e1c3f6b30da0eec07272b4e6f8cd89"
    );
    assert_eq!(
        hex::encode(tx_root),
        "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
    );
    assert_eq!(
        hex::encode(receipt_root),
        "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
    );
    assert_eq!(hex::encode(bloom), "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(hex::encode(difficulty), "057a418a7c3e");
    assert_eq!(hex::encode(block_number), "061a80");
    assert_eq!(hex::encode(gas_limit), "2fefd8");
    assert_eq!(hex::encode(gas_used), "");
    assert_eq!(hex::encode(block_time), "5622efdc");
    assert_eq!(
        hex::encode(extra),
        "d583010202844765746885676f312e35856c696e7578"
    );
    assert_eq!(
        hex::encode(mix_digest),
        "3fbea7af642a4e20cd93a945a1f5e23bd72fc5261153e09102cf718980aeff38"
    );
    assert_eq!(hex::encode(nonce), "6af23caae95692ef");
}
