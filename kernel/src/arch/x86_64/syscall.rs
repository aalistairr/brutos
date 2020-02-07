use brutos_platform_pc::msr;

pub unsafe fn initialize() {
    msr::write::<msr::Ia32Star>(
        msr::Star::new()
            .with_kernel_selector(brutos_task::arch::GDT_CODE_KERN)
            .with_user_selector(brutos_task::arch::GDT_CODE_USER),
    );
    msr::write::<msr::Ia32LStar>(syscall_entry as u64);
    msr::write::<msr::Ia32FMask>(!0);
}

// registers:
//   %rcx       return RIP      (                 clobbered in ABI)
//   %r11       return RFLAGS   (                 clobbered in ABI)
//
//   %rdi       arg 1           (enforced by ABI, clobbered)
//   %rsi       arg 2           (enforced by ABI, clobbered)
//   %rdx       arg 3           (enforced by ABI, clobbered)
//   %r10       arg 4           (                 clobbered, %rcx is arg 4 in ABI)
//   %r8        arg 5           (enforced by ABI, clobbered)
//   %r9        arg 6           (enforced by ABI, clobbered)
//
//   %rbx       saved           (enforced by ABI)
//   %rbp       saved           (enforced by ABI)
//   %rsp       saved
//   %r12       saved           (enforced by ABI)
//   %r13       saved           (enforced by ABI)
//   %r14       saved           (enforced by ABI)
//   %r15       saved           (enforced by ABI)
//
//   %rax       return value    (enforced in ABI)

#[no_mangle]
#[naked]
pub unsafe fn syscall_entry() {
    // %rcx contains the return RIP
    // %r11 contains the return RFLAGS
    asm!("
    .global syscall_unswapped_gs_prefix_start
    .global syscall_unswapped_gs_prefix_end
    .global syscall_unswapped_gs_postfix_start
    .global syscall_unswapped_gs_postfix_end
    syscall_unswapped_gs_prefix_start:
        swapgs
    syscall_unswapped_gs_prefix_end:
        mov %rsp, %rax
        mov %gs:0xb8, %rsp
        sti

        sub $$0x8, %rsp
        push %rax
        push %rcx
        push %r11

        mov %r10, %rcx
        call handle_syscall

        pop %r11
        pop %rcx
        pop %r10

        cli
        mov %r10, %rsp
    syscall_unswapped_gs_postfix_start:
        swapgs
        sysret
    syscall_unswapped_gs_postfix_end:
    " :::: "volatile");
}

#[no_mangle]
pub extern "C" fn handle_syscall(
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    unimplemented!()
}

pub fn perform_syscall(
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    unsafe {
        let r: usize;
        asm!("syscall"
            : "={rax}" (r)
            : "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3), "{r10}" (arg4), "{r8}" (arg5), "{r9}" (arg6)
            : "memory", "rcx", "r11", "rdi", "rsi", "rdx", "r10", "r8", "r9"
            : "volatile");
        r
    }
}
