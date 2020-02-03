#![feature(asm, global_asm, naked_functions)]
#![feature(const_raw_ptr_deref, const_mut_refs)]
#![feature(maybe_uninit_extra)]
#![no_std]
#![no_main]

use core::ops::Range;
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::AtomicBool;

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory};
use brutos_memory::slab_alloc as slab;
use brutos_memory::{AllocMappedPage, AllocPhysPage, Order, PhysAddr, VirtAddr};
use brutos_sync::mutex::{Mutex, PinMutex};
use brutos_task::Task;

#[macro_use]
pub mod arch;
pub mod memory;

pub unsafe fn main(mmap: impl Clone + Iterator<Item = Range<PhysAddr>>) -> ! {
    println!("Loading BrutOS");
    memory::initialize();
    initialize_task_allocator();
    initialize_mapping_allocator();
    initialize_addr_space_allocator();
    let available_memory = memory::bootstrap(mmap).expect("Failed to bootstrap physical memory");
    println!("{} bytes available", available_memory);
    create_kernel_address_space().expect("failed to create kernel address space");
    unimplemented!()
}

#[derive(Default)]
pub struct Cx;

unsafe impl brutos_sync::Critical for Cx {
    unsafe fn enter_critical() {
        brutos_task::arch::current_task_inc_critical_count();
    }

    unsafe fn leave_critical() {
        if brutos_task::arch::current_task_dec_critical_count() {
            self::arch::interrupt::enable();
        }
    }
}

unsafe impl brutos_sync::waitq::Context for Cx {
    type WaitQSel = brutos_task::WaitQSel<Cx>;

    unsafe fn deschedule(&mut self) -> Pin<Arc<Task<Cx>, Cx>> {
        unimplemented!()
    }

    unsafe fn schedule(&mut self, _: Pin<Arc<Task<Cx>, Cx>>) {
        unimplemented!()
    }

    unsafe fn unlock_and_yield(_: &AtomicBool) {
        unimplemented!()
    }
}

unsafe impl AllocPhysPage for Cx {
    const MAX_ORDER: Order = Order(brutos_memory::phys_alloc::MAX_ORDER);

    type PageData = memory::PageData;

    fn alloc(order: Order) -> Result<(PhysAddr, &'static Self::PageData), ()> {
        self::memory::phys_allocator()
            .lock()
            .as_mut()
            .allocate(order)
            .expect("Allocation is too large")
            .ok_or(())
    }

    unsafe fn dealloc(addr: PhysAddr, _: Order) {
        self::memory::phys_allocator()
            .lock()
            .as_mut()
            .free(addr)
            .expect("Failed to deallocate")
    }

    fn get_data(addr: PhysAddr) -> &'static Self::PageData {
        self::memory::get_data(addr).expect("Address is not allocated")
    }
}

unsafe impl AllocMappedPage for Cx {
    const MAX_ORDER: Order = Order(brutos_memory::phys_alloc::MAX_ORDER);

    fn alloc(order: Order) -> Result<NonNull<u8>, ()> {
        self::memory::phys_allocator()
            .lock()
            .as_mut()
            .allocate(order)
            .expect("Allocation is too large")
            .ok_or(())
            .and_then(|(addr, _)| self::arch::memory::map_phys_ident(addr, order.size()))
    }

    unsafe fn dealloc(ptr: NonNull<u8>, _: Order) {
        self::memory::phys_allocator()
            .lock()
            .as_mut()
            .free(self::arch::memory::phys_ident_addr(ptr))
            .expect("Failed to deallocate")
    }
}

macro_rules! slab_allocator {
    ($n:ident, $o:expr, $t:ty) => {
        fn $n() -> Pin<&'static PinMutex<slab::Allocator<Cx>, Cx>> {
            static ALLOCATOR: PinMutex<slab::Allocator<Cx>, Cx> =
                PinMutex::new(slab::Allocator::new::<$t>(Order($o)));
            unsafe { Pin::new_unchecked(&ALLOCATOR) }
        }

        unsafe impl AllocOne<$t> for Cx {
            unsafe fn alloc(&mut self) -> Result<NonNull<$t>, OutOfMemory> {
                $n().lock().as_mut().alloc().map(NonNull::cast)
            }
            unsafe fn dealloc(&mut self, ptr: NonNull<$t>) {
                $n().lock().as_mut().dealloc(ptr.cast())
            }
        }
    };
}

slab_allocator!(task_allocator, 1, ArcInner<Task<Cx>>);
fn initialize_task_allocator() {
    task_allocator().initialize();
    task_allocator().lock().as_mut().initialize();
}
slab_allocator!(
    mapping_allocator,
    1,
    ArcInner<brutos_memory::vm::Mapping<Cx>>
);
fn initialize_mapping_allocator() {
    mapping_allocator().initialize();
    mapping_allocator().lock().as_mut().initialize();
}
slab_allocator!(addr_space_allocator, 1, ArcInner<AddressSpace>);
fn initialize_addr_space_allocator() {
    addr_space_allocator().initialize();
    addr_space_allocator().lock().as_mut().initialize();
}

pub struct AddressSpace {
    vm: brutos_memory::vm::Space<Cx>,
}

static mut KERNEL_ADDR_SPACE: core::mem::MaybeUninit<Pin<Arc<AddressSpace, Cx>>> =
    core::mem::MaybeUninit::uninit();

unsafe fn create_kernel_address_space() -> Result<(), OutOfMemory> {
    let addr_space = KERNEL_ADDR_SPACE.write(
        Arc::pin(AddressSpace {
            vm: brutos_memory::vm::Space::new(
                crate::arch::memory::KERNEL_ADDR_SPACE_RANGE,
                crate::arch::memory::create_kernel_mmu_tables()?,
            ),
        })
        .map_err(|(e, space)| {
            crate::arch::memory::destroy_kernel_mmu_tables(Mutex::into_inner(space.vm.mmu_tables));
            e
        })?,
    );
    addr_space.vm().initialize();
    Ok(())
}

impl AddressSpace {
    pub unsafe fn kernel() -> &'static Pin<Arc<AddressSpace, Cx>> {
        &*KERNEL_ADDR_SPACE.as_ptr()
    }

    pub fn vm<'a>(self: &'a Pin<Arc<AddressSpace, Cx>>) -> Pin<&'a brutos_memory::vm::Space<Cx>> {
        unsafe { self.as_ref().map_unchecked(|x| &x.vm) }
    }
}

impl brutos_task::Context for Cx {
    type AddrSpace = Pin<Arc<AddressSpace, Cx>>;

    fn alloc_stack(&mut self) -> Result<VirtAddr, OutOfMemory> {
        unimplemented!()
    }

    unsafe fn dealloc_stack(&mut self, _: VirtAddr) {
        unimplemented!()
    }
}

impl brutos_memory::vm::Context for Cx {
    fn shared_empty_page(&mut self, _: Order) -> Option<(PhysAddr, &Self::PageData)> {
        unimplemented!()
    }
}

unsafe impl brutos_memory::MapPhysPage for Cx {
    type Err = ();

    unsafe fn with_mapped_page<F, R>(addr: PhysAddr, order: Order, f: F) -> Result<R, Self::Err>
    where
        F: FnOnce(*mut u8) -> R,
    {
        crate::arch::memory::map_phys_ident(addr, order.size())
            .map(NonNull::as_ptr)
            .map(f)
    }
}
