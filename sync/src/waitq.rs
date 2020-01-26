use core::sync::atomic::AtomicBool;

use brutos_util::linked_list::Sel;

use crate::Critical;

pub unsafe trait Context: Default + Critical {
    type WaitQSel: Sel;

    unsafe fn deschedule(&mut self) -> <Self::WaitQSel as Sel>::Immovable;
    unsafe fn schedule(&mut self, task: <Self::WaitQSel as Sel>::Immovable);

    /// This function is called in a critical context and must return in a critical context
    unsafe fn unlock_and_yield(is_locked: &AtomicBool);
}
