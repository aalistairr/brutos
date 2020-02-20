use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};

use brutos_alloc::{Arc, OutOfMemory};
use brutos_memory_vm as vm;
use brutos_sync::mutex::Mutex;

use crate::Cx;

pub struct AddressSpace {
    is_alive: AtomicBool,
    vm: vm::Space<Cx>,
}

static mut KERNEL_ADDR_SPACE: core::mem::MaybeUninit<Pin<Arc<AddressSpace, Cx>>> =
    core::mem::MaybeUninit::uninit();

pub unsafe fn create_kernel_address_space() -> Result<(), OutOfMemory> {
    let addr_space = KERNEL_ADDR_SPACE.write(
        Arc::pin(AddressSpace {
            is_alive: AtomicBool::new(true),
            vm: vm::Space::new(
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
    crate::arch::memory::create_kernel_mappings(addr_space);
    Ok(())
}

pub fn create_user_address_space() -> Result<Pin<Arc<AddressSpace, Cx>>, vm::mmu::MapError> {
    let addr_space = Arc::pin(AddressSpace {
        is_alive: AtomicBool::new(true),
        vm: vm::Space::new(crate::arch::memory::USER_ADDR_SPACE_RANGE, unsafe {
            crate::arch::memory::create_user_mmu_tables()?
        }),
    })
    .map_err(|(e, space)| {
        unsafe {
            crate::arch::memory::destroy_user_mmu_tables(Mutex::into_inner(space.vm.mmu_tables));
        }
        e
    })?;
    addr_space.vm().initialize();
    Ok(addr_space)
}

impl AddressSpace {
    pub unsafe fn kernel() -> &'static Pin<Arc<AddressSpace, Cx>> {
        &*KERNEL_ADDR_SPACE.as_ptr()
    }

    pub fn vm<'a>(self: &'a Pin<Arc<AddressSpace, Cx>>) -> Pin<&'a vm::Space<Cx>> {
        unsafe { self.as_ref().map_unchecked(|x| &x.vm) }
    }

    pub fn kill(&self) {
        self.is_alive.store(false, Ordering::Release);
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive.load(Ordering::Acquire)
    }
}
