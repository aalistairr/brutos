use core::pin::Pin;

use brutos_alloc::OutOfMemory;
use brutos_memory::{AllocPhysPage, Order, PhysAddr, VirtAddr};
use brutos_multiboot2::ffi::BootInfo;
use brutos_multiboot2::{MmapEntryTy, Tag};
use brutos_platform_pc as pc;
use brutos_sync::mutex::Mutex;
use brutos_task as task;

use crate::Cx;

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::arch::print(core::format_args_nl!($($arg)*))
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::print(core::format_args!($($arg)*)));
}

pub mod entry;
pub mod interrupt;
pub mod memory;
#[cfg(not(test))]
pub mod panic;

pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("
                cli
                hlt
            " :::: "volatile");
        }
    }
}

pub fn screen() -> Pin<&'static Mutex<pc::fb::Screen, Cx>> {
    static SCREEN: Mutex<pc::fb::Screen, Cx> = unsafe {
        Mutex::new(pc::fb::Screen::with_framebuffer({
            (pc::fb::FRAMEBUFFER_ADDR + memory::PHYS_IDENT_OFFSET) as *mut _
        }))
    };
    unsafe { Pin::new_unchecked(&SCREEN) }
}

pub extern "C" fn multiboot2_entry(multiboot_info_addr: PhysAddr) -> ! {
    let mut dummy_state = task::State::<crate::Cx>::dummy();
    let dummy_state = unsafe { core::pin::Pin::new_unchecked(&mut dummy_state) };
    unsafe {
        task::State::activate(dummy_state);
        task::arch::current_task_inc_critical_count();
    }

    screen().initialize();
    screen().lock().clear();

    unsafe {
        task::arch::initialize_and_load_gdt();
        self::interrupt::initialize();
        task::arch::current_task_dec_critical_count();
        pc::msr::map::<pc::msr::Ia32Efer, _>(|x| x.with_nx_enabled(true));
    }

    let multiboot_info = (multiboot_info_addr.0 + memory::PHYS_IDENT_OFFSET) as *const BootInfo;
    let multiboot_info = unsafe { &*multiboot_info };
    let multiboot_range = multiboot_info_addr..multiboot_info_addr + multiboot_info.size as usize;

    let mmap = multiboot_info
        .tags()
        .filter_map(|tag| match tag {
            Tag::MemoryMap(mmap) => Some(mmap),
            _ => None,
        })
        .next()
        .expect("No memory map found");
    let mmap = mmap
        .iter()
        .filter(|entry| entry.ty == MmapEntryTy::Available)
        .map(|entry| entry.range.clone());
    let mmap = self::memory::remove_reserved_memory(multiboot_range, mmap);

    unsafe {
        crate::main(mmap);
    }
}

pub unsafe fn initialize_with_address_space() {
    self::interrupt::initialize_with_address_space();
}

#[no_mangle]
pub fn print(args: core::fmt::Arguments) {
    core::fmt::Write::write_fmt(&mut *screen().lock(), args).expect("failed to write");
}

unsafe impl brutos_memory::vm::mmu::arch::Context for Cx {
    fn alloc_table(&mut self) -> Result<PhysAddr, OutOfMemory> {
        <Cx as AllocPhysPage>::alloc(Order(0))
            .map(|(addr, _)| addr)
            .map_err(|()| OutOfMemory)
    }

    unsafe fn dealloc_table(&mut self, addr: PhysAddr) {
        <Cx as AllocPhysPage>::dealloc(addr, Order(0));
    }

    fn map_table(&mut self, addr: PhysAddr) -> *mut brutos_memory::vm::mmu::arch::Table {
        self::memory::map_phys_ident(addr, Order(0).size())
            .expect("Failed to map page translation table into memory")
            .as_ptr() as *mut _
    }
}

#[naked]
unsafe fn idle_task_entry() {
    asm!("
    1:
        hlt
        jmp 1b
    " :::: "volatile");
}

pub fn idle_task_entry_addr() -> VirtAddr {
    VirtAddr(idle_task_entry as usize)
}

pub fn idle_task_entry_arg() -> usize {
    0
}
