use core::pin::Pin;

use brutos_util_macros::{bitfield, BitEnum, BitfieldNew};

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default, Debug, BitfieldNew)]
    #[repr(transparent)]
    pub struct Descriptor([u32; 4]);

    pub field offset: usize => 0[0..16] ~ 1[16..32] ~ 2[0..32];
    pub field segment: u16 => 0[16..32];
    pub field ist: usize => 1[0..3];
    pub field ty: Type => 1[8..12];
    pub field dpl: usize => 1[13..15];
    pub field present: bool => 1[15];
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Reserved = 0b0000,
    Interrupt = 0b1110,
    Trap = 0b1111,
}

pub const IDT_LEN: usize = 256;

#[repr(transparent)]
pub struct Idt(pub [Descriptor; IDT_LEN]);

impl Idt {
    pub const fn new() -> Idt {
        Idt([Descriptor::new(); IDT_LEN])
    }

    pub unsafe fn load(this: Pin<&Self>) {
        let limit = core::mem::size_of::<Idt>() - 1;
        let this = this.get_ref() as *const Self as usize;
        asm!("
            sub $$0x10, %rsp
            mov $0, 0x6(%rsp)
            mov $1, 0x8(%rsp)
            lidtq 0x6(%rsp)
            add $$0x10, %rsp
        " :: "r" (limit), "r" (this) :: "volatile");
    }
}

impl core::ops::Index<usize> for Idt {
    type Output = Descriptor;

    fn index(&self, i: usize) -> &Descriptor {
        &self.0[i]
    }
}

impl core::ops::IndexMut<usize> for Idt {
    fn index_mut(&mut self, i: usize) -> &mut Descriptor {
        &mut self.0[i]
    }
}