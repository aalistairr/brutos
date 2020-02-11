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

    pub fn read_byte(&mut self) -> Result<u8, ReadError> {
        match self.read_bytes(1)? {
            [byte] => Ok(*byte),
            _ => unreachable!(),
        }
    }
}

pub fn fill_array(bytes: ByteStream, array: &mut [u8]) -> Result<(), ReadError> {
    for i in 0..array.len() {
        array[i] = bytes.read_byte()?;
    }
    Ok(())
}

#[macro_export]
macro_rules! read_array {
    ($bytes:expr, $n:expr) => {{
        const N: usize = $n;
        let bytes: &mut $crate::byte_stream::ByteBuffer = $bytes;
        let mut array: [u8; N] = [0; N];
        $crate::byte_stream::fill_array(bytes, &mut array).map(|()| array)
    }};
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Endianness {
    Little,
    Big,
}

macro_rules! read_uint {
    ($name:ident, $t:ty) => {
        pub fn $name<E>(&mut self, endianness: E) -> Result<$t, ReadError>
        where
            E: Into<Endianness>
        {
            let f = match endianness.into() {
                Endianness::Little => <$t>::from_le_bytes,
                Endianness::Big => <$t>::from_be_bytes,
            };
            Ok(f(read_array!(self, core::mem::size_of::<$t>())?))
        }
    }
}

impl<'a> ByteBuffer<'a> {
    read_uint!(read_u16, u16);
    read_uint!(read_u32, u32);
    read_uint!(read_u64, u64);
    read_uint!(read_u128, u128);
}

impl<'a> ByteBuffer<'a> {
    pub fn read_ascii_octal(&mut self, len: usize) -> Result<usize, ReadError> {
        let bytes = self.read_bytes(len)?;
        let s = str::from_utf8(bytes)?;
        Ok(usize::from_str_radix(s, 8)?)
    }
}
