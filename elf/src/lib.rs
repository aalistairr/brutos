#![no_std]

use core::convert::TryInto;
use core::fmt;
use core::str::{self, Utf8Error};
use core::usize;

use bitbash::{bitfield_nonconst as bitfield, BitEnumNonConst as BitEnum, ConvertRepr};

use brutos_util::byte_stream::{
    ByteBuffer, ByteStream, Endianness as ByteStreamEndianness, ReadError,
};
use brutos_util::read_array;

#[derive(Clone, Debug)]
pub enum Error {
    Read(ReadError),
    Invalid,
    Overflow,
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

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Class {
    Class32 = 1,
    Class64 = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ClassUsize {
    Usize32(u32),
    Usize64(u64),
}

impl fmt::Debug for ClassUsize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClassUsize::Usize32(n) => f
                .debug_tuple("Usize32")
                .field(&(*n as usize as *const ()))
                .finish(),
            ClassUsize::Usize64(n) => f
                .debug_tuple("Usize64")
                .field(&(*n as usize as *const ()))
                .finish(),
        }
    }
}

impl ClassUsize {
    fn read(bytes: ByteStream, class: Class, endianness: Endianness) -> Result<ClassUsize, Error> {
        match class {
            Class::Class32 => Ok(ClassUsize::Usize32(bytes.read_u32(endianness)?)),
            Class::Class64 => Ok(ClassUsize::Usize64(bytes.read_u64(endianness)?)),
        }
    }

    pub fn try_into_usize(self) -> Result<usize, Error> {
        match self {
            ClassUsize::Usize32(x) if x <= usize::MAX as u32 => Ok(x as usize),
            ClassUsize::Usize64(x) if x <= usize::MAX as u64 => Ok(x as usize),
            _ => Err(Error::Overflow),
        }
    }

    pub fn u32(&self) -> Option<u32> {
        match self {
            ClassUsize::Usize32(n) => Some(*n),
            _ => None,
        }
    }

    pub fn u64(&self) -> Option<u64> {
        match self {
            ClassUsize::Usize64(n) => Some(*n),
            _ => None,
        }
    }
}

