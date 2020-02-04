use core::cell::UnsafeCell;
use core::ptr;

use brutos_util_macros::{bitfield, BitEnum, BitfieldNew};

use crate::io::outb;

#[repr(C)]
pub struct Character {
    value: u8,
    style: Style,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Debug, BitfieldNew)]
    #[repr(transparent)]
    pub struct Style(u8);

    pub field foreground: Color => 0..4;
    pub field background: Color => 4..8;
}

#[derive(BitEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Purple = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    LightPurple = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

impl Style {
    pub const fn inverted(self) -> Self {
        let fg = self.foreground();
        let bg = self.background();
        Style::new().with_foreground(bg).with_background(fg)
    }
}

pub const DEFAULT_STYLE: Style = Style::new()
    .with_foreground(Color::Gray)
    .with_background(Color::Black);

pub const FB_WIDTH: usize = 80;
pub const FB_HEIGHT: usize = 25;

type Framebuffer = [[UnsafeCell<Character>; FB_WIDTH]; FB_HEIGHT];

pub const FRAMEBUFFER_ADDR: usize = 0xb8000;

pub struct Screen {
    framebuffer: *mut Framebuffer,
    pub style: Style,
    x: usize,
    y: usize,
}

unsafe impl Send for Screen {}
unsafe impl Sync for Screen {}

impl Screen {
    pub const unsafe fn with_framebuffer(framebuffer: *mut Framebuffer) -> Screen {
        Screen {
            framebuffer,
            style: DEFAULT_STYLE,
            x: 0,
            y: 0,
        }
    }

    pub fn clear(&mut self) {
        for y in 0..FB_HEIGHT {
            for x in 0..FB_WIDTH {
                unsafe {
                    ptr::write_volatile(
                        (*self.framebuffer)[y][x].get(),
                        Character {
                            value: ' ' as u8,
                            style: self.style,
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
            ptr::write_volatile((*self.framebuffer)[y][x].get(), c);
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
                    let c = ptr::read((*self.framebuffer)[y + 1][x].get());
                    ptr::write_volatile((*self.framebuffer)[y][x].get(), c);
                }
            }
        }
        self.y = FB_HEIGHT - 1;
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
                    style: self.style,
                }),
                _ => self.put_character(Character {
                    value: '?' as u8,
                    style: self.style.inverted(),
                }),
            }
        }
        self.update_cursor();
        Ok(())
    }
}
