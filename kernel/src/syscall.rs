use core::char;

use brutos_syscall as sc;
use brutos_syscall::GeneralError;

macro_rules! handle {
    ($($path:path => $f:ident,)*) => {
        pub fn handle(number: usize, args: brutos_syscall::arch::Args) -> brutos_syscall::arch::Rets {
            use brutos_syscall::{Convert, Syscall};
            match number {
                $(
                    <$path>::NUMBER => {
                        let arg = match <<$path as brutos_syscall::Syscall>::Arg>::convert_from(args) {
                            Some(arg) => arg,
                            None => return GeneralError::InvalidParameters.into(),
                        };
                        let ret: Result<<$path as Syscall>::RetOk, <$path as Syscall>::RetErr> = $f(arg);
                        ret.convert_into().expect("syscall: invalid ret")
                    }
                )*
                _ => return GeneralError::UnknownSyscall.into(),
            }
        }
    };
    ($($path:expr => $f:ident),*) => { handle!($($path => $f,)*); }
}

handle! {
    sc::debug::PrintChar => debug_print_char,
}

pub fn debug_print_char(c: u32) -> Result<(), sc::debug::PrintCharError> {
    let c = char::from_u32(c).ok_or(sc::debug::PrintCharError::InvalidParameters)?;
    print!("{}", c);
    Ok(())
}
