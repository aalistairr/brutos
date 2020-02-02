use crate::io::{inb, outb};

pub mod apic;
pub mod idt;

pub unsafe fn disable_pic() {
    loop {
        // ICW1
        outb(0x20, 0x11);
        outb(0xa0, 0x11);
        // ICW2
        outb(0x21, 0x20);
        outb(0xa1, 0x20);
        // ICW3
        outb(0x21, 0x04);
        outb(0xa1, 0x02);
        // ICW4
        outb(0x21, 0x01);
        outb(0xa1, 0x01);

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

pub unsafe fn cli() {
    asm!("cli" :::: "volatile");
}

pub unsafe fn sti() {
    asm!("sti" :::: "volatile");
}
