use core::pin::Pin;

use brutos_util_macros::bitfield;

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
    #[repr(transparent)]
    pub struct Descriptor([u32; 4]);

    pub field offset: usize => 0[0..16] ~ 1[16..32] ~ 2[0..32];
    pub field segment: u16 => 0[16..32];
    field raw_ist: usize => 1[0..3];
    field raw_ty: usize => 1[8..12];
    field raw_dpl: usize => 1[13..15];
    pub field present: bool => 1[15];
}

pub enum Type {
    Reserved = 0,
    Interrupt = 0b1110,
    Trap = 0b1111,
}

impl Descriptor {
    pub const fn new() -> Descriptor {
        Descriptor([0; 4])
    }

    pub const fn ty(&self) -> Type {
        match self.raw_ty() {
            0b1110 => Type::Interrupt,
            0b1111 => Type::Trap,
            _ => Type::Reserved,
        }
    }

    pub const fn set_ty(&mut self, ty: Type) {
        self.set_raw_ty(ty as usize)
    }

    pub const fn with_ty(mut self, ty: Type) -> Self {
        self.set_ty(ty);
        self
    }
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
