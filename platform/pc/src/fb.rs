use core::cell::UnsafeCell;
use core::ptr;

use crate::io::outb;

#[repr(C)]
pub struct Character {
    value: u8,
    color: u8,
}

const COLOR_DEFAULT: u8 = 0x07;
const COLOR_INVERTED: u8 = 0x70;

pub const FB_WIDTH: usize = 80;
pub const FB_HEIGHT: usize = 25;

type Framebuffer = [[UnsafeCell<Character>; FB_WIDTH]; FB_HEIGHT];

pub const FRAMEBUFFER_ADDR: usize = 0xb8000;

pub struct Screen<'a> {
    framebuffer: &'a Framebuffer,
    x: usize,
    y: usize,
}

impl<'a> Screen<'a> {
    pub fn with_framebuffer(framebuffer: &'a Framebuffer) -> Screen<'a> {
        Screen {
            framebuffer,
            x: 0,
            y: 0,
        }
    }

    pub fn clear(&mut self) {
        for y in 0..FB_HEIGHT {
            for x in 0..FB_WIDTH {
                unsafe {
                    ptr::write_volatile(
                        self.framebuffer[y][x].get(),
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
            ptr::write_volatile(self.framebuffer[y][x].get(), c);
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
        for y in 0..FB_HEIGHT - 1 {
            for x in 0..FB_WIDTH {
                unsafe {
                    let c = ptr::read(self.framebuffer[y + 1][x].get());
                    ptr::write_volatile(self.framebuffer[y][x].get(), c);
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

impl<'a> core::fmt::Write for Screen<'a> {
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
