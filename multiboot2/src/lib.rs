#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::mem;
use core::ops::Range;
use core::slice;
use core::str;

use brutos_memory::PhysAddr;

pub mod ffi;

pub enum Tag<'a> {
    Unknown,
    CommandLine(&'a str),
    BootloaderName(&'a str),
    Module {
        range: Range<PhysAddr>,
        name: &'a str,
    },
    MemoryMap(Mmap<'a>),
}

#[derive(Copy, Clone)]
pub struct Mmap<'a> {
    len: usize,
    entry_size: usize,
    entry_bytes: &'a [u8],
}

#[derive(Copy, Clone)]
pub struct TagIter<'a>(&'a [u8]);

impl ffi::BootInfo {
    pub fn tags(&self) -> TagIter {
        let ptr = self.tags.as_ptr();
        let len = self.size as usize - mem::size_of::<ffi::BootInfo>();
        unsafe { TagIter(slice::from_raw_parts(ptr, len)) }
    }
}

fn read_bytes<'a>(bytes: &mut &'a [u8], len: usize) -> &'a [u8] {
    let (x, xs) = bytes.split_at(len);
    *bytes = xs;
    x
}

fn read_str<'a>(bytes: &mut &'a [u8]) -> &'a str {
    let bytes = read_bytes(bytes, bytes.len());
    let mut len = 0;
    while len < bytes.len() && bytes[len] != 0 {
        len += 1;
    }
    str::from_utf8(&bytes[..len]).expect("Invalid UTF-8")
}

unsafe fn read_value<'a, T>(bytes: &mut &'a [u8]) -> &'a T {
    &*(read_bytes(bytes, mem::size_of::<T>()).as_ptr() as *const T)
}

impl<'a> Iterator for TagIter<'a> {
    type Item = Tag<'a>;

    fn next(&mut self) -> Option<Tag<'a>> {
        if self.0.len() == 0 {
            return None;
        }

        let header = unsafe { read_value::<ffi::TagHeader>(&mut self.0) };
        let mut body = read_bytes(
            &mut self.0,
            header.size as usize - mem::size_of::<ffi::TagHeader>(),
        );
        self.0 = &self.0[self
            .0
            .as_ptr()
            .align_offset(mem::align_of::<ffi::TagHeader>())..];

        Some(match header.ty {
            ffi::TAG_COMMAND_LINE => Tag::CommandLine(read_str(&mut body)),
            ffi::TAG_BOOTLOADER_NAME => Tag::BootloaderName(read_str(&mut body)),
            ffi::TAG_MODULE => {
                let module = unsafe { read_value::<ffi::Module>(&mut body) };
                Tag::Module {
                    range: PhysAddr(module.start as usize)..PhysAddr(module.end as usize),
                    name: read_str(&mut body),
                }
            }
            ffi::TAG_MMAP => {
                let mmap = unsafe { read_value::<ffi::Mmap>(&mut body) };
                let entry_size = mmap.entry_size as usize;
                assert!(entry_size >= mem::size_of::<ffi::MmapEntry>());
                Tag::MemoryMap(Mmap {
                    len: body.len() / entry_size,
                    entry_size: entry_size,
                    entry_bytes: body,
                })
            }
            _ => Tag::Unknown,
        })
    }
}

pub struct MmapEntry {
    pub range: Range<PhysAddr>,
    pub ty: MmapEntryTy,
}

#[derive(PartialEq, Eq)]
pub enum MmapEntryTy {
    Unknown,
    Available,
    AcpiReclaimable,
    SaveOnHibernate,
    Defective,
}

impl<'a> Mmap<'a> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, i: usize) -> MmapEntry {
        assert!(i < self.len);
        let entry = &self.entry_bytes[i * self.entry_size..(i + 1) * self.entry_size];
        let entry = unsafe { &*(entry.as_ptr() as *const ffi::MmapEntry) };
        let addr = entry.address as usize;
        MmapEntry {
            range: PhysAddr(addr)..PhysAddr(addr + entry.length as usize),
            ty: match entry.ty {
                ffi::MMAP_ENTRY_TY_AVAILABLE => MmapEntryTy::Available,
                ffi::MMAP_ENTRY_TY_ACPI_RECLAIMABLE => MmapEntryTy::AcpiReclaimable,
                ffi::MMAP_ENTRY_TY_SAVE_ON_HIBERNATE => MmapEntryTy::SaveOnHibernate,
                ffi::MMAP_ENTRY_TY_DEFECTIVE => MmapEntryTy::Defective,
                _ => MmapEntryTy::Unknown,
            },
        }
    }

    pub fn iter<'this>(&'this self) -> impl 'this + Clone + Iterator<Item = MmapEntry> {
        (0..self.len).map(move |i| self.get(i))
    }
}
