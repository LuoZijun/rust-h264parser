
// 指数哥伦布编码
// https://en.wikipedia.org/wiki/Exponential-Golomb_coding
// 
// ue(v): codeNum = 2^leadingZeroBits − 1 + read_bits( leadingZeroBits )
// se(v): (−1)^(k+1) * Ceil( k÷2 )
// 
// RFC: Rec. ITU - T H.264 ( 04/2017 )
// 9.1 Parsing process for Exp-Golomb codes (Page 208)
// 
// 

use crate::bit_vec::BitVec;
use crate::bitstream_io::{ BitReader, BitWriter, Endianness, BigEndian, Numeric };

use std::io::{ Read, Write };

// H264 字节流中的 指数哥伦布编码的 `K` 阶 为 `0` .
pub const K: u32 = 0;


// NOTE: 最大支持对 32位 的无符号整数进行 指数哥伦布编码
pub fn ue_encode<W: Write, E: Endianness>(num: u32, out: &mut BitWriter<W, E>) -> u32 {
    // CodeNum + 2^K
    let t1 = num + 2u32.pow(K);
    let t1_bits_len = t1.count_ones() + t1.count_zeros() - t1.leading_zeros();
    // M = T1_BITS_LEN - 1 - K
    let m = t1_bits_len - 1 - K;
    // MZeros
    out.write(m, 0b0).unwrap();
    // INFO
    out.write(t1_bits_len, t1).unwrap();

    let v = t1_bits_len + m;

    v
}

// ue(v)
// NOTE: 最大支持32位的哥伦布编码
pub fn ue_decode<R: Read, E: Endianness>(input: &mut BitReader<R, E>, out: &mut u32) -> u32 {
    // CodeLen: 2M + k + 1
    let mut m = 0u32;

    let mut bits = BitVec::new();

    #[allow(unused_assignments)]
    let mut v = 0u32;

    loop {
        let bit = input.read_bit().unwrap();
        if bit == true {
            // T1_BITS_LEN = M + 1 + K
            let t1_bits_len = m + 1 + K;
            v = t1_bits_len + m;

            // FIXME: 最高支持 32 位
            assert_eq!(t1_bits_len <= 32, true);

            for _ in 0..m {
                bits.push(false);
            }

            // NOTE: 高位补零
            for _ in 0..(32-t1_bits_len-m) {
                bits.push(false);
            }

            bits.push(bit);

            for _ in 1..t1_bits_len {
                bits.push(input.read_bit().unwrap());
            }
            break;
        } else {
            m += 1;
        }
    }
    
    let bytes = bits.to_bytes();
    assert_eq!(bits.len(), 32);
    assert_eq!(bytes.len(), 4);
    
    *out = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) - 2u32.pow(K);

    v
}

// se(v)
pub fn se_decode(num: u32) -> i32 {
    // 公式: (−1)^(k+1) * Ceil( k÷2 )
    (-1i32).pow(num+1) * (num as f64 / 2.0).ceil() as i32
}

// me(v)
pub fn me_decode(num: u32, chroma_array_type: u8) -> (u8, u8) {
    match chroma_array_type {
        1 | 2 => {
            match TABLE_A.binary_search_by_key(&(num as u8), |&(a, _b, _c)| a) {
                Ok(pos) => {
                    let item = TABLE_A[pos];
                    (item.1, item.2)
                },
                Err(_) => panic!("oops ...")
            }
        },
        0 | 3 => {
            match TABLE_B.binary_search_by_key(&(num as u8), |&(a, _b, _c)| a) {
                Ok(pos) => {
                    let item = TABLE_B[pos];
                    (item.1, item.2)
                },
                Err(_) => panic!("oops ...")
            }
        },
        _ => panic!("oops ...")
    }
}


pub const UE_GOLOMB_LEN: [u8; 256] = [
     1, 3, 3, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 7, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,11,
    11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,13,
    13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,
    13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,15,
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,17,
];

// Page 211
// Table 9-4 – Assignment of codeNum to values of coded_block_pattern for macroblock prediction modes
// (a) ChromaArrayType is equal to 1 or 2
// (codeNum, Intra_4x4_OR_Intra_8x8, Inter)
pub const TABLE_A: [(u8, u8, u8); 48] = [
    (0, 47, 0),
    (1, 31, 16),
    (2, 15, 1),
    (3, 0, 2),
    (4, 23, 4),
    (5, 27, 8),
    (6, 29, 32),
    (7, 30, 3),
    (8, 7, 5),
    (9, 11, 10),
    (10, 13, 12),
    (11, 14, 15),
    (12, 39, 47),
    (13, 43, 7),
    (14, 45, 11),
    (15, 46, 13),
    (16, 16, 14),
    (17, 3, 6),
    (18, 5, 9),
    (19, 10, 31),
    (20, 12, 35),
    (21, 19, 37),
    (22, 21, 42),
    (23, 26, 44),
    (24, 28, 33),
    (25, 35, 34),
    (26, 37, 36),
    (27, 42, 40),
    (28, 44, 39),
    (29, 1, 43),
    (30, 2, 45),
    (31, 4, 46),
    (32, 8, 17),
    (33, 17, 18),
    (34, 18, 20),
    (35, 20, 24),
    (36, 24, 19),
    (37, 6, 21),
    (38, 9, 26),
    (39, 22, 28),
    (40, 25, 23),
    (41, 32, 27),
    (42, 33, 29),
    (43, 34, 30),
    (44, 36, 22),
    (45, 40, 25),
    (46, 38, 38),
    (47, 41, 41),
];

