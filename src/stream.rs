
use crate::error;
use crate::nalu::Nalu;
use crate::rbsp::RawByteSequencePayload;


use std::io::{ self, Read, };
use std::marker::PhantomData;
use std::convert::TryFrom;


// Annex B: Byte stream format ( Page: 328 )
// Syntax: B.1.1 ( Page 328 )
// Semantic: B.1.2 ( Page 328 )


#[derive(Debug, PartialEq, Eq)]
pub enum StreamFormat {
    AnnexB,
    AvcC,
}


#[derive(Debug)]
pub struct StreamReader<R: Read> {
    stream: R,
    format: StreamFormat,
    buffer: Vec<u8>,
    last_byte: Option<u8>,

}

impl<R: Read> StreamReader<R> {
    pub fn new(stream: R, format: StreamFormat) -> Self {
        Self {
            stream: stream,
            format: format,
            buffer: vec![],
            last_byte: None,
        }
    }
    
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn next_byte(&mut self) -> Result<u8, error::Error> {
        let mut buf = [0u8; 1];
        self.stream.read_exact(&mut buf)?;

        Ok(buf[0])
    }

    pub fn next_annex_b_nalu(&mut self) -> Result<Nalu, error::Error> {
        let malformed_input_data = io::Error::new(io::ErrorKind::InvalidData, "malformed input data").into();
        
        debug!("parse nal unit prefix zeros ...");

        loop {
            let byte = match self.last_byte {
                Some(n) => {
                    self.last_byte = None;
                    n
                },
                None => self.next_byte()?,
            };
            
            match byte {
                0x00 => continue,
                0x01 => break,
                _ => {
                    error!("start prefix byte ({}) must be 0x00 or 0x01", byte);
                    return Err(malformed_input_data)
                },
            }
        }

        debug!("parse nal unit body ...");
        loop {
            let byte = self.next_byte()?;
            self.buffer.push(byte);

            let buffer_len = self.buffer.len();

            if buffer_len >= 3 {
                let last_three_bytes = &self.buffer[buffer_len-3..];

                if last_three_bytes == [0x00, 0x00, 0x00] {
                    let byte = self.next_byte()?;
                    match byte {
                        0x03 => {
                            self.buffer.push(0x03);
                            continue
                        },
                        n @ _ => {
                            self.last_byte = Some(n);
                            self.buffer.remove(self.buffer.len()-1);
                            self.buffer.remove(self.buffer.len()-1);
                            self.buffer.remove(self.buffer.len()-1);
                            break;
                        },
                    }
                }
            }
        }

        Nalu::try_from(&self.buffer[..])
    }

    pub fn next_avcc_nalu(&mut self) -> Result<Nalu, error::Error> {
        unimplemented!()
    }
}

impl<R: Read> Iterator for StreamReader<R> {
    type Item = Result<Nalu, error::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.format {
            StreamFormat::AnnexB => self.next_annex_b_nalu(),
            StreamFormat::AvcC => self.next_avcc_nalu(),
        };
        
        self.buffer.clear();

        match res {
            Err(error::Error::IoError(e)) => {
                match e.kind() {
                    io::ErrorKind::UnexpectedEof => None,
                    _ => Some(Err(e.into())),
                }
            },
            Ok(nalu) => Some(Ok(nalu)),
        }
    }
}
