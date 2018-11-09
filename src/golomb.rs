
// 指数哥伦布编码
// https://en.wikipedia.org/wiki/Exponential-Golomb_coding

// ue(v)：无符号整数指数哥伦布码编码的语法元素，左位在先。
// se(v)：有符号整数指数哥伦布码编码的语法元素，左位在先。


use crate::bit_vec::BitVec;
use crate::bitstream_io::{ BitReader, BitWriter, Endianness, BigEndian };

use std::io::{ Read, Write };

// H264 字节流中的 指数哥伦布编码的 `K` 阶 为 `0` .
const K: usize = 0;

pub fn encode<W: Write, E: Endianness>(num: u8, out: &mut BitWriter<W, E>) -> usize {
    let t1 = num + 1; // num >> K
    let t1_bits_len = t1.count_ones() + t1.count_zeros() - t1.leading_zeros();
    let m = t1_bits_len - 1;

    out.write(m, 0b0).unwrap();
    out.write(t1_bits_len, t1).unwrap();

    let v = (t1_bits_len + m) as usize;

    v
}

pub fn decode<R: Read, E: Endianness>(input: &mut BitReader<R, E>, out: &mut u8) -> usize {
    // CodeLen: 2M + k + 1
    let mut m = 0usize;

    let mut bits = BitVec::new();

    #[allow(unused_assignments)]
    let mut v = 0usize;

    loop {
        let bit = input.read_bit().unwrap();
        if bit == true {
            v = 2 * m + K + 1;
            let code_len = v - m;
            for _ in 0..(8-code_len) {
                bits.push(false);
            }
            bits.push(bit);
            for _ in 1..code_len {
                bits.push(input.read_bit().unwrap());
            }
            break;
        } else {
            m += 1;
        }
    }
    
    assert_eq!(bits.len() <= 8, true);
    let bytes = bits.to_bytes();

    assert_eq!(bytes.len(), 1);

    *out = bytes[0] - 1;

    v
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


#[cfg(test)]
mod test {

    use crate::bit_vec::BitVec;
    use crate::bitstream_io::{ BitReader, BitWriter, Endianness, BigEndian };

    use super::{encode, decode};

    use std::io::{Read, Cursor};

    #[test]
    fn test_encode() {
        let f = |num: u8, ensure_bit_str: &str| {
            let mut out = BitWriter::endian(Vec::new(), BigEndian);
            let v = encode(num, &mut out);

            let mut aligned_bits = 0usize;

            while !out.byte_aligned() {
                // out.byte_align().unwrap();
                out.write_bit(false).unwrap();
                aligned_bits += 1;
            }

            let acc = out.into_writer().iter().map(|n| format!("{:08b}", n)).collect::<String>();

            let bits_len = acc.len() - aligned_bits;
            let m = v - bits_len;
            
            let res = "0".repeat(m) + &acc[..bits_len];

            assert_eq!(ensure_bit_str, res);
            assert_eq!(v, ensure_bit_str.len());
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
        let f = |num: u8, ensure_bit_str: &str, bytes: &[u8]| {
            let mut reader = BitReader::endian(Cursor::new(bytes), BigEndian);
            let mut x = 0u8;
            let v = decode(&mut reader, &mut x);

            assert_eq!(x, num);
            assert_eq!(ensure_bit_str.len(), v);
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
}

