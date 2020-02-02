use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};

use brutos_platform_pc as pc;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    static IS_PANICKING: AtomicBool = AtomicBool::new(false);
    if IS_PANICKING
        .compare_exchange_weak(false, true, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        crate::arch::halt();
    }

    let mut screen = crate::arch::SCREEN.lock();
    screen.style = pc::fb::Style::new()
        .with_foreground(pc::fb::Color::White)
        .with_background(pc::fb::Color::Red);
    let _ = write!(&mut *screen, "{}", info);
    crate::arch::halt();
}