// // (a) ChromaArrayType is equal to 0 or 3
pub const TABLE_B: [(u8, u8, u8); 16] = [
    (0, 15, 0),
    (1, 0, 1),
    (2, 7, 2),
    (3, 11, 4),
    (4, 13, 8),
    (5, 14, 3),
    (6, 3, 5),
    (7, 5, 10),
    (8, 10, 12),
    (9, 12, 15),
    (10, 1, 7),
    (11, 2, 11),
    (12, 4, 13),
    (13, 8, 14),
    (14, 6, 6),
    (15, 9, 9),
];

#[cfg(test)]
mod test {

    use crate::bit_vec::BitVec;
    use crate::bitstream_io::{ BitReader, BitWriter, Endianness, BigEndian };

    use super::{ue_encode, ue_decode, se_decode, me_decode};

    use std::io::{Read, Cursor};

    #[test]
    fn test_encode() {
        let f = |num: u32, ensure_bit_str: &str| {
            let mut out = BitWriter::endian(Vec::new(), BigEndian);
            let v = ue_encode(num, &mut out);

            let mut aligned_bits = 0u32;

            while !out.byte_aligned() {
                // out.byte_align().unwrap();
                out.write_bit(false).unwrap();
                aligned_bits += 1;
            }

            let acc = out.into_writer().iter().map(|n| format!("{:08b}", n)).collect::<String>();

            let bits_len = acc.len() - aligned_bits as usize;
            let m = v - bits_len as u32;
            
            let res = "0".repeat(m as usize) + &acc[..bits_len];

            assert_eq!(ensure_bit_str, res);
            assert_eq!(v as usize, ensure_bit_str.len());
        };
        
        f(0, "1");
        f(1, "010");
        f(2, "011");
        f(3, "00100");
        f(4, "00101");
        f(5, "00110");
        f(6, "00111");
        f(7, "0001000");
        f(8, "0001001");
        f(9, "0001010");

        f(10, "0001011");
        f(11, "0001100");
        f(12, "0001101");
        f(13, "0001110");
        f(14, "0001111");
        f(15, "000010000");
        f(16, "000010001");
        f(17, "000010010");
        f(18, "000010011");
        f(19, "000010100");

        f(20, "000010101");
        f(21, "000010110");
        f(22, "000010111");
        f(23, "000011000");
        f(24, "000011001");
        f(25, "000011010");
        f(26, "000011011");
        f(27, "000011100");
        f(28, "000011101");
        f(29, "000011110");
    }

    #[test]
    fn test_decode() {
        let f = |num: u32, ensure_bit_str: &str, bytes: &[u8]| {
            let mut reader = BitReader::endian(Cursor::new(bytes), BigEndian);
            let mut x = 0u32;
            let v = ue_decode(&mut reader, &mut x);

            assert_eq!(x, num);
            assert_eq!(ensure_bit_str.len(), v as usize);
        };

        f(0, "1", &[0b1000_0000]);
        f(1, "010", &[0b0100_0000]);
        f(2, "011", &[0b0110_0000]);
        f(3, "00100", &[0b0010_0000]);
        f(4, "00101", &[0b0010_1000]);
        f(5, "00110", &[0b0011_0000]);
        f(6, "00111", &[0b0011_1000]);
        f(7, "0001000", &[0b0001_0000]);
        f(8, "0001001", &[0b0001_0010]);
        f(9, "0001010", &[0b0001_0100]);

        f(10, "0001011", &[0b0001_0110]);
        f(11, "0001100", &[0b0001_1000]);
        f(12, "0001101", &[0b0001_1010]);
        f(13, "0001110", &[0b0001_1100]);
        f(14, "0001111", &[0b0001_1110]);
        f(15, "000010000", &[0b0000_1000, 0b0000_0000]);
        f(16, "000010001", &[0b0000_1000, 0b1000_0000]);
        f(17, "000010010", &[0b0000_1001, 0b0000_0000]);
        f(18, "000010011", &[0b0000_1001, 0b1000_0000]);
        f(19, "000010100", &[0b0000_1010, 0b0000_0000]);

        f(20, "000010101", &[0b0000_1010, 0b1000_0000]);
        f(21, "000010110", &[0b0000_1011, 0b0000_0000]);
        f(22, "000010111", &[0b0000_1011, 0b1000_0000]);
        f(23, "000011000", &[0b0000_1100, 0b0000_0000]);
        f(24, "000011001", &[0b0000_1100, 0b1000_0000]);
        f(25, "000011010", &[0b0000_1101, 0b0000_0000]);
        f(26, "000011011", &[0b0000_1101, 0b1000_0000]);
        f(27, "000011100", &[0b0000_1110, 0b0000_0000]);
        f(28, "000011101", &[0b0000_1110, 0b1000_0000]);
        f(29, "000011110", &[0b0000_1111, 0b0000_0000]);
    }

    #[test]
    fn test_se_decode() {
        assert_eq!(se_decode(0), 0);
        assert_eq!(se_decode(1), 1);
        assert_eq!(se_decode(2), -1);
        assert_eq!(se_decode(3), 2);
        assert_eq!(se_decode(4), -2);
        assert_eq!(se_decode(5), 3);
        assert_eq!(se_decode(6), -3);
    }

    #[test]
    fn test_me_decode() {
        assert_eq!(me_decode(0, 1), (47, 0));
        assert_eq!(me_decode(0, 2), (47, 0));

        assert_eq!(me_decode(47, 1), (41, 41));
        assert_eq!(me_decode(47, 2), (41, 41));


        assert_eq!(me_decode(0, 0), (15, 0));
        assert_eq!(me_decode(0, 3), (15, 0));

        assert_eq!(me_decode(15, 0), (9, 9));
        assert_eq!(me_decode(15, 3), (9, 9));
    }

}

