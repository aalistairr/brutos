use brutos_util_macros::{bitfield, ConvertInner};

unsafe fn read_address(addr: u32) -> u64 {
    let (lo, hi): (u64, u64);
    asm!("rdmsr" : "={rax}" (lo), "={rdx}" (hi) : "{rcx}" (addr) :: "volatile");
    lo | (hi << 32)
}

unsafe fn write_address(addr: u32, value: u64) {
    let addr = addr as u64;
    let (lo, hi): (u64, u64) = (value & 0xffffffff, value >> 32);
    asm!("wrmsr" :: "{rcx}" (addr), "{rax}" (lo), "{rdx}" (hi) : "memory" : "volatile");
}

pub unsafe trait Msr: Sized {
    type Value: From<u64> + Into<u64>;
    const ADDRESS: u32;
}

pub trait R: Msr {
    unsafe fn read() -> Self::Value {
        read_address(Self::ADDRESS).into()
    }
}
pub trait W: Msr {
    unsafe fn write(value: Self::Value) {
        write_address(Self::ADDRESS, value.into());
    }
}
pub trait RW: R + W {
    unsafe fn map<F: FnOnce(Self::Value) -> Self::Value>(f: F) {
        Self::write(f(Self::read()));
    }
}
impl<A: R + W> RW for A {}

macro_rules! msr {
    ($addr:expr => $name:ident: $value:ty = $($access:ident)*) => {
        pub enum $name {}

        unsafe impl Msr for $name {
            type Value = $value;
            const ADDRESS: u32 = $addr;
        }

        $(impl $access for $name {})*
    }
}

msr!(0x1b => Ia32ApicBase: ApicBase = R W);
msr!(0x277 => Ia32Pat: Pat = R W);
msr!(0xc000_0080 => Ia32Efer: Efer = R W);
msr!(0xc000_0082 => Ia32LStar: u64 = R W);
msr!(0xc000_0084 => Ia32FMask: u64 = R W);
msr!(0xc000_0100 => Ia32FsBase: u64 = R W);
msr!(0xc000_0101 => Ia32GsBase: u64 = R W);
msr!(0xc000_0102 => Ia32KernelGsBase: u64 = R W);

bitfield! {
    #[derive(Copy, Clone, ConvertInner)]
    pub struct ApicBase(u64);

    pub field bsp: bool => 8;
    pub field x2apic_enabled: bool => 10;
    pub field enabled: bool => 11;
    pub field base: u64 { 12..48 => 12..48 }
}

bitfield! {
    #[derive(Copy, Clone, ConvertInner)]
    pub struct Pat(u64);

    pub field pa0: usize => 0..3;
    pub field pa1: usize => 8..11;
    pub field pa2: usize => 16..19;
    pub field pa3: usize => 24..27;
    pub field pa4: usize => 32..35;
    pub field pa5: usize => 40..43;
    pub field pa6: usize => 48..51;
    pub field pa7: usize => 56..59;
}

bitfield! {
    #[derive(Copy, Clone, ConvertInner)]
    pub struct Efer(u64);

    pub field syscall_enabled: bool => 0;
    pub field ia32_enabled: bool => 8;
    field ia32_active_raw: bool => 10;
    pub field nx_enabled: bool => 11;
}

impl Efer {
    pub fn is_ia32_active(&self) -> bool {
        self.is_ia32_active_raw()
    }
}
