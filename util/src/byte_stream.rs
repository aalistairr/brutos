use core::num::ParseIntError;
use core::str::{self, Utf8Error};

#[derive(Clone, Debug)]
pub enum ReadError {
    UnexpectedEof,
    Utf8(Utf8Error),
    ParseInt(ParseIntError),
}

impl From<Utf8Error> for ReadError {
    fn from(e: Utf8Error) -> ReadError {
        ReadError::Utf8(e)
    }
}

impl From<ParseIntError> for ReadError {
    fn from(e: ParseIntError) -> ReadError {
        ReadError::ParseInt(e)
    }
}

pub struct ByteBuffer<'a>(pub &'a [u8]);

pub type ByteStream<'a, 's> = &'s mut ByteBuffer<'a>;

impl<'a> ByteBuffer<'a> {
    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], ReadError> {
        if len > self.0.len() {
            return Err(ReadError::UnexpectedEof);
        }
        let (x, xs) = self.0.split_at(len);
        self.0 = xs;
        Ok(x)
    }

    pub fn read_ascii_octal(&mut self, len: usize) -> Result<usize, ReadError> {
        let bytes = self.read_bytes(len)?;
        let s = str::from_utf8(bytes)?;
        Ok(usize::from_str_radix(s, 8)?)
    }
}
