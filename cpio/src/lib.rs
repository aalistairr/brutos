#![no_std]

use core::num::ParseIntError;
use core::str::{self, Utf8Error};

use brutos_util::iter::unfold;

#[derive(Clone, Debug)]
pub enum Error {
    UnexpectedEof,
    Utf8(Utf8Error),
    ParseInt(ParseIntError),
    Invalid,
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error::Utf8(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        Error::ParseInt(e)
    }
}

struct ByteStream<'a>(&'a [u8]);

impl<'a> ByteStream<'a> {
    fn read(&mut self, len: usize) -> Result<&'a [u8], Error> {
        if len > self.0.len() {
            return Err(Error::UnexpectedEof);
        }
        let (x, xs) = self.0.split_at(len);
        self.0 = xs;
        Ok(x)
    }

    fn read_octal(&mut self, len: usize) -> Result<usize, Error> {
        let bytes = self.read(len)?;
        let s = str::from_utf8(bytes)?;
        Ok(usize::from_str_radix(s, 8)?)
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

impl<'a> ByteStream<'a> {
    pub fn read_header(&mut self) -> Result<Header, Error> {
        Ok(Header {
            magic: self.read_octal(6)?,
            dev: self.read_octal(6)?,
            ino: self.read_octal(6)?,
            mode: self.read_octal(6)?,
            uid: self.read_octal(6)?,
            gid: self.read_octal(6)?,
            nlink: self.read_octal(6)?,
            rdev: self.read_octal(6)?,
            mtime: self.read_octal(11)?,
            namesize: self.read_octal(6)?,
            filesize: self.read_octal(11)?,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Entry<'a> {
    pub header: Header,
    pub filename: Option<&'a str>,
    pub contents: &'a [u8],
}

impl<'a> ByteStream<'a> {
    fn read_entry(&mut self) -> Result<Entry<'a>, Error> {
        let header = self.read_header()?;
        Ok(Entry {
            header,
            filename: match self.read(header.namesize)? {
                [] | [0] => None,
                [bytes @ .., 0] => Some(str::from_utf8(bytes)?),
                _ => return Err(Error::Invalid),
            },
            contents: self.read(header.filesize)?,
        })
    }
}

pub fn entries(cpio: &[u8]) -> impl Iterator<Item = Result<Entry, Error>> {
    unfold(Some(ByteStream(cpio)), |stream| match stream.take() {
        None => None,
        Some(mut s) => match s.read_entry() {
            Err(e) => Some(Err(e)),
            Ok(e) if e.filename == Some("TRAILER!!") => None,
            Ok(e) => {
                *stream = Some(s);
                Some(Ok(e))
            }
        },
    })
}
