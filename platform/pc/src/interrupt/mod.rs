use crate::io::{inb, outb};

pub mod apic;
pub mod idt;

pub unsafe fn initialize_pic(vector_offset_0: u8, vector_offset_1: u8) {
    assert!(vector_offset_0 % 32 == 0);
    assert!(vector_offset_1 % 32 == 0);

    // ICW1
    outb(0x20, 0x11);
    outb(0xa0, 0x11);
    // ICW2
    outb(0x21, vector_offset_0);
    outb(0xa1, vector_offset_1);
    // ICW3
    outb(0x21, 0x04);
    outb(0xa1, 0x02);
    // ICW4
    outb(0x21, 0x01);
    outb(0xa1, 0x01);
}

pub unsafe fn disable_pic() {
    loop {
        initialize_pic(0x20, 0x20);

        // Set IMR
        outb(0x21, 0xff);
        outb(0xa1, 0xff);

        // OCW3
        outb(0x20, 0xa);
        outb(0xa0, 0xa);

        // Read IRR
        if inb(0x20) == 0 && inb(0xa0) == 0 {
            break;
        }
    }
}

pub mod pit {
    use brutos_util_macros::bitfield;

    use crate::io::outb;

    pub const FREQ: usize = 1193182;

    bitfield! {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub struct Command(u8);
        pub field operating_mode: usize => 1..4;
        pub field access_mode: usize => 4..6;
        pub field channel: usize => 6..8;
    }
    impl Command {
        pub fn new() -> Command {
            Command(0).with_access_mode(0b11)
        }
    }

    pub unsafe fn send_command(command: Command) {
        outb(0x43, command.0)
    }

    pub unsafe fn initialize() {
        send_command(Command::new());
    }

    pub unsafe fn set_reload_value(reload_value: u16) {
        let lo: u8 = (reload_value & 0xff) as u8;
        let hi: u8 = (reload_value >> 8) as u8;
        outb(0x40, lo);
        outb(0x40, hi);
    }
}

pub unsafe fn cli() {
    asm!("cli" :::: "volatile");
}

pub unsafe fn sti() {
    asm!("sti" :::: "volatile");
}
