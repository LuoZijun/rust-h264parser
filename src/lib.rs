#![feature(try_from, conservative_impl_trait)]

extern crate byteorder;

use byteorder::{NetworkEndian, ReadBytesExt};


pub mod nalu;
pub mod rbsp;
pub mod stream;
