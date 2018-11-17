#![feature(int_to_from_bytes, try_from, associated_type_defaults, impl_trait_in_bindings)]
#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate log;
extern crate byteorder;
extern crate bit_vec;
extern crate bitstream_io;


use byteorder::{ NetworkEndian, ReadBytesExt };
pub use crate::bit_vec::BitVec;


pub mod nalu;
pub mod rbsp;
pub mod stream;
pub mod golomb;
pub mod error;
