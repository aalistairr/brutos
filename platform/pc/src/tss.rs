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
            addr => Some(unsafe { NonNull::new_unchecked(addr as *mut Tss) }),
        }
    }

    pub fn set_tss(&mut self, tss: Option<NonNull<Tss>>) {
        self.set_base_address(match tss {
            None => 0,
            Some(tss) => tss.as_ptr() as usize,
        });
    }

    pub fn with_tss(mut self, tss: Option<NonNull<Tss>>) -> Self {
        self.set_tss(tss);
        self
    }
}

bitfield! {
    #[derive(PartialEq, Eq, Debug, Default)]
    #[repr(C)]
    pub struct Tss {
        _reserved0: [u32; 1],
        rsp0_lo: u32,
        rsp0_hi: u32,
        rsp1_lo: u32,
        rsp1_hi: u32,
        rsp2_lo: u32,
        rsp2_hi: u32,
        _reserved1: [u32; 2],
        ist1_lo: u32,
        ist1_hi: u32,
        ist2_lo: u32,
        ist2_hi: u32,
        ist3_lo: u32,
        ist3_hi: u32,
        ist4_lo: u32,
        ist4_hi: u32,
        ist5_lo: u32,
        ist5_hi: u32,
        ist6_lo: u32,
        ist6_hi: u32,
        ist7_lo: u32,
        ist7_hi: u32,
        _reserved2: [u32; 2],
        _reserved3: [u16; 1],
        pub io_map_offset: u16,
    }

    pub field rsp0: u64 = rsp0_lo ~ rsp0_hi;
    pub field rsp1: u64 = rsp1_lo ~ rsp1_hi;
    pub field rsp2: u64 = rsp2_lo ~ rsp2_hi;

    pub field ist1: u64 = ist1_lo ~ ist1_hi;
    pub field ist2: u64 = ist2_lo ~ ist2_hi;
    pub field ist3: u64 = ist3_lo ~ ist3_hi;
    pub field ist4: u64 = ist4_lo ~ ist4_hi;
    pub field ist5: u64 = ist5_lo ~ ist5_hi;
    pub field ist6: u64 = ist6_lo ~ ist6_hi;
    pub field ist7: u64 = ist7_lo ~ ist7_hi;
}

impl Tss {
    pub const fn new() -> Tss {
        Tss {
            _reserved0: [0; 1],
            rsp0_lo: 0,
            rsp0_hi: 0,
            rsp1_lo: 0,
            rsp1_hi: 0,
            rsp2_lo: 0,
            rsp2_hi: 0,
            _reserved1: [0; 2],
            ist1_lo: 0,
            ist1_hi: 0,
            ist2_lo: 0,
            ist2_hi: 0,
            ist3_lo: 0,
            ist3_hi: 0,
            ist4_lo: 0,
            ist4_hi: 0,
            ist5_lo: 0,
            ist5_hi: 0,
            ist6_lo: 0,
            ist6_hi: 0,
            ist7_lo: 0,
            ist7_hi: 0,
            _reserved2: [0; 2],
            _reserved3: [0; 1],
            io_map_offset: 0,
        }
    }
}

pub unsafe fn load(gdt_entry: u16) {
    asm!("ltr %ax" :: "{ax}" (gdt_entry) :: "volatile");
}
