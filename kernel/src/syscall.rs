use core::char;

use brutos_syscall as sc;

macro_rules! handle {
    ($($path:path => $f:ident,)*) => {
        pub fn handle(number: usize, args: brutos_syscall::arch::Args) -> brutos_syscall::arch::Rets {
            use brutos_syscall::{Convert, Syscall};
            match number {
                $(
                    <$path>::NUMBER => {
                        let arg: <$path as Syscall>::Arg = match <_>::convert_from(args) {
                            Some(arg) => arg,
                            None => {
                                let e: Result<(), sc::GeneralError> = Err(sc::GeneralError::InvalidParameters);
                                return e.convert_into().expect("syscall: invalid ret");
                            }
                        };
                        let ret: <$path as Syscall>::Ret = $f(arg);
                        ret.convert_into().expect("syscall: invalid ret")
                    }
                )*
                _ => {
                    let e: Result<(), sc::GeneralError> = Err(sc::GeneralError::UnknownSyscall);
                    e.convert_into().expect("syscall: invalid ret")
                }
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
