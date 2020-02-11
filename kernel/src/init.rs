use core::mem::MaybeUninit;
use core::pin::Pin;

use brutos_alloc::{Arc, OutOfMemory};
use brutos_cpio as cpio;
use brutos_elf as elf;
use brutos_memory_units::VirtAddr;
use brutos_memory_vm as vm;

use crate::{create_user_address_space, AddressSpace, Cx};

#[derive(Clone, Debug)]
pub enum Error {
    Cpio(cpio::Error),
    Elf(elf::Error),
    NoInit,
    OutOfMemory,
    Map(vm::mmu::MapError),
}

impl From<cpio::Error> for Error {
    fn from(e: cpio::Error) -> Error {
        Error::Cpio(e)
    }
}

impl From<elf::Error> for Error {
    fn from(e: elf::Error) -> Error {
        Error::Elf(e)
    }
}

impl From<OutOfMemory> for Error {
    fn from(OutOfMemory: OutOfMemory) -> Error {
        Error::OutOfMemory
    }
}

impl From<vm::mmu::MapError> for Error {
    fn from(e: vm::mmu::MapError) -> Error {
        Error::Map(e)
    }
}

static mut ADDR_SPACE: MaybeUninit<Pin<Arc<AddressSpace, Cx>>> = MaybeUninit::uninit();

pub unsafe fn run_init(cpio_module: &[u8]) -> Result<(), Error> {
    let init = cpio_get_init(cpio_module)?;

    let addr_space = ADDR_SPACE.write(create_user_address_space()?);
    let _entry = load_init(addr_space, init).expect("failed to load init");
    unimplemented!()
}

fn cpio_get_init(module: &[u8]) -> Result<&[u8], Error> {
    let mut init = None;
    for entry in cpio::entries(module) {
        let entry = entry?;
        if entry.filename == Some("brutos-init") {
            init = Some(entry.contents);
            break;
        }
    }
    init.ok_or(Error::NoInit)
}

fn load_init(_addr_space: &Pin<Arc<AddressSpace, Cx>>, _init: &[u8]) -> Result<VirtAddr, Error> {
    unimplemented!()
}
