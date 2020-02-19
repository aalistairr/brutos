use core::{i16, isize};

use bitbash::ConvertRepr;

use crate::arch::Rets;
use crate::Object;

pub trait Convert<T>: Sized {
    fn convert_from(x: T) -> Option<Self>;
    fn convert_into(self) -> Option<T>;
}

pub trait Error: ConvertRepr {}

macro_rules! Error_output_enum {
    ($name:ident { $($variants:tt)* }) => {
        #[derive(bitbash::BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
        #[non_exhaustive]
        #[repr(isize)]
        pub enum $name {
            UnknownSyscall = -1,
            InvalidParameters = -2,
            OutOfMemory = -3,

            $($variants)*
        }

        impl crate::sugar::Error for $name {}
    };
}

#[allow(dead_code)]
pub const GENERAL_ERROR_END: isize = i16::MIN as isize;
Error_output_enum!(GeneralError {});

impl Into<Rets> for GeneralError {
    fn into(self) -> Rets {
        let r: Result<(), Self> = Err(self);
        match r.convert_into() {
            Some(r) => r,
            None => unreachable!(),
        }
    }
}

macro_rules! Error {
    ($name:ident { $($variant:ident,)* }) => { Error_assign!(crate::sugar::GENERAL_ERROR_END; $name { $($variant,)* } => {}); };
    ($name:ident { $($variant:ident),* }) => { Error!($name { $($variant,)* }); };
}
macro_rules! Error_assign {
    ($i:expr; $name:ident { $in_variant:ident, $($in_variants_rest:tt)* } => { $($out_variants:tt)* }) => {
        Error_assign!($i - 1; $name { $($in_variants_rest)* } => { $($out_variants)* $in_variant = $i, });
    };
    ($i:expr; $name:ident {} => $variants:tt) => { Error_output_enum!($name $variants); };
}

macro_rules! Syscall {
    ([$number:expr] $name:ident ($arg:ty) -> $ret:ty) => {
        pub enum $name {}
        impl crate::Syscall for $name {
            const NUMBER: usize = $number;

            type Arg = $arg;
            type Ret = $ret;
        }
    };
}

impl From<usize> for Object {
    fn from(x: usize) -> Object {
        Object(x)
    }
}

impl Into<usize> for Object {
    fn into(self) -> usize {
        self.0
    }
}

macro_rules! Handle {
    () => {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        #[repr(transparent)]
        pub struct Handle(pub crate::Object);

        impl From<crate::Object> for Handle {
            fn from(x: crate::Object) -> Handle {
                Handle(x)
            }
        }

        impl Into<crate::Object> for Handle {
            fn into(self) -> crate::Object {
                self.0
            }
        }

        impl From<usize> for Handle {
            fn from(x: usize) -> Handle {
                Handle(crate::Object(x))
            }
        }

        impl Into<usize> for Handle {
            fn into(self) -> usize {
                (self.0).0
            }
        }
    };
}
