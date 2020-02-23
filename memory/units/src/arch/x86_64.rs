use crate::{Order, PageSize};

pub const PAGE_SIZE: usize = 0x1000;

impl PageSize {
    pub fn order(&self) -> Order {
        match self {
            PageSize::Normal => Order(0 * 9),
            PageSize::Large => Order(1 * 9),
            PageSize::Huge => Order(2 * 9),
        }
    }
}
