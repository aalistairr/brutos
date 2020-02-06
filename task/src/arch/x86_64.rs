use core::pin::Pin;
use core::sync::atomic::{fence, AtomicBool, Ordering};

use brutos_alloc::Arc;
use brutos_memory::VirtAddr;
use brutos_platform_pc::{
    gdt,
    tss::{self, Tss},
};

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

#[inline]
pub unsafe fn current_task_get_critical_count() -> usize {
    let count: usize;
    asm!("mov %gs:0xc0, $0" : "=r" (count));
    count
}

impl<Cx: Context> Task<Cx> {
    #[inline]
    pub unsafe fn current_task_ptr() -> *const Task<Cx> {
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
    pub rax: u64,         // 0x00
    pub rbx: u64,         // 0x08
    pub rcx: u64,         // 0x10
    pub rdx: u64,         // 0x18
    pub rdi: u64,         // 0x20
    pub rsi: u64,         // 0x28
    pub rbp: u64,         // 0x30
    pub rsp: u64,         // 0x38
    pub r8: u64,          // 0x40
    pub r9: u64,          // 0x48
    pub r10: u64,         // 0x50
    pub r11: u64,         // 0x58
    pub r12: u64,         // 0x60
    pub r13: u64,         // 0x68
    pub r14: u64,         // 0x70
    pub r15: u64,         // 0x78
    pub rip: u64,         // 0x80
    pub rflags: u64,      // 0x88
    pub fs_base: u64,     // 0x90
    pub gs_base: u64,     // 0x98
    pub gs_base_alt: u64, // 0xa0
    pub cs: u16,          // 0xa8
    pub ds: u16,          // 0xaa
    pub ss: u16,          // 0xac
    pub es: u16,          // 0xae
    pub fs: u16,          // 0xb0
    pub gs: u16,          // 0xb2
    _padding0: u16,       // 0xb4
    _padding1: u16,       // 0xb6
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
            EntryPoint::Kernel(entry_point, data0, data1) => {
                self.rip = entry_point.0 as u64;
                self.rsp = kernel_stack.0 as u64;
                self.rdi = data0 as u64;
                self.rsi = data1 as u64;

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
    tss_mut().rsp[0] = (*to).kernel_stack.0 as u64;
    fence(Ordering::SeqCst);
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

            movq $$0, -0x8(%rsp)

        // Set up iret parameters
            sub $$0x6, %rsp
            pushw 0xac(%rdi)            // ss
            pushq 0x38(%rdi)            // rsp
            pushq 0x88(%rdi)            // rflags
            sub $$0x6, %rsp
            pushw 0xa8(%rdi)            // cs
            pushq 0x80(%rdi)            // rip

            pushq 0x20(%rdi)            // %rdi
            pushq 0x28(%rdi)            // %rsi
        
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

            mov 0xae(%rdi), %es
            mov 0xb0(%rdi), %fs
            mov 0xb2(%rdi), %gs

            mov 0x90(%rdi), %rsi
            wrfsbase %rsi
            mov 0xa0(%rdi), %rsi
            wrgsbase %rsi
            swapgs
            mov 0x98(%rdi), %rsi
            wrgsbase %rsi

            mov 0xaa(%rdi), %ds

            pop %rsi
            pop %rdi

            iretq
        1:
        "
        :
        : "{rax}" (switch_lock), "{rdi}" (to)
        : "cc", "memory"
        : "volatile");
    Cx::leave_critical();
}

pub const GDT_NULL: u16 = 0x0;
pub const GDT_CODE_KERN: u16 = 0x8;
pub const GDT_DATA_KERN: u16 = 0x10;
pub const GDT_CODE_USER: u16 = 0x18;
pub const GDT_DATA_USER: u16 = 0x20;
pub const GDT_TSS: u16 = 0x28;

#[repr(C, align(8))]
pub struct Gdt {
    null: u64,
    code_kern: gdt::CDDescriptor,
    data_kern: gdt::CDDescriptor,
    code_user: gdt::CDDescriptor,
    data_user: gdt::CDDescriptor,
    tss: tss::Descriptor,
}

unsafe fn gdt_mut() -> Pin<&'static mut Gdt> {
    static mut GDT: Gdt = Gdt {
        null: 0,
        code_kern: gdt::CDDescriptor::new(gdt::CDType::Code)
            .with_present(true)
            .with_long(true)
            .with_dpl(0),
        data_kern: gdt::CDDescriptor::new(gdt::CDType::Data)
            .with_present(true)
            .with_long(true)
            .with_dpl(0),
        code_user: gdt::CDDescriptor::new(gdt::CDType::Code)
            .with_present(true)
            .with_long(true)
            .with_dpl(3),
        data_user: gdt::CDDescriptor::new(gdt::CDType::Data)
            .with_present(true)
            .with_long(true)
            .with_dpl(3),
        tss: tss::Descriptor::new(tss::Type::TssAvailable).with_present(false),
    };
    Pin::new_unchecked(&mut GDT)
}

pub unsafe fn tss_mut() -> Pin<&'static mut Tss> {
    static mut TSS: tss::Tss = Tss::new();
    Pin::new_unchecked(&mut TSS)
}

pub unsafe fn initialize_and_load_gdt() {
    gdt_mut().tss = tss::Descriptor::new(tss::Type::TssAvailable)
        .with_present(true)
        .with_dpl(3)
        .with_tss_size(core::mem::size_of::<tss::Tss>())
        .with_tss(Some(tss_mut().get_mut().into()));
    gdt::load(gdt_mut().as_ref());
    tss::load(GDT_TSS);
}
