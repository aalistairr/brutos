#![feature(naked_functions)]
#![feature(asm)]
#![no_std]
#![no_main]

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
#[naked]
unsafe fn _start() {
    asm!("1: jmp 1b" :::: "volatile");
}
