use core::cell::UnsafeCell;
use core::ptr;

use brutos_sync::spinlock::{Spinlock, SpinlockGuard};

use crate::arch::io::outb;
use crate::Cx;

#[repr(C)]
struct Character {
    value: u8,
    color: u8,
}

const COLOR_DEFAULT: u8 = 0x07;
const COLOR_INVERTED: u8 = 0x70;

const FB_WIDTH: usize = 80;
const FB_HEIGHT: usize = 25;

pub struct Screen {
    x: usize,
    y: usize,
}

impl Screen {
    pub fn lock() -> SpinlockGuard<'static, Screen, Cx> {
        static SCREEN: Spinlock<Screen, Cx> = Spinlock::new(Screen { x: 0, y: 0 });
        SCREEN.lock()
    }

    fn framebuffer(&mut self) -> &[[UnsafeCell<Character>; FB_WIDTH]; FB_HEIGHT] {
        unsafe { &*((0xb8000 + crate::arch::memory::PHYS_IDENT_OFFSET) as *const _) }
    }

    pub fn clear(&mut self) {
        let fb = self.framebuffer();
        for y in 0..FB_HEIGHT {
            for x in 0..FB_WIDTH {
                unsafe {
                    ptr::write_volatile(
                        fb[y][x].get(),
                        Character {
                            value: ' ' as u8,
                            color: COLOR_DEFAULT,
                        },
                    );
                }
            }
        }
        self.x = 0;
        self.y = 0;
        self.update_cursor();
    }

    fn newline(&mut self) {
        self.x = 0;
        self.y += 1;
        if self.y >= FB_HEIGHT {
            self.scroll();
        }
    }

    fn put_character(&mut self, c: Character) {
        unsafe {
            let (x, y) = (self.x, self.y);
            ptr::write_volatile(self.framebuffer()[y][x].get(), c);
        }
        self.x += 1;
        if self.x >= FB_WIDTH {
            self.x = 0;
            self.y += 1;
            if self.y >= FB_HEIGHT {
                self.y = 0;
                self.scroll();
            }
        }
    }

    fn scroll(&mut self) {
        let fb = self.framebuffer();
        for y in 0..FB_HEIGHT - 1 {
            for x in 0..FB_WIDTH {
                unsafe {
                    let c = ptr::read(fb[y + 1][x].get());
                    ptr::write_volatile(fb[y][x].get(), c);
                }
            }
        }
    }

    fn update_cursor(&mut self) {
        unsafe {
            let pos = (self.y * FB_WIDTH + self.x) as u16;
            outb(0x3d4, 0xf);
            outb(0x3d5, pos as u8);
            outb(0x3d4, 0xe);
            outb(0x3d5, (pos >> 8) as u8);
        }
    }
}

impl core::fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            match c {
                '\n' => self.newline(),
                c if c.is_ascii() && !c.is_control() => self.put_character(Character {
                    value: c as u8,
                    color: COLOR_DEFAULT,
                }),
                _ => self.put_character(Character {
                    value: '?' as u8,
                    color: COLOR_INVERTED,
                }),
            }
        }
        self.update_cursor();
        Ok(())
    }
}

pub fn print(args: core::fmt::Arguments) {
    let _ = core::fmt::Write::write_fmt(&mut *Screen::lock(), args);
}
