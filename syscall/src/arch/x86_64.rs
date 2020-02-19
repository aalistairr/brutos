use core::convert::{TryFrom, TryInto};
use core::isize;

use bitbash::ConvertRepr;

use crate::{Convert, Error, Object};

pub fn perform_syscall(number: usize, args: Args) -> Rets {
    unsafe {
        let Args(a1, a2, a3, a4, a5): Args = args;
        let Rets(r1, r2, r3, r4, r5, r6): Rets;
        asm!("syscall"
            : "={rdi}" (r1), "={rsi}" (r2), "={rdx}" (r3), "={r10}" (r4), "={r8}" (r5), "={r9}" (r6)
            : "{rdi}" (number), "{rsi}" (a1), "{rdx}" (a2), "{r10}" (a3), "{r8}" (a4), "{r9}" (a5)
            : "memory", "rcx", "r11", "rax", "r12", "r13", "r14", "r15"
            : "volatile");
        Rets(r1, r2, r3, r4, r5, r6)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Args(pub usize, pub usize, pub usize, pub usize, pub usize);
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Rets(
    pub isize,
    pub usize,
    pub usize,
    pub usize,
    pub usize,
    pub usize,
);
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct PosRets(
    pub usize,
    pub usize,
    pub usize,
    pub usize,
    pub usize,
    pub usize,
);

impl Convert<Args> for () {
    fn convert_from(_: Args) -> Option<()> {
        Some(())
    }

    fn convert_into(self) -> Option<Args> {
        Some(Args::default())
    }
}

impl Convert<Rets> for () {
    fn convert_from(_: Rets) -> Option<()> {
        Some(())
    }

    fn convert_into(self) -> Option<Rets> {
        Some(Rets::default())
    }
}

impl Convert<PosRets> for () {
    fn convert_from(_: PosRets) -> Option<()> {
        Some(())
    }

    fn convert_into(self) -> Option<PosRets> {
        Some(PosRets::default())
    }
}

macro_rules! convert_uN {
    ($($t:ident),*) => {$(
        impl Convert<Args> for $t {
            fn convert_from(Args(a1, _, _, _, _): Args) -> Option<$t> {
                if a1 <= core::$t::MAX as usize {
                    Some(a1 as $t)
                } else {
                    None
                }
            }

            fn convert_into(self) -> Option<Args> {
                Some(Args(self as usize, 0, 0, 0, 0))
            }
        }

        impl Convert<Rets> for $t {
            fn convert_from(Rets(r1, _, _, _, _, _): Rets) -> Option<$t> {
                if r1 as usize <= core::$t::MAX as usize {
                    Some(r1 as usize as $t)
                } else {
                    None
                }
            }

            fn convert_into(self) -> Option<Rets> {
                Some(Rets(self as usize as isize, 0, 0, 0, 0, 0))
            }
        }

        impl Convert<PosRets> for $t {
            fn convert_from(PosRets(r1, _, _, _, _, _): PosRets) -> Option<$t> {
                if r1 <= core::$t::MAX as usize {
                    Some(r1 as $t)
                } else {
                    None
                }
            }

            fn convert_into(self) -> Option<PosRets> {
                Some(PosRets(self as usize, 0, 0, 0, 0, 0))
            }
        }
    )*};
}

convert_uN!(u8, u16, u32, u64, usize);

impl<T, E> Convert<Rets> for Result<T, E>
where
    T: Convert<PosRets>,
    E: Error + ConvertRepr<Repr = isize>,
{
    fn convert_from(Rets(r1, r2, r3, r4, r5, r6): Rets) -> Option<Result<T, E>> {
        if r1 >= 0 {
            T::convert_from(PosRets(r1 as usize, r2, r3, r4, r5, r6)).map(Ok)
        } else {
            E::try_from_repr(r1).map(Err)
        }
    }

    fn convert_into(self) -> Option<Rets> {
        match self {
            Ok(x) => match x.convert_into() {
                Some(PosRets(r1, r2, r3, r4, r5, r6)) if r1 <= isize::MAX as usize => {
                    Some(Rets(r1 as isize, r2, r3, r4, r5, r6))
                }
                _ => None,
            },
            Err(e) => match e.into_repr() {
                e if e < 0 => Some(Rets(e, 0, 0, 0, 0, 0)),
                _ => None,
            },
        }
    }
}

impl<T: From<Object> + Into<Object>> Convert<Args> for T {
    fn convert_from(Args(a1, _, _, _, _): Args) -> Option<T> {
        Object::try_from(a1).map(Into::into).ok()
    }

    fn convert_into(self) -> Option<Args> {
        Some(Args(self.into().0, 0, 0, 0, 0))
    }
}

impl<T: From<Object> + Into<Object>> Convert<PosRets> for T {
    fn convert_from(PosRets(r1, _, _, _, _, _): PosRets) -> Option<T> {
        Object::try_from(r1).map(Into::into).ok()
    }

    fn convert_into(self) -> Option<PosRets> {
        Some(PosRets(self.into().0, 0, 0, 0, 0, 0))
    }
}

use brutos_memory_units::VirtAddr;

impl Convert<Args> for VirtAddr {
    fn convert_from(Args(a1, _, _, _, _): Args) -> Option<VirtAddr> {
        Some(VirtAddr(a1))
    }

    fn convert_into(self) -> Option<Args> {
        Some(Args(self.0, 0, 0, 0, 0))
    }
}

impl Convert<Rets> for VirtAddr {
    fn convert_from(Rets(r1, _, _, _, _, _): Rets) -> Option<VirtAddr> {
        Some(VirtAddr(r1 as usize))
    }

    fn convert_into(self) -> Option<Rets> {
        Some(Rets(self.0 as isize, 0, 0, 0, 0, 0))
    }
}

impl Convert<PosRets> for VirtAddr {
    fn convert_from(PosRets(r1, _, _, _, _, _): PosRets) -> Option<VirtAddr> {
        Some(VirtAddr(r1))
    }

    fn convert_into(self) -> Option<PosRets> {
        Some(PosRets(self.0, 0, 0, 0, 0, 0))
    }
}

use crate::addr_space::mapping::{
    CreateArgs as CreateMappingArgs, CreateFlags as CreateMappingFlags,
};

impl Convert<Args> for CreateMappingArgs {
    fn convert_from(Args(a1, a2, a3, a4, _): Args) -> Option<CreateMappingArgs> {
        Some(CreateMappingArgs {
            addr_space: a1.try_into().ok()?,
            size: a2,
            at: a3,
            flags: CreateMappingFlags(a4),
        })
    }

    fn convert_into(self) -> Option<Args> {
        Some(Args(
            (self.addr_space.0).0,
            self.size,
            self.at,
            self.flags.0,
            0,
        ))
    }
}
