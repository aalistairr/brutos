use core::cmp::max;
use core::convert::TryInto;
use core::pin::Pin;

use brutos_memory_units::{PhysAddr, VirtAddr};
use brutos_memory_vm::{FaultConditions, PageFaultError};
use brutos_platform_pc as pc;
use brutos_platform_pc::cpuid;
use brutos_platform_pc::interrupt::apic::{self, Apic};
use brutos_platform_pc::interrupt::idt::{Descriptor, Idt, Type};
use brutos_platform_pc::interrupt::InterruptStackFrame;
use brutos_platform_pc::msr;
use brutos_sync::spinlock::Spinlock;
use brutos_task::arch::{tss_mut, GDT_CODE_KERN};
use brutos_util_macros::bitfield;

use crate::Cx;

pub mod entry;
use self::entry::vector;

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

unsafe fn check_task() {
    if !brutos_task::Context::is_task_active(&mut Cx::default(), &*brutos_task::Task::current()) {
        <Cx as brutos_sync::Critical>::enter_critical();
        crate::destroy_task(crate::scheduler().deschedule());
        <Cx as brutos_sync::waitq::Context>::unlock_and_yield(
            &core::sync::atomic::AtomicBool::new(true),
        );
        unreachable!()
    }
}

macro_rules! interrupt_handler {
    ($name:ident => $f:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(vector: usize, stack_frame: &InterruptStackFrame, error: usize) {
            $f(vector, stack_frame, error);
            unsafe {
                check_task();
            }
        }
    };
}

interrupt_handler!(int_handler_panic => panic);
fn panic(vector: usize, stack_frame: &InterruptStackFrame, error: usize) {
    panic!(
        "don't know how to handle interrupt (vector={}, cs={:#x}, %rip={:#x}, error={:#x})",
        vector, stack_frame.cs, stack_frame.rip, error
    );
}

unsafe fn kill_addr_space() {
    match &*brutos_task::Task::<Cx>::current().addr_space.lock() {
        crate::TaskAddrSpace::Active(addr_space) => addr_space.kill(),
        _ => unreachable!(),
    }
}

interrupt_handler!(int_handler_kill => kill);
fn kill(vector: usize, stack_frame: &InterruptStackFrame, error: usize) {
    if stack_frame.cs == GDT_CODE_KERN {
        panic!(
            "fatal exception in kernel (vector={}, %rip={:#x}, error={:#x})",
            vector, stack_frame.rip, error
        );
    }
    unsafe {
        crate::arch::interrupt::unmask();
        kill_addr_space();
    }
}

fn cr2() -> VirtAddr {
    let addr: usize;
    unsafe {
        asm!("mov %cr2, $0" : "=r" (addr) ::: "volatile");
    }
    VirtAddr(addr)
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct PageFaultErrorCode(usize);

    pub field present: bool = [0];
    pub field write: bool = [1];
    pub field user_mode: bool = [2];
    pub field instruction_fetch: bool = [4];
}

interrupt_handler!(int_handler_page_fault => page_fault);
fn page_fault(_vector: usize, stack_frame: &InterruptStackFrame, error: usize) {
    let fault_addr = cr2();
    let critical_count = unsafe { brutos_task::arch::current_task_get_critical_count() };
    if critical_count > 0 {
        panic!(
            "page fault in a critical section (%rip={:#x}, fault addr={:?}, error={:#x})",
            stack_frame.rip, fault_addr, error
        );
    }
    unsafe {
        crate::arch::interrupt::unmask();
    }
    let addr_space = match &*brutos_task::Task::<Cx>::current().addr_space.lock() {
        crate::TaskAddrSpace::Active(addr_space) => addr_space.clone(),
        _ => unreachable!(),
    };
    let error = PageFaultErrorCode(error);
    let fault_conditions = FaultConditions {
        was_present: error.is_present(),
        was_write: error.is_write(),
        was_instruction_fetch: error.is_instruction_fetch(),
        was_user_access: error.is_user_mode(),
    };
    match addr_space.vm().page_fault(fault_addr, fault_conditions) {
        Ok(()) => (),
        Err(PageFaultError::InvalidAccess) => unsafe { kill_addr_space() },
        Err(e) => panic!("error in page fault: {:?}", e),
    }
}

#[export_name = "int_handler_spurious"]
pub extern "C" fn spurious() {}

