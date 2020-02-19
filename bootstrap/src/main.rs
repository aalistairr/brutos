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
    asm!("
    0:
        mov $$0, %r12
    1:
        mov $$0xfffffffffffff000, %rdi
        mov $$0x41, %rsi
        add %r12, %rsi
        syscall
        inc %r12
        cmp $$25, %r12
        jne 1b
        jmp 0b
    " :::: "volatile");
}
