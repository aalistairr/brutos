use core::cell::UnsafeCell;

use crate::msr::{self, RW as _};

#[repr(C, align(16))]
pub struct Register {
    value: UnsafeCell<u32>,
    _padding: [u32; 3],
}

#[repr(transparent)]
pub struct Apic([Register; 64]);

pub unsafe trait Reg {
    type Value: From<u32> + Into<u32>;
    const INDEX: usize;
}
pub trait R {}
pub trait W {}

impl Apic {
    pub unsafe fn enable() {
        msr::Ia32ApicBase::map(|x| x.with_enabled(true));
    }

    pub unsafe fn read<A: Reg + R>(&self) -> A::Value {
        core::ptr::read_volatile(self.0[A::INDEX].value.get()).into()
    }

    pub unsafe fn write<A: Reg + W>(&mut self, value: A::Value) {
        core::ptr::write_volatile(self.0[A::INDEX].value.get(), value.into())
    }
}

macro_rules! reg {
    ($offset:expr => $name:ident: $t:ty = $($access:ident)*) => {
        pub enum $name {}

        unsafe impl Reg for $name {
            type Value = $t;
            const INDEX: usize = {
                assert!($offset & (core::mem::align_of::<Register>() - 1) == 0);
                $offset / core::mem::size_of::<Register>()
            };
        }

        $(impl $access for $name {})*
    }
}

reg!(0x020 => ApicId: u32 = R W);
reg!(0x030 => ApicVersion: u32 = R);
reg!(0x080 => TaskPriority: u32 = R W);
reg!(0x090 => ArbitrationPriority: u32 = R);
reg!(0x0a0 => ProcessorPriority: u32 = R);
reg!(0x0b0 => EndOfInterrupt: u32 = W);
reg!(0x0c0 => RemoteRead: u32 = R);
reg!(0x0d0 => LogicalDestination: u32 = R W);
reg!(0x0e0 => DestinationFormat: u32 = R W);
reg!(0x0f0 => SpuriousInterruptVector: u32 = R W);
reg!(0x100 => InService0_32: u32 = R);
reg!(0x110 => InService32_64: u32 = R);
reg!(0x120 => InService64_96: u32 = R);
reg!(0x130 => InService96_128: u32 = R);
reg!(0x140 => InService128_160: u32 = R);
reg!(0x150 => InService160_192: u32 = R);
reg!(0x160 => InService192_224: u32 = R);
reg!(0x170 => InService224_256: u32 = R);
reg!(0x180 => TriggerMode0_32: u32 = R);
reg!(0x190 => TriggerMode32_64: u32 = R);
reg!(0x1a0 => TriggerMode64_96: u32 = R);
reg!(0x1b0 => TriggerMode96_128: u32 = R);
reg!(0x1c0 => TriggerMode128_160: u32 = R);
reg!(0x1d0 => TriggerMode160_192: u32 = R);
reg!(0x1e0 => TriggerMode192_224: u32 = R);
reg!(0x1f0 => TriggerMode224_256: u32 = R);
reg!(0x200 => InterruptRequest0_32: u32 = R);
reg!(0x210 => InterruptRequest32_64: u32 = R);
reg!(0x220 => InterruptRequest64_96: u32 = R);
reg!(0x230 => InterruptRequest96_128: u32 = R);
reg!(0x240 => InterruptRequest128_160: u32 = R);
reg!(0x250 => InterruptRequest160_192: u32 = R);
reg!(0x260 => InterruptRequest192_224: u32 = R);
reg!(0x270 => InterruptRequest224_256: u32 = R);
reg!(0x280 => ErrorStatus: u32 = R);
reg!(0x2f0 => LvtCorrectedMachineCheckInterrupt: u32 = R W);
reg!(0x300 => InterruptCommand0: u32 = R W);
reg!(0x310 => InterruptCommand1: u32 = R W);
reg!(0x320 => LvtTimer: u32 = R W);
reg!(0x330 => LvtThermalSensor: u32 = R W);
reg!(0x340 => LvtPerformanceMonitoringCounters: u32 = R W);
reg!(0x350 => LvtLInt0: u32 = R W);
reg!(0x360 => LvtLInt1: u32 = R W);
reg!(0x370 => LvtError: u32 = R W);
reg!(0x380 => InitialCount: u32 = R W);
reg!(0x390 => CurrentCount: u32 = R);
reg!(0x3e0 => DivideConfiguration: u32 = R W);
