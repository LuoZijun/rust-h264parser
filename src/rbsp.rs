
use std::fmt;

// RBSP
pub trait RawByteSequencePayload: 'static + Send + fmt::Debug + Clone + Copy + Sized {

}

// SPS
#[derive(Debug, Clone, Copy)]
pub struct SequenceParameterSet {
    pub n: u8
}

// PPS
#[derive(Debug, Clone, Copy)]
pub struct PictureParameterSet {
    b: u8
}

// SEI
#[derive(Debug, Clone, Copy)]
pub struct SupplementalEnhancementInformation {

}

// IDR Picture
#[derive(Debug, Clone, Copy)]
pub struct CodedSliceIdr {

}

// non-IDR Picture
#[derive(Debug, Clone, Copy)]
pub struct CodedSliceNonIdr {

}



impl RawByteSequencePayload for SequenceParameterSet {

}

impl RawByteSequencePayload for PictureParameterSet {

}
