#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![feature(asm)]

pub mod arch;
#[macro_use]
pub mod sugar;

pub use sugar::*;

pub mod addr_space {
    Handle!();

    Syscall!([0x1000] Create(()) -> Result<Handle, CreateError>);
    Error!(CreateError {});

    Syscall!([0x1001] Destroy(Handle) -> Result<(), DestroyError>);
    Error!(DestroyError {});

    pub mod mapping {
        use crate::addr_space;

        use bitbash::bitfield;

        use brutos_memory_units::VirtAddr;

        Handle!();

        Syscall!([0x1100] Create(CreateArgs) -> Result<VirtAddr, CreateError>);
        pub struct CreateArgs {
            pub addr_space: addr_space::Handle,
            pub size: usize,
            pub at: usize,
            pub flags: CreateFlags,
        }
        bitfield! {
            pub struct CreateFlags(pub usize);

            pub new();
            derive_debug;

            pub field read: bool = [0];
            pub field write: bool = [1];
            pub field execute: bool = [2];
            pub field fixed: bool = [3];
            pub field guard: bool = [4];
        }
        Error!(CreateError {
            OutsideSpaceRange,
            OutOfSpace,
        });

        Syscall!([0x1101] Destroy(VirtAddr) -> Result<(), DestroyError>);
        Error!(DestroyError { NotStartOfMapping });
    }
}

pub mod debug {
    Syscall!([0xfffffffffffff000] PrintChar(u32) -> Result<(), PrintCharError>);
    Error!(PrintCharError {});
}

pub trait Syscall {
    const NUMBER: usize;

    type Arg: Convert<arch::Args>;
    type Ret: Convert<arch::Rets>;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Object(pub usize);

pub unsafe fn syscall<S: Syscall>(arg: S::Arg) -> S::Ret {
    let rets = arch::perform_syscall(S::NUMBER, arg.convert_into().expect("syscall: invalid arg"));
    S::Ret::convert_from(rets).expect("syscall: invalid rets")
}
