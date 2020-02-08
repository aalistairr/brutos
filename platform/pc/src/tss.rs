use core::ptr::NonNull;

use brutos_util_macros::{bitfield, BitEnum};

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct Descriptor([u32; 4]);

    field base_address: usize = 0[16..32] ~ 1[0..8] ~ 1[24..32] ~ 2[0..32];
    field segment_limit: usize = 0[0..16] ~ 1[16..20];
    field ty: Type = 1[8..12];
    pub field dpl: usize = 1[13..15];
    pub field present: bool = 1[15];
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Type {
    TssAvailable = 0b1001,
    TssBusy = 0b1011,
}

impl Descriptor {
    pub const fn new(ty: Type) -> Descriptor {
        Descriptor([0; 4]).with_ty(ty)
    }

    pub const fn tss_size(&self) -> usize {
        self.segment_limit() + 1
    }

    pub const fn set_tss_size(&mut self, size: usize) {
        assert!(size >= core::mem::size_of::<Tss>());
        self.set_segment_limit(size - 1);
    }

    pub const fn with_tss_size(mut self, size: usize) -> Self {
        self.set_tss_size(size);
        self
    }

    pub const fn tss(&self) -> Option<NonNull<Tss>> {
        match self.base_address() {
            0 => None,
            addr => Some(unsafe { NonNull::new_unchecked((addr - TSS_PADDING) as *mut Tss) }),
        }
    }

    pub fn set_tss(&mut self, tss: Option<NonNull<Tss>>) {
        self.set_base_address(match tss {
            None => 0,
            Some(tss) => tss.as_ptr() as usize + TSS_PADDING,
        });
    }

    pub fn with_tss(mut self, tss: Option<NonNull<Tss>>) -> Self {
        self.set_tss(tss);
        self
    }
}

const TSS_PADDING: usize = core::mem::size_of::<u32>();

#[derive(PartialEq, Eq, Debug, Default)]
#[repr(C, align(8))]
pub struct Tss {
    _padding0: u32,
    _reserved0: u32,
    pub rsp: [u64; 3],
    _reserved1: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    _reserved2: u64,
    _reserved3: u16,
    pub io_map_offset: u16,
}

impl Tss {
    pub const fn new() -> Tss {
        Tss {
            _padding0: 0,
            _reserved0: 0,
            rsp: [0; 3],
            _reserved1: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            _reserved2: 0,
            _reserved3: 0,
            io_map_offset: 0,
        }
    }
}

pub unsafe fn load(gdt_entry: u16) {
    asm!("ltr %ax" :: "{ax}" (gdt_entry) :: "volatile");
}
