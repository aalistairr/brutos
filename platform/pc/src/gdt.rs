use core::pin::Pin;

use brutos_util_macros::{bitfield, BitEnum};

pub unsafe fn load<T>(gdt: Pin<&T>) {
    assert!(core::mem::size_of::<T>() % 4 == 0);
    assert!(core::mem::size_of::<T>() - 1 < core::u16::MAX as usize);
    assert!(core::mem::align_of::<T>() % 4 == 0);
    let limit: u16 = (core::mem::size_of::<T>() - 1) as u16;
    let addr: u64 = &*gdt as *const T as u64;
    asm!("
        sub $$0x10, %rsp
        mov %ax, 0x6(%rsp)
        mov %rdi, 0x8(%rsp)
        lgdt 0x6(%rsp)
        add $$0x10, %rsp
    " :: "{ax}" (limit), "{rdi}" (addr) :: "volatile");
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    pub struct CDDescriptor([u32; 2]);

    field ty: CDType => 1[8..13];
    field limit: u16 => 0[0..16];
    pub field dpl: usize => 1[13..15];
    pub field present: bool => 1[15];
    pub field long: bool => 1[21];
    field db: bool => 1[22];
    field granularity_4k: bool => 1[23];
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum CDType {
    Code = 0b11011,
    Data = 0b10011,
}

impl CDDescriptor {
    pub const fn new(ty: CDType) -> CDDescriptor {
        CDDescriptor([0; 2])
            .with_ty(ty)
            .with_limit(0xffff)
            .with_granularity_4k(true)
            .with_db(match ty {
                CDType::Data => true,
                CDType::Code => false,
            })
    }
}
