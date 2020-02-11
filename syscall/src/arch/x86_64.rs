pub fn perform_syscall(
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> (usize, usize, usize, usize, usize, usize, usize) {
    unsafe {
        let (r0, r1, r2, r3, r4, r5, r6): (usize, usize, usize, usize, usize, usize, usize);
        asm!("syscall"
            : "={rax}" (r0), "={rdi}" (r1), "={rsi}" (r2), "={rdx}" (r3), "={r10}" (r4), "={r8}" (r5), "={r9}" (r6)
            : "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3), "{r10}" (arg4), "{r8}" (arg5), "{r9}" (arg6)
            : "memory", "rcx", "r11"
            : "volatile");
        (r0, r1, r2, r3, r4, r5, r6)
    }
}
