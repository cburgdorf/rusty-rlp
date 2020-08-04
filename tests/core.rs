#![feature(test)]

use bytes::{Bytes, BytesMut, BufMut};
use hex_literal::hex;
use rlp::{Rlp, RlpStream, Encodable, DecoderError, Decodable};

extern crate test;

// LALALA, this file is pure junk, look another way!

#[derive(Debug, Clone)]
struct FictiveType {
    val1: Bytes,
    val2: [u8; 9],
    val3: [u8; 32],
    val4: [u8; 32],
}

impl Decodable for FictiveType {
    fn decode(r: &Rlp) -> Result<Self, DecoderError> {

        //let mut val1: [u8; 1] = Default::default();
        //val1.copy_from_slice(r.at(0)?.data()?);
        let val1 = BytesMut::from(r.at(0)?.data()?).freeze();

        let mut val2: [u8; 9] = Default::default();
        val2.copy_from_slice(r.at(1)?.data()?);

        let mut val3: [u8; 32] = Default::default();
        val3.copy_from_slice(r.at(2)?.data()?);

        let mut val4: [u8; 32] = Default::default();
        val4.copy_from_slice(r.at(3)?.data()?);

        let val = FictiveType {
            val1: val1,
            val2: val2,
            val3: val3,
            val4: val4,
        };

        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_encode_u64(b: &mut Bencher) {
        b.iter(|| {
			let mut stream = rlp::RlpStream::new();
			stream.append(&0x1023_4567_89ab_cdefu64);
			let _ = stream.out();
		})
    }


    // #[test]
    // fn test_encode_list() {
    //     let x = [hex!("05"), hex!("010efbef67941f79b2"), hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"), hex!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470")];
    //     rlp::encode_list(x);
    //     //let data = hex!("f84d0589010efbef67941f79b2a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
    //     //let rlp = rlp::Rlp::new(&data);
    //     //assert_eq!(format!("{}", rlp), "[\"0x05\", \"0x010efbef67941f79b2\", \"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421\", \"0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470\"]");
    // }

    #[test]
    fn test_cat() {
        let mut stream = rlp::RlpStream::new();
        //stream.begin_unbounded_list();
        stream
            .begin_list(4)
            .append(&5u64)
            .append(&577777777777u64)
            .append(&400000000000u64)
            .append(&200000000000u64);
            //.append_raw(&hex!("010efbef67941f79b2"), 1);
            // .append_raw(&hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"), 1)
            // .append_raw(&hex!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"), 1);
            //.encoder().encode_value(&hex!("05"));// .append_list(&hex!("05"));
            // stream.encoder().encode_value(&hex!("010efbef67941f79b2"))
            // stream.encoder().encode_value(&hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"))
            // stream.encoder().encode_value(&hex!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"));
        //assert!(!stream.is_finished());
        //stream.finalize_unbounded_list();
        //assert!(stream.is_finished());
        // 0x010efbef67941f79b2 + b"2";
        // let items = [400u32, b"cat", 2u32];
        // let encoded = rlp::encode_list(&items);
        // let decoded:std::vec::Vec<u32> = rlp::decode_list(&encoded);
        //println!("{}",hex::encode(stream.out()));

        assert_eq!(format!("{:?}", hex::encode(stream.out())), "\"d3058586863d3c71855d21dba000852e90edd000\"");
        //assert_eq!(format!("{:?}", stream.drain()), "[1, 2]");
    }

    #[test]
    fn test_rlp_display() {
        let data = hex!("f84d0589010efbef67941f79b2a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
        // let rlp_bytes: std::vec::Vec<u8> = rlp::decode_list(&data);
        // //assert_eq!(format!("{:?}", rlp_bytes), "[\"0x05\", \"0x010efbef67941f79b2\", \"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421\", \"0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470\"]");
        //print!("{:?}", rlp_bytes)

        let rlp = rlp::Rlp::new(&data);
        assert_eq!(format!("{}", rlp), "[\"0x05\", \"0x010efbef67941f79b2\", \"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421\", \"0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470\"]");
    }

    #[test]
    fn test_rlp_display_something() {
        let data = hex!("d3058586863d3c71855d21dba000852e90edd000");
        // let rlp_bytes: std::vec::Vec<u8> = rlp::decode_list(&data);
        // //assert_eq!(format!("{:?}", rlp_bytes), "[\"0x05\", \"0x010efbef67941f79b2\", \"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421\", \"0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470\"]");
        //print!("{:?}", rlp_bytes)

        let rlp = rlp::Rlp::new(&data);

        // let mut val1: [u8; 1] = Default::default();
        // val1.copy_from_slice(rlp.at(0).unwrap().data().unwrap());
        print!("AAA{:?}", rlp.val_at::<u64>(1).unwrap());

        assert_eq!(format!("{}", rlp), "[\"0x05\", \"0x86863d3c71\", \"0x5d21dba000\", \"0x2e90edd000\"]");
    }

    #[test]
    fn test_rlp_display2() {
        let data = hex!("f84d0589010efbef67941f79b2a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
        // let rlp_bytes: std::vec::Vec<u8> = rlp::decode_list(&data);
        // //assert_eq!(format!("{:?}", rlp_bytes), "[\"0x05\", \"0x010efbef67941f79b2\", \"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421\", \"0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470\"]");
        //print!("{:?}", rlp_bytes)

        let rlp: FictiveType = rlp::decode(&data).unwrap();
        println!("{:?}", rlp);

        println!("{}",hex::encode(rlp.val1));
        println!("{}",hex::encode(rlp.val2));
        println!("{}",hex::encode(rlp.val3));
        println!("{}",hex::encode(rlp.val4));
        //89010efbef67941f79b2
        //assert_eq!(format!("{:?}", rlp), "[\"0x05\", \"0x010efbef67941f79b2\", \"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421\", \"0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470\"]");
        //assert_eq!(format!("{}", rlp), "[\"0x05\", \"0x010efbef67941f79b2\", \"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421\", \"0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470\"]");
    }

    #[bench]
    fn bench_rlp_display(b: &mut Bencher) {
        b.iter(|| {
            let data = hex!("f84d0589010efbef67941f79b2a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
            let rlp = rlp::Rlp::new(&data);
            assert_eq!(format!("{}", rlp), "[\"0x05\", \"0x010efbef67941f79b2\", \"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421\", \"0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470\"]");
        })
    }

    // fn bench_encode_u256c (b: &mut Bencher) {
	// 	b.iter(|| {
	// 		let mut stream = rlp::RlpStream::new();
	// 		let uint: primitive_types::U256 = "8090a0b0c0d0e0f00910203040506077000000000000000100000000000012f0".into();
	// 		stream.append(&uint);
	// 		let _ = stream.out();
	// 	})
    // }

    #[bench]
    fn bench_encode_1000_empty_lists(b: &mut Bencher) {
		b.iter(|| {
			let mut stream = rlp::RlpStream::new_list(1000);
			for _ in 0..1000 {
				stream.begin_list(0);
			}
			let _ = stream.out();
		})
	}

}