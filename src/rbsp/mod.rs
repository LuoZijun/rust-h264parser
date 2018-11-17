
// SODB     +      RBSP trailing bits    =  RBSP
// NAL Header(1 byte)      +      RBSP   = NALU
// Start Code Prefix(3 bytes)  +   NALU  +  Start Code Prefix(3 bytes)  +   NALU   + ... +  = H.264BitsStream
// 

use crate::error::Error;


mod sps;
mod pps;
mod sei;

pub use self::sps::{ SequenceParameterSet, SequenceParameterSetFlag };
pub use self::pps::{ PictureParameterSet, };



use std::fmt;
use std::any::Any;
use std::io::{ self, Read, };
use std::convert::TryFrom;


// SODB: String Of Data Bits ( 原始数据比特流, 长度不一定是8的倍数，故需要补齐 )

// 原始数据字节流
// RBSP: A NALU contains a Raw Byte Sequence Payload, a sequence of bytes containingsyntax elements.
pub trait RawByteSequencePayload: fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}


impl RawByteSequencePayload for SequenceParameterSet {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RawByteSequencePayload for PictureParameterSet {
    fn as_any(&self) -> &dyn Any {
        self
    }
}



pub struct DebugRbSp {
    bytes: Vec<u8>,
}

impl RawByteSequencePayload for DebugRbSp {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Debug for DebugRbSp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Length({:?})", &self.bytes.len())
    }
}

impl TryFrom<&[u8]> for DebugRbSp {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            bytes: value.to_vec(),
        })
    }
}