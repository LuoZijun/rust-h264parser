
use crate::nalu::Nalu;
use crate::rbsp::RawByteSequencePayload;

use std::io::Read;
use std::io::Error;
use std::io::ErrorKind;
use std::marker::PhantomData;


pub enum State {
    Slice,
}


#[derive(Debug)]
pub struct AnnexBReader<R: Read, RBSP: RawByteSequencePayload> {
    reader: R,
    phantom: PhantomData<RBSP>,
}

impl<R: Read, RBSP: RawByteSequencePayload> AnnexBReader<R, RBSP> {
    pub fn new(reader: R) -> Self {
        Self { reader, phantom: PhantomData }
    }
}

impl<R: Read, RBSP: RawByteSequencePayload> Iterator for AnnexBReader<R, RBSP> {
    type Item = Result<Nalu<RBSP>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct AVCReader<R: Read, RBSP: RawByteSequencePayload> {
    reader: R,
    phantom: PhantomData<RBSP>,
}

impl<R: Read, RBSP: RawByteSequencePayload> AVCReader<R, RBSP> {
    pub fn new(reader: R) -> Self {
        Self { reader, phantom: PhantomData }
    }
}

impl<R: Read, RBSP: RawByteSequencePayload> Iterator for AVCReader<R, RBSP> {
    type Item = Result<Nalu<RBSP>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}