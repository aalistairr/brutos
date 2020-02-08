use brutos_memory_units::Order;

pub use brutos_platform_pc::mmu::*;

use crate::mmu::{PageSize, Tables};

impl PageSize {
    pub fn level(&self) -> Level {
        match self {
            PageSize::Normal => Level::Pt,
            PageSize::Large => Level::Pd,
            PageSize::Huge => Level::Pdp,
        }
    }

    pub fn order(&self) -> Order {
        match self {
            PageSize::Normal => Order(0),
            PageSize::Large => Order(9),
            PageSize::Huge => Order(9 * 2),
        }
    }
}

impl Tables {
    pub unsafe fn with_root(root: Entry) -> Tables {
        Tables {
            root: EntryCell::with_entry(root),
        }
    }
}
