use brutos_memory::PhysAddr;
use brutos_multiboot2::ffi::BootInfo;
use brutos_multiboot2::{MmapEntryTy, Tag};
use brutos_task as task;

pub const VMA_OFFSET: usize = 0xffffffff80000000;

pub mod entry;
pub mod framebuffer;
pub mod io;
pub mod memory;

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

pub extern "C" fn multiboot2_entry(multiboot_info_addr: PhysAddr) -> ! {
    let mut dummy_state = task::State::<crate::Cx>::dummy();
    let dummy_state = unsafe { core::pin::Pin::new_unchecked(&mut dummy_state) };
    unsafe {
        task::arch::load_gdt();
        task::State::activate(dummy_state);
        task::arch::current_task_inc_critical_count();
    }

    self::framebuffer::Screen::lock().clear();

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

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print!($($arg)*);
        $crate::print!("\n");
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::framebuffer::print(core::format_args!($($arg)*)));
}