interrupt_handler!(int_handler_timer => timer);
fn timer(_vector: usize, _stack_frame: &InterruptStackFrame, _error: usize) {
    let critical_count = unsafe { brutos_task::arch::current_task_get_critical_count() };
    if critical_count > 0 {
        panic!("timer interrupt in critical section");
    }
    unsafe {
        crate::yieldd();
    }
}

interrupt_handler!(int_handler_any => any);
fn any(_vector: usize, _stack_frame: &InterruptStackFrame, _error: usize) {
    unimplemented!()
}

unsafe fn idt_mut() -> Pin<&'static mut Idt> {
    static mut IDT: Idt = Idt::new();
    Pin::new_unchecked(&mut IDT)
}

static mut APIC: Option<Spinlock<&'static mut Apic, Cx>> = None;
#[no_mangle]
static mut APIC_EOI: *mut u32 = core::ptr::null_mut();

static mut USE_TSC: bool = false;
static mut TICKS_PER_10NS: usize = 0;
static mut TICKS_PER_1MS: usize = 0;

pub unsafe fn get_apic() -> &'static Spinlock<&'static mut Apic, Cx> {
    APIC.as_ref().unwrap()
}

pub unsafe fn initialize() {
    pc::interrupt::disable_pic();
    Idt::load(idt_mut().as_ref());
    setup_apic();
    setup_idt();
}

const NMI_IST: usize = 1;

pub unsafe fn initialize_with_address_space() {
    use brutos_task::Context;
    let nmi_stack = Cx::default()
        .alloc_stack()
        .expect("failed to allocate NMI stack")
        .0 as u64;
    tss_mut().as_mut().set_ist1(nmi_stack);
}

pub unsafe fn unmask() {
    pc::interrupt::sti();
}

pub unsafe fn mask() {
    pc::interrupt::cli();
}

unsafe fn setup_apic() {
    pc::interrupt::pit::initialize();
    pc::interrupt::disable_pic();

    let apic_addr = msr::read::<msr::Ia32ApicBase>().base();
    let apic_ptr =
        crate::arch::memory::map_phys_ident(PhysAddr(apic_addr), core::mem::size_of::<Apic>())
            .expect("failed to map APIC into memory");
    APIC = Some(Spinlock::new(apic_ptr.cast().as_mut()));

    let mut apic = get_apic().lock();
    APIC_EOI = apic.register_as_mut_ptr::<apic::reg::EndOfInterrupt>();
    apic.write::<apic::reg::SpuriousInterruptVector>(
        apic::SpuriousInterruptVector::new()
            .with_vector(vector::SPURIOUS)
            .with_apic_enabled(true),
    );
    drop(apic);

    let tsc_freq = cpuid::leaf::CoreCrystalClock::get().tsc_freq();
    if cpuid::leaf::InvariantTsc::get().is_available() && tsc_freq.is_some() {
        let tsc_freq = tsc_freq.expect("failed to get tsc frequency");
        USE_TSC = true;
        println!("tsc frequency: {}Hz", tsc_freq);
        TICKS_PER_10NS = tsc_freq / (1000000000 / 10);
        TICKS_PER_1MS = tsc_freq / 1000;

        let mut apic = get_apic().lock();
        apic.write::<apic::reg::LvtTimer>(
            apic::Timer::new()
                .with_vector(vector::TIMER)
                .with_timer_mode(apic::TimerMode::TscDeadline),
        );
        apic.write::<apic::reg::DivideConfiguration>(
            apic::DivideConfiguration::new().with_divide_value(apic::DivideValue::By1),
        );
        drop(apic);
    } else {
        let mut apic = get_apic().lock();
        apic.write::<apic::reg::LvtTimer>(
            apic::Timer::new()
                .with_vector(vector::TIMER)
                .with_timer_mode(apic::TimerMode::OneShot),
        );
        apic.write::<apic::reg::DivideConfiguration>(
            apic::DivideConfiguration::new().with_divide_value(apic::DivideValue::By1),
        );
        drop(apic);

        let bus_freq = determine_bus_freq();
        println!("bus frequency: {}Hz", bus_freq);
        TICKS_PER_10NS = bus_freq / (1000000000 / 10);
        TICKS_PER_1MS = bus_freq / 1000;
    }

    if TICKS_PER_10NS < 10 {
        println!("warning: using a timer with low resolution");
    } else if TICKS_PER_1MS < 1 {
        println!("warning: using a timer with very low resolution");
        TICKS_PER_1MS = 1000;
    }
}

