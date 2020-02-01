#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    asm!("outb $1, $0" :: "{dx}" (port), "{al}" (value) :: "volatile");
}
