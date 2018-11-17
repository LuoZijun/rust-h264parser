
use std::io;


#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::IoError(io_error)
    }
}

