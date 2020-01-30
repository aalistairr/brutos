#[repr(C, align(8))]
pub struct BootInfo {
    pub size: u32,
    _reserved: u32,
    pub tags: [u8; 0],
}

#[repr(C, align(8))]
pub struct TagHeader {
    pub ty: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Module {
    pub start: u32,
    pub end: u32,
    pub name: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mmap {
    pub entry_size: u32,
    pub _entry_version: u32,
    pub entries: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MmapEntry {
    pub address: u64,
    pub length: u64,
    pub ty: u32,
}

pub const TAG_COMMAND_LINE: u32 = 1;
pub const TAG_BOOTLOADER_NAME: u32 = 2;
pub const TAG_MODULE: u32 = 3;
pub const TAG_MMAP: u32 = 6;

pub const MMAP_ENTRY_TY_AVAILABLE: u32 = 1;
pub const MMAP_ENTRY_TY_ACPI_RECLAIMABLE: u32 = 3;
pub const MMAP_ENTRY_TY_SAVE_ON_HIBERNATE: u32 = 4;
pub const MMAP_ENTRY_TY_DEFECTIVE: u32 = 5;
