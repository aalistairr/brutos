use brutos_platform_pc::msr;
use brutos_syscall as sc;

pub unsafe fn initialize() {
    assert_eq!(
        brutos_task::arch::GDT_CODE_KERN + 8,
        brutos_task::arch::GDT_DATA_KERN
    );
    assert_eq!(
        brutos_task::arch::GDT_CODE_USER - 8,
        brutos_task::arch::GDT_DATA_USER
    );
    msr::map::<msr::Ia32Efer, _>(|efer| efer.with_syscall_enabled(true));
    msr::write::<msr::Ia32Star>(
        msr::Star::new()
            .with_kernel_selector(brutos_task::arch::GDT_CODE_KERN)
            .with_user_selector(brutos_task::arch::GDT_CODE_USER - 16),
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
    syscall_unswapped_gs_prefix_start:
        swapgs
    syscall_unswapped_gs_prefix_end:
        mov %rsp, %rax
        mov %gs:0xb8, %rsp
        sti

        push %rax
        push %rbp
        push %rcx
        push %r11

        mov %r10, %rcx
        call syscall_entry_rust
    " :::: "volatile");
}

global_asm!(
    "
.section .text
.global syscall_return
.global syscall_unswapped_gs_postfix_start
.global syscall_unswapped_gs_postfix_end
syscall_return:
    cli

    mov %rcx, %r10

    add $8, %rsp
    pop %r11
    pop %rcx
    pop %rbp
    pop %rsp

    mov $0, %rax
    mov $0, %rbx
    mov $0, %r12
    mov $0, %r13
    mov $0, %r14
    mov $0, %r15
    mov $0, %rbp

syscall_unswapped_gs_postfix_start:
    swapgs
    sysretq
syscall_unswapped_gs_postfix_end:
"
);

extern "C" {
    fn syscall_return(
        r1: usize,
        r2: usize,
        r3: usize,
        r4: usize,
        r5: usize,
        r6: usize,
        r11: usize,
        rcx: usize,
        rbp: usize,
        rsp: usize,
    ) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn syscall_entry_rust(
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
    r11: usize,
    rcx: usize,
    rbp: usize,
    rsp: usize,
) -> ! {
    let (r1, r2, r3, r4, r5, r6) = handle_syscall(arg1, arg2, arg3, arg4, arg5, arg6);
    syscall_return(r1, r2, r3, r4, r5, r6, r11, rcx, rbp, rsp);
}

pub fn handle_syscall(
    arg1: usize,
    arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
    _arg6: usize,
) -> (usize, usize, usize, usize, usize, usize) {
    let r1;
    let (r2, r3, r4, r5, r6) = (0, 0, 0, 0, 0);
    match arg1 {
        sc::DEBUG_PRINT_CHAR => r1 = crate::syscall::debug_print_char(arg2),
        _ => r1 = !0,
    }
    (r1, r2, r3, r4, r5, r6)
}
