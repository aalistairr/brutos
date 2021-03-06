use core::cmp::min;
use core::mem::MaybeUninit;
use core::pin::Pin;

use brutos_alloc::{Arc, OutOfMemory};
use brutos_cpio as cpio;
use brutos_elf as elf;
use brutos_elf::SegmentTypeStandard;
use brutos_memory_traits::{MapPhysPage, MmuEntry, MmuFlags, MmuMap, PageSize};
use brutos_memory_units::arch::PAGE_SIZE;
use brutos_memory_units::{Order, VirtAddr};
use brutos_memory_vm as vm;
use brutos_sync::spinlock::Spinlock;
use brutos_task::{self as task, Task};
use brutos_util::UInt;

use crate::arch::memory::MmuMap as ArchMmuMap;
use crate::memory::addr_space::{create_user_address_space, AddressSpace};
use crate::memory::Object;
use crate::Cx;

#[derive(Clone, Debug)]
pub enum Error {
    Cpio(cpio::Error),
    Elf(elf::Error),
    MmuMap(<ArchMmuMap as MmuMap>::MapErr),
    VmCreate(vm::CreateError),
    VmPrefill(vm::PrefillError<Cx>),
    NoBootstrap,
    OutOfMemory,
    InvalidExecutable,
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

static mut ADDR_SPACE: MaybeUninit<Pin<Arc<AddressSpace, Cx>>> = MaybeUninit::uninit();

pub unsafe fn create_bootstrap_task(cpio_module: &[u8]) -> Result<Pin<Arc<Task<Cx>, Cx>>, Error> {
    let bootstrap = cpio_get_bootstrap(cpio_module)?;

    let addr_space = ADDR_SPACE.write(create_user_address_space().map_err(Error::MmuMap)?);
    let entry = load_bootstrap(addr_space, bootstrap).expect("failed to load bootstrap");
    let page_tables = addr_space
        .vm()
        .mmu_map()
        .lock()
        .page_tables()
        .expect("page tables have not been created");
    Ok(Task::new(
        Spinlock::new(crate::task::TaskAddrSpace::Inactive(Arc::pin_downgrade(
            addr_space,
        ))),
        0,
        task::EntryPoint::User(entry, 0),
        page_tables,
    )?)
}

fn cpio_get_bootstrap(module: &[u8]) -> Result<&[u8], Error> {
    let mut bootstrap = None;
    for entry in cpio::entries(module) {
        let entry = entry?;
        if entry.filename == Some("brutos-bootstrap") {
            bootstrap = Some(entry.contents);
            break;
        }
    }
    bootstrap.ok_or(Error::NoBootstrap)
}

fn load_bootstrap(
    addr_space: &Pin<Arc<AddressSpace, Cx>>,
    bootstrap: &[u8],
) -> Result<VirtAddr, Error> {
    let file_header = elf::file_header(bootstrap)?;
    if file_header.class != elf::Class::Class64 || file_header.ty != elf::FileType::Exec {
        return Err(Error::InvalidExecutable);
    }

    let segments = elf::program_headers(bootstrap, &file_header)?;
    let load_segments = segments.filter(|s| match s {
        Ok(s) => s.ty == SegmentTypeStandard::Load,
        Err(_) => false,
    });
    for segment in load_segments {
        let segment = segment?;
        let segment_addr = VirtAddr(segment.vaddr.u64().ok_or(Error::InvalidExecutable)? as usize);
        let segment_offset = segment.offset.u64().ok_or(Error::InvalidExecutable)? as usize;
        let segment_filesize = segment.filesz.u64().ok_or(Error::InvalidExecutable)? as usize;
        let segment_memsize = segment.memsz.u64().ok_or(Error::InvalidExecutable)? as usize;

        if !segment_addr.is_aligned(PAGE_SIZE) || segment_addr == VirtAddr(0) {
            return Err(Error::InvalidExecutable);
        }

        let mapping = addr_space
            .vm()
            .create_mapping(
                segment_memsize.align_up(PAGE_SIZE),
                vm::Location::Fixed(segment_addr),
                vm::Source::Private(Object::Anonymous),
                PageSize::Normal,
                vm::Flags {
                    mapping: vm::MappingFlags {
                        guarded: false,
                        wired: true,
                    },
                    mmu: MmuFlags {
                        user_accessible: true,
                        writable: segment.flags.write(),
                        executable: segment.flags.execute(),
                        global: false,
                        copied: false,
                        cache_type: 0,
                    },
                },
            )
            .map_err(Error::VmCreate)?;

        addr_space
            .vm()
            .prefill(&mapping)
            .map_err(Error::VmPrefill)?;

        let mmu_map = addr_space.vm().mmu_map().lock();
        for offset in (0..segment_filesize).step_by(PAGE_SIZE) {
            const FAIL_GET_PAGE: &str = "load_bootstrap: failed to get mapped page";
            let addr = segment_addr + offset;
            let mmu_entry = mmu_map
                .get_entry(PageSize::Normal, addr)
                .expect(FAIL_GET_PAGE)
                .expect(FAIL_GET_PAGE);
            let page =
                <<ArchMmuMap as MmuMap>::Entry as MmuEntry>::address(&mmu_entry, PageSize::Normal);
            let size = min(PAGE_SIZE, segment_filesize - offset);

            let segment_bytes =
                bootstrap[segment_offset + offset..segment_offset + offset + size].as_ptr();
            unsafe {
                <Cx as MapPhysPage>::with_mapped_page(page, Order(0), |page| {
                    core::ptr::copy(segment_bytes, page, size);
                })
                .expect("load_bootstrap: failed to map mapped page");
            }
        }
    }

    let entry = file_header.entry.u64().ok_or(Error::InvalidExecutable)? as usize;
    if entry == 0 {
        return Err(Error::InvalidExecutable);
    }
    Ok(VirtAddr(entry))
}
