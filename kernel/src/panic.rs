use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    static IS_PANICKING: AtomicBool = AtomicBool::new(false);
    if IS_PANICKING
        .compare_exchange_weak(false, true, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        crate::arch::halt();
    }

    let _ = write!(&mut *crate::arch::framebuffer::Screen::lock(), "{}", info);
    crate::arch::halt();
}
