


// SODB     +      RBSP trailing bits    =  RBSP
// NAL Header(1 byte)      +      RBSP   = NALU
// Start Code Prefix(3 bytes)  +   NALU  +  Start Code Prefix(3 bytes)  +   NALU   + ... +  = H.264BitsStream
// 

mod sps;
mod pps;
mod sei;

pub use self::sps::{ SequenceParameterSet, SequenceParameterSetFlag };
pub use self::pps::{ PictureParameterSet, };


use std::fmt;

// SODB: String Of Data Bits ( 原始数据比特流, 长度不一定是8的倍数，故需要补齐 )

// 原始数据字节流
// RBSP: A NALU contains a Raw Byte Sequence Payload, a sequence of bytes containingsyntax elements.
pub trait RawByteSequencePayload: 'static + Send + fmt::Debug + Clone + Copy  {

}


impl RawByteSequencePayload for SequenceParameterSet {

}

impl RawByteSequencePayload for PictureParameterSet {

}