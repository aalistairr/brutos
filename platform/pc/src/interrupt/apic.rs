use core::cell::UnsafeCell;

use brutos_util_macros::{bitfield, BitEnum, ConvertInner};

use crate::msr::{self, RW as _};

#[repr(C, align(16))]
pub struct Register {
    value: UnsafeCell<u32>,
    _padding: [u32; 3],
}

#[repr(transparent)]
pub struct Apic([Register; 64]);

pub unsafe trait Reg {
    type Value;
}
pub trait R: Reg {
    unsafe fn read(apic: &Apic) -> Self::Value;
}
pub trait W: Reg {
    unsafe fn write(apic: &Apic, value: Self::Value);
}

impl Apic {
    pub unsafe fn global_enable() {
        msr::Ia32ApicBase::map(|x| x.with_enabled(true));
    }

    pub unsafe fn read<A>(&self) -> A::Value
    where
        A: Reg + R,
    {
        A::read(self)
    }

    pub unsafe fn write<A>(&mut self, value: A::Value)
    where
        A: Reg + W,
    {
        A::write(self, value);
    }

    pub unsafe fn map<A, F>(&mut self, f: F)
    where
        A: Reg + R + W,
        F: FnOnce(A::Value) -> A::Value,
    {
        self.write::<A>(f(self.read::<A>()));
    }
}

pub mod reg {
    use super::{Apic, Reg, Register, R, W};

    macro_rules! reg {
    ($offset:expr => $name:ident: $t:ty = $($access:tt)*) => {
        pub enum $name {}

        impl $name {
            const INDEX: usize = {
                assert!($offset & (core::mem::align_of::<Register>() - 1) == 0);
                $offset / core::mem::size_of::<Register>()
            };
        }

        unsafe impl Reg for $name {
            type Value = $t;
        }

        $(reg_access!($name: $t = $access);)*
    }
}

    macro_rules! reg_access {
        ($name:ident: $t:ty = R) => {
            impl R for $name {
                unsafe fn read(apic: &Apic) -> $t {
                    core::ptr::read_volatile(apic.0[Self::INDEX].value.get()).into()
                }
            }
        };
        ($name:ident: $t:ty = W) => {
            impl W for $name {
                unsafe fn write(apic: &Apic, value: $t) {
                    core::ptr::write_volatile(apic.0[Self::INDEX].value.get(), value.into());
                }
            }
        };
    }

    reg!(0x020 => ApicId: u32 = R W);
    reg!(0x030 => ApicVersion: u32 = R);
    reg!(0x080 => TaskPriority: u32 = R W);
    reg!(0x090 => ArbitrationPriority: u32 = R);
    reg!(0x0a0 => ProcessorPriority: u32 = R);
    reg!(0x0b0 => EndOfInterrupt: u32 = W);
    reg!(0x0c0 => RemoteRead: u32 = R);
    reg!(0x0d0 => LogicalDestination: super::LogicalDestination = R W);
    reg!(0x0e0 => DestinationFormat: super::DestinationFormat = R W);
    reg!(0x0f0 => SpuriousInterruptVector: super::SpuriousInterruptVector = R W);
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
    reg!(0x280 => ErrorStatus: super::ErrorStatus = R);
    reg!(0x2f0 => LvtCorrectedMachineCheckInterrupt: super::LvtRegister = R W);
    // reg!(0x300 => InterruptCommandReg0: u32 = R W);
    // reg!(0x310 => InterruptCommandReg1: u32 = R W);
    reg!(0x320 => LvtTimer: super::Timer = R W);
    reg!(0x330 => LvtThermalSensor: super::LvtRegister = R W);
    reg!(0x340 => LvtPerformanceMonitoringCounters: super::LvtRegister = R W);
    reg!(0x350 => LvtLInt0: super::LInt = R W);
    reg!(0x360 => LvtLInt1: super::LInt = R W);
    reg!(0x370 => LvtError: super::ErrorRegister = R W);
    reg!(0x380 => InitialCount: u32 = R W);
    reg!(0x390 => CurrentCount: u32 = R);
    reg!(0x3e0 => DivideConfiguration: super::DivideConfiguration = R W);

    pub enum InterruptCommandReg {}

    impl InterruptCommandReg {
        const INDEX0: usize = 0x300 / core::mem::size_of::<Register>();
        const INDEX1: usize = 0x310 / core::mem::size_of::<Register>();
    }

    unsafe impl Reg for InterruptCommandReg {
        type Value = super::InterruptCommand;
    }

