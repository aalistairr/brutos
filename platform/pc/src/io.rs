#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    asm!("outb $1, $0" :: "{dx}" (port), "{al}" (value) :: "volatile");
}

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("inb $1" : "={al}" (value) : "{dx}" (port) :: "volatile");
    value
}
