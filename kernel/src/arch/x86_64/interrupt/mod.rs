use core::pin::Pin;

use brutos_platform_pc as pc;
use brutos_platform_pc::interrupt::idt::{Descriptor, Idt, Type};

pub mod handlers;

use self::handlers::vectors::*;

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

#[export_name = "int_handler_panic"]
pub extern "C" fn panic(vector: usize, cs: usize, error: usize) {
    panic!(
        "don't know how to handle interrupt (vector={}, cs={:#x}, error={:#x})",
        vector, cs, error
    );
}

#[export_name = "int_handler_kill"]
pub extern "C" fn kill(_vector: usize, _cs: usize, _error: usize) {
    unimplemented!()
}

#[export_name = "int_handler_page_fault"]
pub extern "C" fn page_fault(_vector: usize, _cs: usize, _error: usize) {
    unimplemented!()
}

#[export_name = "int_handler_any"]
pub extern "C" fn any(_vector: usize, _cs: usize, _error: usize) {
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
            .with_offset(self::handlers::HANDLERS[i] as usize)
            .with_segment(brutos_task::arch::GDT_CODE_KERN)
            .with_ty(Type::Interrupt)
            .with_present(true);
    }
    idt[PAGE_FAULT].set_ty(Type::Trap);

    Idt::load(idt.as_ref());

    sti();
}

pub unsafe fn cli() {
    asm!("cli" :::: "volatile");
}

pub unsafe fn sti() {
    asm!("sti" :::: "volatile");
}
