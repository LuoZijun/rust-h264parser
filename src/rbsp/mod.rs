
mod sps;
mod pps;
mod sei;

pub use self::sps::{ SequenceParameterSet, SequenceParameterSetFlag };
pub use self::pps::{ PictureParameterSet, };


use std::fmt;

// RBSP
pub trait RawByteSequencePayload: 'static + Send + fmt::Debug + Clone + Copy + Sized {

}


impl RawByteSequencePayload for SequenceParameterSet {

}

impl RawByteSequencePayload for PictureParameterSet {

}