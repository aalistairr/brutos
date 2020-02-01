use core::pin::Pin;
use core::sync::atomic::AtomicBool;

use brutos_alloc::Arc;
use brutos_memory::VirtAddr;

use crate::{Context, EntryPoint, State, Task};

#[inline]
pub unsafe fn current_task_inc_critical_count() {
    asm!("incq %gs:0xc0" ::: "cc", "memory" : "volatile");
}

/// Returns true if the decremented count is zero
#[inline]
pub unsafe fn current_task_dec_critical_count() -> bool {
    let is_zero: bool;
    asm!("decq %gs:0xc0" : "={@ccz}" (is_zero) :: "cc", "memory" : "volatile");
    is_zero
}

impl<Cx: Context> Task<Cx> {
    #[inline]
    pub unsafe fn current_task() -> *const Task<Cx> {
        let task: *const Task<Cx>;
        asm!("mov %gs:0xc8, $0" : "=r" (task) ::::);
        task
    }
}

impl<Cx: Context> State<Cx> {
    pub unsafe fn activate(this: Pin<&mut Self>) {
        let this = Pin::into_inner_unchecked(this) as *mut Self;
        asm!("wrgsbase $0" :: "r" (this) :: "volatile");
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Regs {
    rax: u64,         // 0x00
    rbx: u64,         // 0x08
    rcx: u64,         // 0x10
    rdx: u64,         // 0x18
    rdi: u64,         // 0x20
    rsi: u64,         // 0x28
    rbp: u64,         // 0x30
    rsp: u64,         // 0x38
    r8: u64,          // 0x40
    r9: u64,          // 0x48
    r10: u64,         // 0x50
    r11: u64,         // 0x58
    r12: u64,         // 0x60
    r13: u64,         // 0x68
    r14: u64,         // 0x70
    r15: u64,         // 0x78
    rip: u64,         // 0x80
    rflags: u64,      // 0x88
    fs_base: u64,     // 0x90
    gs_base: u64,     // 0x98
    gs_base_alt: u64, // 0xa0
    cs: u16,          // 0xa8
    ds: u16,          // 0xaa
    ss: u16,          // 0xac
    es: u16,          // 0xae
    fs: u16,          // 0xb0
    gs: u16,          // 0xb2
    _padding0: u16,   // 0xb4
    _padding1: u16,   // 0xb6
                      // 0xb8
}

impl Regs {
    pub fn initialize<Cx: Context>(
        &mut self,
        state: *const State<Cx>,
        _: &Pin<Arc<Task<Cx>, Cx>>,
        entry_point: EntryPoint,
        kernel_stack: VirtAddr,
    ) {
        match entry_point {
            EntryPoint::Kernel(entry_point, data) => {
                self.rip = entry_point.0 as u64;
                self.rsp = kernel_stack.0 as u64;
                self.rdi = data as u64;

                self.gs_base = state as *const _ as usize as u64;
                self.gs_base_alt = state as *const _ as usize as u64;

                self.cs = GDT_CODE_KERN;
                self.ds = GDT_DATA_KERN;
                self.ss = GDT_DATA_KERN;
                self.es = GDT_DATA_KERN;
                self.fs = GDT_DATA_KERN;
                self.gs = GDT_DATA_KERN;
            }
            #[cfg(feature = "user-mode")]
            EntryPoint::User(entry_point, data) => {
                self.rip = entry_point.0 as u64;
                self.rdi = data as u64;

                self.gs_base_alt = state as *const _ as usize as u64;

                self.cs = GDT_CODE_USER;
                self.ds = GDT_DATA_USER;
                self.ss = GDT_DATA_USER;
                self.es = GDT_DATA_USER;
                self.fs = GDT_DATA_USER;
                self.gs = GDT_DATA_USER;
            }
        }
    }
}

pub unsafe fn switch<Cx: Context>(switch_lock: &AtomicBool, to: *mut State<Cx>) {
    asm!("
        // Save state
            mov %rax, %gs:0x00
            mov %rbx, %gs:0x08
            mov %rcx, %gs:0x10
            mov %rdx, %gs:0x18
            mov %rdi, %gs:0x20
            mov %rsi, %gs:0x28
            mov %rbp, %gs:0x30
            mov %rsp, %gs:0x38
            mov  %r8, %gs:0x40
            mov  %r9, %gs:0x48
            mov %r10, %gs:0x50
            mov %r11, %gs:0x58
            mov %r12, %gs:0x60
            mov %r13, %gs:0x68
            mov %r14, %gs:0x70
            mov %r15, %gs:0x78

            // rip
            movq $$1f, %gs:0x80

            // rflags
            pushfq
            popq %gs:0x88

            // fs_base
            rdfsbase %rdi
            mov %rdi, %gs:0x90

            // gs_base
            rdgsbase %rdi
            mov %rdi, %gs:0x98

            // gs_base_alt
            swapgs
            rdgsbase %rdi
            swapgs
            mov %rdi, %gs:0xa0

            mov %gs:0x20, %rdi

            mov %cs, %gs:0xa8
            mov %ds, %gs:0xaa
            mov %ss, %gs:0xac
            mov %es, %gs:0xae
            mov %fs, %gs:0xb0
            mov %gs, %gs:0xb2

            // done switching
            mov $$0, %rsp
            movb $$0, (%rax)


        // Set up stack
            cmpw $$0x8, 0xa8(%rdi)      // cs == 0x8
            mov 0x38(%rdi), %rsp        // use saved stack
            cmovne 0xb8(%rdi), %rsp     // use kernel stack

        // Set up iret parameters
            pushq 0xac(%rdi)            // ss
            pushq 0x38(%rdi)            // rsp
            pushq 0x88(%rdi)            // rflags
            pushq 0xa8(%rdi)            // cs
            pushq 0x80(%rdi)            // rip
        
        // Load state
            mov 0x00(%rdi), %rax
            mov 0x08(%rdi), %rbx
            mov 0x10(%rdi), %rcx
            mov 0x18(%rdi), %rdx
            // skip %rdi for now
            // skip %rsi for now
            mov 0x30(%rdi), %rbp
            // skip %rsp
            mov 0x40(%rdi), %r8
            mov 0x48(%rdi), %r9
            mov 0x50(%rdi), %r10
            mov 0x58(%rdi), %r11
            mov 0x60(%rdi), %r12
            mov 0x68(%rdi), %r13
            mov 0x70(%rdi), %r14
            mov 0x78(%rdi), %r15

            mov 0x90(%rdi), %rsi
            wrfsbase %rsi
            mov 0xa0(%rdi), %rsi
            wrgsbase %rsi
            swapgs
            mov 0x98(%rdi), %rsi
            wrgsbase %rsi

            mov 0xaa(%rdi), %ds
            mov 0xae(%rdi), %es
            mov 0xb0(%rdi), %fs
            mov 0xb2(%rdi), %gs

            mov 0x28(%rdi), %rsi
            mov 0x20(%rdi), %rdi

            iretq
        1:
        "
        :
        : "{rax}" (switch_lock), "{rdi}" (to)
        : "memory"
        : "volatile");

    Cx::leave_critical();
}

global_asm!(
    "
    .section .rodata
    .align 8
    GDT:
    .quad 0x0                   // 0x00 -> null
    .quad 0x0020980000000000    // 0x08 -> code (kernel)
    .quad 0x0000920000000000    // 0x10 -> data (kernel)
    .quad 0x0020f80000000000    // 0x18 -> code (user)
    .quad 0x0000f20000000000    // 0x20 -> data (user)
    GDT_end:
    
    .byte 0
    .byte 0
    .byte 0
    .byte 0
    .byte 0
    .byte 0
    .global GDTR
    GDTR:
    .short GDT_end - GDT - 1
    .quad GDT
"
);

pub const GDT_NULL: u16 = 0x0;
pub const GDT_CODE_KERN: u16 = 0x8;
pub const GDT_DATA_KERN: u16 = 0x10;
pub const GDT_CODE_USER: u16 = 0x18;
pub const GDT_DATA_USER: u16 = 0x20;

#[inline]
pub unsafe fn load_gdt() {
    asm!("lgdt GDTR" :::: "volatile");
}
