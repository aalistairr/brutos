use core::pin::Pin;
use core::slice;

use brutos_memory_phys_alloc::bootstrap::CutRange;
use brutos_memory_units::{PhysAddr, VirtAddr};
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
pub mod syscall;

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
        Mutex::new(pc::fb::Screen::with_framebuffer(
            crate::arch::memory::map_phys_ident_unchecked(pc::fb::FRAMEBUFFER_ADDR)
                .cast()
                .as_ptr(),
        ))
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
        syscall::initialize();
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

    let mut mmap_cut_module = PhysAddr(0)..PhysAddr(0);
    let mut init_module = None;
    for tag in multiboot_info.tags() {
        match tag {
            Tag::Module { range, .. } => {
                mmap_cut_module = range.clone();
                let module_size = range.end - range.start;
                let module = memory::map_phys_ident(range.start, module_size)
                    .expect("failed to map init module");
                let module = unsafe { slice::from_raw_parts(module.as_ptr(), module_size) };
                init_module = Some(module);
            }
            _ => (),
        }
    }

    let mmap = CutRange::new(mmap, mmap_cut_module);

    unsafe {
        crate::main(mmap, init_module);
    }
}

pub unsafe fn initialize_with_address_space() {
    self::interrupt::initialize_with_address_space();
}

#[no_mangle]
pub fn print(args: core::fmt::Arguments) {
    core::fmt::Write::write_fmt(&mut *screen().lock(), args).expect("failed to write");
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
