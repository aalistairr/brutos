use core::convert::TryInto;

pub fn debug_print_char(arg: usize) -> usize {
    match arg.try_into().ok().and_then(core::char::from_u32) {
        None => 1,
        Some(c) => {
            print!("{}", c);
            0
        }
    }
}
