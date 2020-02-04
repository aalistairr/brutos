use core::pin::Pin;

use brutos_memory::VirtAddr;
use brutos_platform_pc as pc;
use brutos_platform_pc::interrupt::idt::{Descriptor, Idt, Type};
use brutos_task::arch::GDT_CODE_KERN;

pub mod entry;

macro_rules! alias {
    ($to:ident: $($from:ident),*) => {
        global_asm!(concat!(
            $(".global int_handler_", stringify!($from), "\n", )*
            $("int_handler_", stringify!($from), ":\n", )*
            "jmp int_handler_", stringify!($to)
        ));
    }
}

alias! { panic:
    nmi,
    general_protection,
    alignment_check,
    simd_error,
    fp_error,
    divide_error,
    breakpoint,
    debug_exception,
    double_fault,
    machine_check,
    virtualization_exception,
    control_protection_exception,
    coprocessor_segment_overrun
}

alias! { kill:
    overflow,
    bound_range_exceeded,
    invalid_opcode,
    no_math_coprocessor,
    invalid_tss,
    segment_not_present,
    stack_segment_fault
}

#[repr(C)]
pub struct InterruptStackFrame {
    rip: usize,
    cs: u16,
    _padding: [u8; 6],
    rflags: usize,
    rsp: usize,
    ss: usize,
}

#[export_name = "int_handler_panic"]
pub extern "C" fn panic(vector: usize, stack_frame: &InterruptStackFrame, error: usize) {
    panic!(
        "don't know how to handle interrupt (vector={}, cs={:#x}, %rip={:#x}, error={:#x})",
        vector, stack_frame.cs, stack_frame.rip, error
    );
}

#[export_name = "int_handler_kill"]
pub extern "C" fn kill(vector: usize, stack_frame: &InterruptStackFrame, error: usize) {
    if stack_frame.cs == GDT_CODE_KERN {
        panic!(
            "fatal exception in kernel (vector={}, %rip={:#x}, error={:#x})",
            vector, stack_frame.rip, error
        );
    }
    unimplemented!()
}

fn cr2() -> VirtAddr {
    let addr: usize;
    unsafe {
        asm!("mov %cr2, $0" : "=r" (addr) ::: "volatile");
    }
    VirtAddr(addr)
}

#[export_name = "int_handler_page_fault"]
pub extern "C" fn page_fault(_vector: usize, stack_frame: &InterruptStackFrame, error: usize) {
    let fault_addr = cr2();
    let critical_count = unsafe { brutos_task::arch::current_task_get_critical_count() };
    if critical_count > 0 {
        panic!(
            "page fault in a critical section (%rip={:#x}, fault addr={:?}, error={:#x})",
            stack_frame.rip, fault_addr, error
        );
    }
    unsafe {
        pc::interrupt::sti();
    }
    unimplemented!()
}

#[export_name = "int_handler_any"]
pub extern "C" fn any(_vector: usize, _stack_frame: &InterruptStackFrame, _error: usize) {
    unimplemented!()
}

unsafe fn idt_mut() -> Pin<&'static mut Idt> {
    static mut IDT: Idt = Idt::new();
    Pin::new_unchecked(&mut IDT)
}

pub unsafe fn initialize() {
    pc::interrupt::disable_pic();

    let mut idt = idt_mut();
    for i in 0..256 {
        idt[i] = Descriptor::new()
            .with_offset(self::entry::ENTRY_FUNCTIONS[i] as usize)
            .with_segment(brutos_task::arch::GDT_CODE_KERN)
            .with_ty(Type::Interrupt)
            .with_present(true);
    }

    Idt::load(idt.as_ref());

    enable();
}

pub unsafe fn enable() {
    pc::interrupt::sti();
}
