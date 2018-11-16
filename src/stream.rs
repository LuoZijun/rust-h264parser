
use crate::nalu::Nalu;
use crate::rbsp::RawByteSequencePayload;

use std::io::Read;
use std::io::Error;
use std::io::ErrorKind;
use std::marker::PhantomData;


// Annex B: Byte stream format ( Page: 328 )
// Syntax: B.1.1 ( Page 328 )
// Semantic: B.1.2 ( Page 328 )
#[derive(Debug)]
pub struct AnnexBReader<R: Read> {
    reader: R,
}

impl<R: Read> AnnexBReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: reader,
        }
    }
}

impl<R: Read> Iterator for AnnexBReader<R> {
    type Item = Result<Nalu, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct AVCReader<R: Read> {
    reader: R,
}

impl<R: Read> AVCReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: reader,
        }
    }
}

impl<R: Read> Iterator for AVCReader<R> {
    type Item = Result<Nalu, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}