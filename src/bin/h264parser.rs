#[macro_use]
extern crate log;
extern crate env_logger;
extern crate bitstream_io;
extern crate h264parser;

use std::env;
use std::fs;


fn main () {
    env_logger::init();
    
    let filename = env::args().nth(1).expect("$ ./h264parse input.h264");
    let file = fs::File::open(filename).unwrap();

    let stream = h264parser::stream::StreamReader::new(file, h264parser::stream::StreamFormat::AnnexB);
    
    for nalu in stream {
        match nalu {
            Ok(nalu) => println!("{:?}", nalu),
            Err(e) => {
                println!("{:?}", e);
                break;
            },
        }
    }
}