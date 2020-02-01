use crate::io::outb;

pub mod apic;
pub mod idt;

pub unsafe fn disable_pic() {
    outb(0xa1, 0xff);
    outb(0x21, 0xff);
}