    impl R for InterruptCommandReg {
        unsafe fn read(apic: &Apic) -> super::InterruptCommand {
            let r0 = core::ptr::read_volatile(apic.0[Self::INDEX0].value.get());
            let r1 = core::ptr::read_volatile(apic.0[Self::INDEX1].value.get());
            super::InterruptCommand([r0, r1])
        }
    }

    impl W for InterruptCommandReg {
        unsafe fn write(apic: &Apic, interrupt_command: super::InterruptCommand) {
            let [r0, r1] = interrupt_command.0;
            core::ptr::write_volatile(apic.0[Self::INDEX1].value.get(), r1);
            core::ptr::write_volatile(apic.0[Self::INDEX0].value.get(), r0);
        }
    }
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum DeliveryStatus {
    Idle = 0,
    SendPending = 1,
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum DeliveryMode {
    Fixed = 0b000,
    Smi = 0b010,
    Nmi = 0b100,
    ExtInt = 0b111,
    Init = 0b101,
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

bitfield! {
    #[derive(Copy, Clone, ConvertInner)]
    pub struct Timer(u32);

    pub field vector: u8 => 0..8;
    #[ro] pub field delivery_status: DeliveryStatus => 12;
    pub field masked: bool => 16;
    pub field timer_mode: TimerMode => 17..19;
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum TimerMode {
    OneShot = 0b00,
    Periodic = 0b01,
    TscDeadline = 0b10,
}

bitfield! {
    #[derive(Copy, Clone, ConvertInner)]
    pub struct LInt(u32);

    pub field vector: u8 => 0..8;
    pub field delivery_mode_raw: DeliveryMode => 8..11;
    #[ro] pub field delivery_status: DeliveryStatus => 12;
    pub field interrupt_input_pin_polarity: PinPolarity => 13;
    pub field remote_irr: bool => 14;
    pub field trigger_mode: TriggerMode => 15;
    pub field masked: bool => 16;
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum PinPolarity {
    ActiveHigh = 0,
    ActiveLow = 1,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct LvtRegister(u32);

    pub field vector: u8 => 0..8;
    pub field delivery_mode: DeliveryMode => 8..11;
    #[ro] pub field delivery_status: DeliveryStatus => 12;
    pub field masked: bool => 16;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct ErrorRegister(u32);

    pub field vector: u8 => 0..8;
    #[ro] pub field delivery_status: DeliveryStatus => 12;
    pub field masked: bool => 16;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct ErrorStatus(u32);

    pub field send_checksum: bool => 0;
    pub field recv_checksum: bool => 1;
    pub field send_accept: bool => 2;
    pub field recv_accept: bool => 3;
    pub field redirectable_ipi: bool => 4;
    pub field send_illegal_vector: bool => 5;
    pub field received_illegal_vector: bool => 6;
    pub field illegal_register_address: bool => 7;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct DivideConfiguration(u32);

    pub field divide_value: DivideValue => 0..2 ~ 3;
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum DivideValue {
    By2 = 0b000,
    By4 = 0b001,
    By8 = 0b010,
    By16 = 0b011,
    By32 = 0b100,
    By64 = 0b101,
    By128 = 0b110,
    By1 = 0b111,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct InterruptCommand([u32; 2]);

    pub field vector: u8 => 0[0..8];
    pub field delivery_mode: DeliveryMode => 0[8..11];
    pub field destination_mode: DestinationMode => 0[11];
    #[ro] pub field delivery_status: DeliveryStatus => 0[12];
    pub field level_assert: bool => 0[14];
    pub field trigger_mode: TriggerMode => 0[15];
    pub field destination_shorthand: DestinationShorthand => 0[18..20];
    pub field destination: u8 => 1[24..32];
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum DestinationMode {
    Physical = 0,
    Logical = 1,
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum DestinationShorthand {
    NoShorthand = 0b00,
    Selff = 0b01,
    AllIncludingSelff = 0b10,
    AllExcludingSelff = 0b11,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct LogicalDestination(u32);

    pub field logical_apic_id: u8 => 24..32;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct DestinationFormat(u32);

    pub field model: Model => 28..32;
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Model {
    Cluster = 0b0000,
    Flat = 0b1111,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct SpuriousInterruptVector(u32);

    pub field vector: u8 => 0..8;
    pub field apic_enabled: bool => 8;
    pub field focus_processor_checking_enabled: bool => 9;
    pub field eoi_broadcast_suppression_enabled: bool => 12;
}