const FILE_HEADER_MAGIC: [u8; 4] = [0x7f, 'E' as u8, 'L' as u8, 'F' as u8];

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FileHeader {
    pub magic: [u8; 4],
    pub class: Class,
    pub endianness: Endianness,
    pub version_0: u8,
    pub abi: Abi,
    pub abi_version: u8,
    pub _padding: [u8; 7],
    pub ty: FileType,
    pub machine: Machine,
    pub version_1: u32,
    pub entry: ClassUsize,
    pub phoff: ClassUsize,
    pub shoff: ClassUsize,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Endianness {
    Little = 1,
    Big = 2,
}

impl Into<ByteStreamEndianness> for Endianness {
    fn into(self) -> ByteStreamEndianness {
        match self {
            Endianness::Little => ByteStreamEndianness::Little,
            Endianness::Big => ByteStreamEndianness::Big,
        }
    }
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Abi {
    SysV = 0,
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum FileType {
    None = 0x0,
    Exec = 0x2,
    Dyn = 0x3,
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum Machine {
    Amd64 = 0x3e,
}

impl FileHeader {
    fn read(bytes: ByteStream) -> Result<FileHeader, Error> {
        let magic = match read_array!(bytes, 4)? {
            x @ FILE_HEADER_MAGIC => x,
            _ => return Err(Error::Invalid),
        };
        let class = Class::try_from_repr(bytes.read_byte()?).ok_or(Error::Invalid)?;
        let endianness = Endianness::try_from_repr(bytes.read_byte()?).ok_or(Error::Invalid)?;
        Ok(FileHeader {
            magic,
            class,
            endianness,
            version_0: match bytes.read_byte()? {
                x @ 1 => x,
                _ => return Err(Error::Invalid),
            },
            abi: Abi::try_from_repr(bytes.read_byte()?).ok_or(Error::Invalid)?,
            abi_version: bytes.read_byte()?,
            _padding: read_array!(bytes, 7)?,
            ty: FileType::try_from_repr(bytes.read_u16(endianness)?).ok_or(Error::Invalid)?,
            machine: Machine::try_from_repr(bytes.read_u16(endianness)?).ok_or(Error::Invalid)?,
            version_1: match bytes.read_u32(endianness)? {
                x @ 1 => x,
                _ => return Err(Error::Invalid),
            },
            entry: ClassUsize::read(bytes, class, endianness)?,
            phoff: ClassUsize::read(bytes, class, endianness)?,
            shoff: ClassUsize::read(bytes, class, endianness)?,
            flags: bytes.read_u32(endianness)?,
            ehsize: bytes.read_u16(endianness)?,
            phentsize: bytes.read_u16(endianness)?,
            phnum: bytes.read_u16(endianness)?,
            shentsize: bytes.read_u16(endianness)?,
            shnum: bytes.read_u16(endianness)?,
            shstrndx: bytes.read_u16(endianness)?,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ProgramHeader {
    pub ty: SegmentType,
    pub flags: SegmentFlags,
    pub offset: ClassUsize,
    pub vaddr: ClassUsize,
    pub paddr: ClassUsize,
    pub filesz: ClassUsize,
    pub memsz: ClassUsize,
    pub align: ClassUsize,
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SegmentTypeStandard {
    Null = 0x0,
    Load = 0x1,
    Dynamic = 0x2,
    Interp = 0x3,
    Note = 0x4,
    Shlib = 0x5,
    Phdr = 0x6,
    Tls = 0x7,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SegmentType {
    Standard(SegmentTypeStandard),
    Custom(u32),
}

impl PartialEq<SegmentTypeStandard> for SegmentType {
    fn eq(&self, other: &SegmentTypeStandard) -> bool {
        match self {
            SegmentType::Standard(ty) => ty == other,
            SegmentType::Custom(_) => false,
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct SegmentFlags(u32);

    derive_debug;

    pub field execute: bool = [0];
    pub field write: bool = [1];
    pub field read: bool = [2];
}

impl ProgramHeader {
    fn read(
        bytes: ByteStream,
        class: Class,
        endianness: Endianness,
    ) -> Result<ProgramHeader, Error> {
        let ty = bytes.read_u32(endianness)?;
        let ty = match SegmentTypeStandard::try_from_repr(ty) {
            Some(x) => SegmentType::Standard(x),
            None if ty >= 0x60000000 && ty <= 0x6FFFFFFF => SegmentType::Custom(ty),
            None if ty >= 0x70000000 && ty <= 0x7FFFFFFF => SegmentType::Custom(ty),
            _ => return Err(Error::Invalid),
        };

        let mut flags_64 = 0;
        if let Class::Class64 = class {
            flags_64 = bytes.read_u32(endianness)?;
        }

        let offset = ClassUsize::read(bytes, class, endianness)?;
        let vaddr = ClassUsize::read(bytes, class, endianness)?;
        let paddr = ClassUsize::read(bytes, class, endianness)?;
        let filesz = ClassUsize::read(bytes, class, endianness)?;
        let memsz = ClassUsize::read(bytes, class, endianness)?;

        let mut flags_32 = 0;
        if let Class::Class32 = class {
            flags_32 = bytes.read_u32(endianness)?;
        }

        let align = ClassUsize::read(bytes, class, endianness)?;

        Ok(ProgramHeader {
            ty,
            offset,
            vaddr,
            paddr,
            filesz,
            memsz,
            align,
            flags: SegmentFlags(match class {
                Class::Class32 => flags_32,
                Class::Class64 => flags_64,
            }),
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SectionHeader {
    pub name: u32,
    pub ty: SectionType,
    pub flags: SectionFlags,
    pub addr: ClassUsize,
    pub offset: ClassUsize,
    pub size: ClassUsize,
    pub link: u32,
    pub info: u32,
    pub addralign: ClassUsize,
    pub entsize: ClassUsize,
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SectionType {
    Null = 0x0,
    Progbits = 0x1,
    Symtab = 0x2,
    Strtab = 0x3,
    Rela = 0x4,
    Hash = 0x5,
    Dynamic = 0x6,
    Note = 0x7,
    Nobits = 0x8,
    Rel = 0x9,
    Shlib = 0xa,
    Dynsym = 0xb,
    InitArray = 0xe,
    FiniArray = 0xf,
    PreinitArray = 0x10,
    Group = 0x11,
    SymtabShndx = 0x12,
    Num = 0x13,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SectionFlags {
    Flags32(SectionFlags32),
    Flags64(SectionFlags64),
}

macro_rules! section_flags {
    ($name:ident, $t:ty) => {
        bitfield! {
            #[derive(Copy, Clone, PartialEq, Eq)]
            pub struct $name($t);

            derive_debug;

            #[ro] pub field write: bool = [0];
            #[ro] pub field alloc: bool = [1];
            #[ro] pub field execinstr: bool = [2];
            #[ro] pub field merge: bool = [4];
            #[ro] pub field strings: bool = [5];
            #[ro] pub field info_link: bool = [6];
            #[ro] pub field link_order: bool = [7];
            #[ro] pub field os_nonconforming: bool = [8];
            #[ro] pub field group: bool = [9];
            #[ro] pub field tls: bool = [10];
        }
    };
}

section_flags!(SectionFlags32, u32);
section_flags!(SectionFlags64, u64);

macro_rules! sf_indirection {
    ($($f:ident),*) => {$(
        impl SectionFlags {
            pub fn $f(&self) -> bool {
                match self {
                    SectionFlags::Flags32(f) => f.$f(),
                    SectionFlags::Flags64(f) => f.$f(),
                }
            }
        }
    )*};
}

sf_indirection!(
    write,
    alloc,
    execinstr,
    merge,
    strings,
    info_link,
    link_order,
    os_nonconforming,
    group,
    tls
);

impl SectionHeader {
    fn read(
        bytes: ByteStream,
        class: Class,
        endianness: Endianness,
    ) -> Result<SectionHeader, Error> {
        Ok(SectionHeader {
            name: bytes.read_u32(endianness)?,
            ty: SectionType::try_from_repr(bytes.read_u32(endianness)?).ok_or(Error::Invalid)?,
            flags: match class {
                Class::Class32 => {
                    SectionFlags::Flags32(SectionFlags32(bytes.read_u32(endianness)?))
                }
                Class::Class64 => {
                    SectionFlags::Flags64(SectionFlags64(bytes.read_u64(endianness)?))
                }
            },
            addr: ClassUsize::read(bytes, class, endianness)?,
            offset: ClassUsize::read(bytes, class, endianness)?,
            size: ClassUsize::read(bytes, class, endianness)?,
            link: bytes.read_u32(endianness)?,
            info: bytes.read_u32(endianness)?,
            addralign: ClassUsize::read(bytes, class, endianness)?,
            entsize: ClassUsize::read(bytes, class, endianness)?,
        })
    }
}

pub fn file_header(elf: &[u8]) -> Result<FileHeader, Error> {
    FileHeader::read(&mut ByteBuffer(elf))
}

fn off(elf: &[u8], off: ClassUsize) -> Result<usize, Error> {
    match off {
        ClassUsize::Usize32(off) if off <= elf.len() as u32 => Ok(off as usize),
        ClassUsize::Usize64(off) if off <= elf.len() as u64 => Ok(off as usize),
        _ => Err(Error::Invalid),
    }
}

pub fn program_header(
    elf: &[u8],
    file_header: &FileHeader,
    i: usize,
) -> Result<ProgramHeader, Error> {
    let phoff = off(elf, file_header.phoff)?;
    let phentsize = file_header.phentsize as usize;
    let offset = phoff.checked_add(i * phentsize).ok_or(Error::Overflow)?;
    if offset.checked_add(phentsize).ok_or(Error::Overflow)? > elf.len() {
        return Err(Error::Invalid);
    }
    let program_header = ProgramHeader::read(
        &mut ByteBuffer(&elf[offset..offset + phentsize]),
        file_header.class,
        file_header.endianness,
    )?;
    if program_header
        .offset
        .try_into_usize()?
        .checked_add(program_header.filesz.try_into_usize()?)
        .ok_or(Error::Overflow)?
        > elf.len()
    {
        return Err(Error::Invalid);
    }
    Ok(program_header)
}

pub fn program_headers<'a>(
    elf: &'a [u8],
    file_header: &'a FileHeader,
) -> Result<impl 'a + Clone + Iterator<Item = Result<ProgramHeader, Error>>, Error> {
    Ok((0..file_header.phnum as usize).map(move |i| program_header(elf, file_header, i)))
}

pub fn section_header(
    elf: &[u8],
    file_header: &FileHeader,
    i: usize,
) -> Result<SectionHeader, Error> {
    let shoff = off(elf, file_header.shoff)?;
    let shentsize = file_header.shentsize as usize;
    let offset = shoff.checked_add(i * shentsize).ok_or(Error::Overflow)?;
    if offset.checked_add(shentsize).ok_or(Error::Overflow)? > elf.len() {
        return Err(Error::Invalid);
    }
    let section_header = SectionHeader::read(
        &mut ByteBuffer(&elf[offset..offset + shentsize]),
        file_header.class,
        file_header.endianness,
    )?;
    if section_header
        .offset
        .try_into_usize()?
        .checked_add(section_header.size.try_into_usize()?)
        .ok_or(Error::Overflow)?
        > elf.len()
    {
        return Err(Error::Invalid);
    }
    Ok(section_header)
}

pub fn section_headers<'a>(
    elf: &'a [u8],
    file_header: &'a FileHeader,
) -> Result<impl 'a + Clone + Iterator<Item = Result<SectionHeader, Error>>, Error> {
    Ok((0..file_header.shnum as usize).map(move |i| section_header(elf, file_header, i)))
}

pub fn section_name<'a>(
    elf: &'a [u8],
    file_header: &FileHeader,
    name: u32,
) -> Result<&'a str, Error> {
    let string_section = section_header(elf, file_header, file_header.shstrndx as usize)?;
    let string_section_offset = off(elf, string_section.offset)?;
    let string_section_end = string_section_offset
        .checked_add(
            match string_section.size {
                ClassUsize::Usize32(x) => x.try_into(),
                ClassUsize::Usize64(x) => x.try_into(),
            }
            .map_err(|_| Error::Overflow)?,
        )
        .ok_or(Error::Overflow)?;
    let string_start = string_section_offset
        .checked_add(name as usize)
        .ok_or(Error::Overflow)?;

    let mut len = 0;
    for i in string_start..string_section_end {
        match elf[i] {
            0 => break,
            _ => len += 1,
        }
    }
    Ok(str::from_utf8(&elf[string_start..string_start + len])?)
}
