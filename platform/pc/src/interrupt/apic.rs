use core::cell::UnsafeCell;

use brutos_util_macros::{bitfield, ConvertInner};

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DeliveryStatus {
    Idle,
    SendPending,
}

macro_rules! impl_delivery_status {
    ($t:ty) => {
        impl $t {
            pub const fn delivery_status(&self) -> DeliveryStatus {
                if self.is_pending() {
                    DeliveryStatus::SendPending
                } else {
                    DeliveryStatus::Idle
                }
            }
        }
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PinPolarity {
    ActiveHigh,
    ActiveLow,
}

macro_rules! impl_pin_polarity {
    ($t:ty) => {
        impl $t {
            pub const fn interrupt_input_pin_polarity(&self) -> PinPolarity {
                if self.is_interrupt_input_pin_active_low() {
                    PinPolarity::ActiveLow
                } else {
                    PinPolarity::ActiveHigh
                }
            }
            pub const fn set_interrupt_input_pin_polarity(&mut self, pin_polarity: PinPolarity) {
                self.set_interrupt_input_pin_active_low(match pin_polarity {
                    PinPolarity::ActiveLow => true,
                    PinPolarity::ActiveHigh => false,
                });
            }

            pub const fn with_interrupt_input_pin_polarity(
                mut self,
                pin_polarity: PinPolarity,
            ) -> Self {
                self.set_interrupt_input_pin_polarity(pin_polarity);
                self
            }
        }
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DeliveryMode {
    Fixed = 0b000,
    Smi = 0b010,
    Nmi = 0b100,
    ExtInt = 0b111,
    Init = 0b101,
}

macro_rules! impl_delivery_mode {
    ($t:ty) => {
        impl $t {
            pub const fn delivery_mode(&self) -> DeliveryMode {
                match self.delivery_mode_raw() {
                    0b000 => DeliveryMode::Fixed,
                    0b010 => DeliveryMode::Smi,
                    0b100 => DeliveryMode::Nmi,
                    0b111 => DeliveryMode::ExtInt,
                    0b101 => DeliveryMode::Init,
                    _ => panic!("invalid delivery mode"),
                }
            }
            pub const fn set_delivery_mode(&mut self, delivery_mode: DeliveryMode) {
                self.set_delivery_mode_raw(delivery_mode as usize)
            }
            pub const fn with_delivery_mode(mut self, delivery_mode: DeliveryMode) -> Self {
                self.set_delivery_mode(delivery_mode);
                self
            }
        }
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TriggerMode {
    Edge,
    Level,
}

macro_rules! impl_trigger_mode {
    ($t:ty) => {
        impl $t {
            pub const fn trigger_mode(&self) -> TriggerMode {
                if self.is_level_triggered() {
                    TriggerMode::Level
                } else {
                    TriggerMode::Edge
                }
            }
            pub const fn set_trigger_mode(&mut self, trigger_mode: TriggerMode) {
                self.set_level_triggered(match trigger_mode {
                    TriggerMode::Level => true,
                    TriggerMode::Edge => false,
                });
            }
            pub const fn with_trigger_mode(mut self, trigger_mode: TriggerMode) -> Self {
                self.set_trigger_mode(trigger_mode);
                self
            }
        }
    };
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TimerMode {
    OneShot = 0b00,
    Periodic = 0b01,
    TscDeadline = 0b10,
}

macro_rules! impl_timer_mode {
    ($t:ty) => {
        impl $t {
            pub const fn timer_mode(&self) -> TimerMode {
                match self.timer_mode_raw() {
                    0b00 => TimerMode::OneShot,
                    0b01 => TimerMode::Periodic,
                    0b10 => TimerMode::TscDeadline,
                    _ => panic!("invalid timer mode"),
                }
            }
            pub const fn set_timer_mode(&mut self, timer_mode: TimerMode) {
                self.set_timer_mode_raw(timer_mode as usize);
            }
            pub const fn with_timer_mode(mut self, timer_mode: TimerMode) -> Self {
                self.set_timer_mode(timer_mode);
                self
            }
        }
    };
}

bitfield! {
    #[derive(Copy, Clone, ConvertInner)]
    pub struct Timer(u32);

    pub field vector: u8 => 0..8;
    field pending: bool => 12;
    pub field masked: bool => 16;
    field timer_mode_raw: usize => 17..19;
}

impl_delivery_status!(Timer);
impl_timer_mode!(Timer);

bitfield! {
    #[derive(Copy, Clone, ConvertInner)]
    pub struct LInt(u32);

    pub field vector: u8 => 0..8;
    field delivery_mode_raw: usize => 8..11;
    field pending: bool => 12;
    field interrupt_input_pin_active_low: bool => 13;
    pub field remote_irr: bool => 14;
    field level_triggered: bool => 15;
    pub field masked: bool => 16;
}

impl_delivery_mode!(LInt);
impl_delivery_status!(LInt);
impl_trigger_mode!(LInt);
impl_pin_polarity!(LInt);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct LvtRegister(u32);

    pub field vector: u8 => 0..8;
    field delivery_mode_raw: usize => 8..11;
    field pending: bool => 12;
    pub field masked: bool => 16;
}

impl_delivery_mode!(LvtRegister);
impl_delivery_status!(LvtRegister);

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct ErrorRegister(u32);

    pub field vector: u8 => 0..8;
    field pending: bool => 12;
    pub field masked: bool => 16;
}

impl_delivery_status!(ErrorRegister);

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

    field divide_value_raw: usize => 0..2 ~ 3;
}

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

impl DivideConfiguration {
    pub const fn divide_value(&self) -> DivideValue {
        match self.divide_value_raw() {
            0b000 => DivideValue::By2,
            0b001 => DivideValue::By4,
            0b010 => DivideValue::By8,
            0b011 => DivideValue::By16,
            0b100 => DivideValue::By32,
            0b101 => DivideValue::By64,
            0b110 => DivideValue::By128,
            0b111 => DivideValue::By1,
            _ => unreachable!(),
        }
    }

    pub const fn set_divide_value(&mut self, divide_value: DivideValue) {
        self.set_divide_value_raw(divide_value as usize);
    }

    pub const fn with_divide_value(mut self, divide_value: DivideValue) -> Self {
        self.set_divide_value(divide_value);
        self
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct InterruptCommand([u32; 2]);

    pub field vector: u8 => 0[0..8];
    field delivery_mode_raw: usize => 0[8..11];
    field logical_destination: bool => 0[11];
    field pending: bool => 0[12];
    pub field level_assert: bool => 0[14];
    field level_triggered: bool => 0[15];
    field destination_shorthand_raw: usize => 0[18..20];
    pub field destination: u8 => 1[24..32];
}

impl_delivery_mode!(InterruptCommand);
impl_delivery_status!(InterruptCommand);
impl_trigger_mode!(InterruptCommand);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DestinationMode {
    Physical,
    Logical,
}

impl InterruptCommand {
    pub const fn destination_mode(&self) -> DestinationMode {
        if self.is_logical_destination() {
            DestinationMode::Logical
        } else {
            DestinationMode::Physical
        }
    }

    pub const fn set_destination_mode(&mut self, destination_mode: DestinationMode) {
        self.set_logical_destination(match destination_mode {
            DestinationMode::Logical => true,
            DestinationMode::Physical => false,
        });
    }

    pub const fn with_destination_mode(mut self, destination_mode: DestinationMode) -> Self {
        self.set_destination_mode(destination_mode);
        self
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DestinationShorthand {
    NoShorthand = 0b00,
    Selff = 0b01,
    AllIncludingSelff = 0b10,
    AllExcludingSelff = 0b11,
}

impl InterruptCommand {
    pub const fn destination_shorthand(&self) -> DestinationShorthand {
        match self.destination_shorthand_raw() {
            0b00 => DestinationShorthand::NoShorthand,
            0b01 => DestinationShorthand::Selff,
            0b10 => DestinationShorthand::AllIncludingSelff,
            0b11 => DestinationShorthand::AllExcludingSelff,
            _ => unreachable!(),
        }
    }

    pub const fn set_destination_shorthand(&mut self, destination_shorthand: DestinationShorthand) {
        self.set_destination_shorthand_raw(destination_shorthand as usize);
    }

    pub const fn with_destination_shorthand(
        mut self,
        destination_shorthand: DestinationShorthand,
    ) -> Self {
        self.set_destination_shorthand(destination_shorthand);
        self
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct LogicalDestination(u32);

    pub field logical_apic_id: u8 => 24..32;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct DestinationFormat(u32);

    field model_raw: usize => 28..32;
}

pub enum Model {
    Cluster = 0b0000,
    Flat = 0b1111,
}

impl DestinationFormat {
    pub const fn model(&self) -> Model {
        match self.model_raw() {
            0b0000 => Model::Cluster,
            0b1111 => Model::Flat,
            _ => unreachable!(),
        }
    }

    pub const fn set_model(&mut self, model: Model) {
        self.set_model_raw(model as usize);
    }

    pub const fn with_model(mut self, model: Model) -> Self {
        self.set_model(model);
        self
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, ConvertInner)]
    pub struct SpuriousInterruptVector(u32);

    pub field vector: u8 => 0..8;
    pub field apic_enabled: bool => 8;
    pub field focus_processor_checking_enabled: bool => 9;
    pub field eoi_broadcast_suppression_enabled: bool => 12;
}
