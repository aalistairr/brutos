#![no_std]

use core::str::{self, Utf8Error};

use brutos_util::byte_stream::{ByteBuffer, ByteStream, ReadError};
use brutos_util::iter::unfold;

pub enum Error {
    Read(ReadError),
    Invalid,
}

impl From<ReadError> for Error {
    fn from(e: ReadError) -> Error {
        Error::Read(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error::Read(ReadError::Utf8(e))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Header {
    pub magic: usize,
    pub dev: usize,
    pub ino: usize,
    pub mode: usize,
    pub uid: usize,
    pub gid: usize,
    pub nlink: usize,
    pub rdev: usize,
    pub mtime: usize,
    pub namesize: usize,
    pub filesize: usize,
}

impl Header {
    pub fn read(bytes: ByteStream) -> Result<Header, ReadError> {
        Ok(Header {
            magic: bytes.read_ascii_octal(6)?,
            dev: bytes.read_ascii_octal(6)?,
            ino: bytes.read_ascii_octal(6)?,
            mode: bytes.read_ascii_octal(6)?,
            uid: bytes.read_ascii_octal(6)?,
            gid: bytes.read_ascii_octal(6)?,
            nlink: bytes.read_ascii_octal(6)?,
            rdev: bytes.read_ascii_octal(6)?,
            mtime: bytes.read_ascii_octal(11)?,
            namesize: bytes.read_ascii_octal(6)?,
            filesize: bytes.read_ascii_octal(11)?,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Entry<'a> {
    pub header: Header,
    pub filename: Option<&'a str>,
    pub contents: &'a [u8],
}

impl<'a> Entry<'a> {
    fn read(bytes: &mut ByteBuffer<'a>) -> Result<Entry<'a>, Error> {
        let header = Header::read(bytes)?;
        Ok(Entry {
            header,
            filename: match bytes.read_bytes(header.namesize)? {
                [] | [0] => None,
                [bytes @ .., 0] => Some(str::from_utf8(bytes)?),
                _ => return Err(Error::Invalid),
            },
            contents: bytes.read_bytes(header.filesize)?,
        })
    }
}

pub fn entries(cpio: &[u8]) -> impl Iterator<Item = Result<Entry, Error>> {
    unfold(Some(ByteBuffer(cpio)), |stream| match stream.take() {
        None => None,
        Some(mut s) => match Entry::read(&mut s) {
            Err(e) => Some(Err(e)),
            Ok(e) if e.filename == Some("TRAILER!!") => None,
            Ok(e) => {
                *stream = Some(s);
                Some(Ok(e))
            }
        },
    })
}