unsafe fn determine_bus_freq() -> usize {
    const TEST_TIME_MS: usize = 10;
    const TEST_COUNT: usize = 10;
    const PIT_VALUE: u16 = (TEST_TIME_MS * pc::interrupt::pit::FREQ / 1000) as u16;

    #[naked]
    unsafe extern "C" fn pit_tick() {
        asm!("
            mov (%rsi), %ebx
            mov $$0x20, %al
            outb %al, $$0x20
            iretq
        " :  ::: "volatile");
    }

    unsafe fn measure_ticks(
        initial_count_reg_ptr: *mut u32,
        current_count_reg_ptr: *mut u32,
    ) -> usize {
        assert!(TEST_COUNT > 0);
        let total_count: usize;
        asm!("
                mov $1, %r8             // TEST_COUNT
                mov $$0, %r9
                mov $$0, %r10
                mov $$0, %rax
                mov $$0, %rbx
                mov $$0, %rcx
                mov $$0, %r10

                inc %r8
            1:
                mov $2, %eax            // PIT_VALUE
                outb %al, $$0x40
                shr $$8, %eax
                outb %al, $$0x40
                movl $$~0, (%rdi)
                hlt

                mov $$~0, %ecx
                sub %rbx, %rcx
                mov $$0, %r9
                cmp $1, %r8
                cmovbe %rcx, %r9
                add %r9, %r10
                mov $$0, %r9

                dec %r8
                test %r8, %r8
                jnz 1b

                movl $$0, (%rdi)
        "
          : "={r10}" (total_count)
          : "n" (TEST_COUNT),
            "n" (PIT_VALUE),
            "{rdi}" (initial_count_reg_ptr),
            "{rsi}" (current_count_reg_ptr)
          : "cc", "memory", "rax", "rbx", "rcx", "r8", "r9", "r10"
          : "volatile");
        total_count
    }

    let mut apic = get_apic().lock();
    let initial_count_reg_ptr: *mut u32 = apic.register_as_mut_ptr::<apic::reg::InitialCount>();
    let current_count_reg_ptr: *mut u32 = apic.register_as_mut_ptr::<apic::reg::CurrentCount>();

    idt_mut()[0x20] = Descriptor::new()
        .with_offset(pit_tick as usize)
        .with_segment(brutos_task::arch::GDT_CODE_KERN)
        .with_ty(Type::Interrupt)
        .with_present(true);
    pc::interrupt::pit::initialize();
    pc::interrupt::initialize_pic(0x20, 0x40);

    unmask();

    let total = measure_ticks(initial_count_reg_ptr, current_count_reg_ptr);
    let bus_freq = total * 1000 / TEST_TIME_MS / TEST_COUNT;

    mask();
    pc::interrupt::disable_pic();

    idt_mut()[0x20] = Descriptor::new();

    bus_freq
}

unsafe fn setup_idt() {
    let mut idt = idt_mut();
    for i in 0..256 {
        idt[i] = Descriptor::new()
            .with_offset(self::entry::ENTRY_FUNCTIONS[i] as usize)
            .with_segment(brutos_task::arch::GDT_CODE_KERN)
            .with_ty(Type::Interrupt)
            .with_present(true);
        if i == vector::NMI as usize {
            idt[i].set_ist(NMI_IST);
        }
    }
}

fn rdtsc() -> u64 {
    let lo: u32;
    let hi: u32;
    unsafe {
        asm!("rdtsc" : "={eax}" (lo), "={edx}" (hi) ::: "volatile");
    }
    lo as u64 + ((hi as u64) << 32)
}

pub unsafe fn set_timer(in_ns: usize) {
    let ticks = match TICKS_PER_10NS * (in_ns / 10) {
        0 => max(1, TICKS_PER_1MS * (in_ns / 1000000)),
        x => x,
    };
    if USE_TSC {
        let deadline = rdtsc() + ticks as u64;
        msr::write::<msr::Ia32TscDeadline>(deadline);
    } else {
        get_apic()
            .lock()
            .write::<apic::reg::InitialCount>(ticks.try_into().expect("too many ticks"));
    }
}
