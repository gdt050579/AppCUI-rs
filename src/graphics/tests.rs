use std::ops::{Deref, DerefMut};

use super::CharFlags;
use super::Character;
use super::Color;
use super::Surface;

struct SurfaceTester {
    surface: Surface,
    temp_buf: String,
}
impl SurfaceTester {
    fn new(width: u32, height: u32) -> SurfaceTester {
        SurfaceTester {
            surface: Surface::new(width, height),
            temp_buf: String::with_capacity(width as usize),
        }
    }
    fn print(&mut self) {
        self.temp_buf.clear();
        let mut x = 0u32;
        let mut y = 0u32;

        // second digit
        self.temp_buf.push_str("      ");
        for i in 0..self.surface.width {
            let digit = ((i % 100) / 10) as u8;
            if digit == 0 {
                self.temp_buf.push(' ');
            } else {
                self.temp_buf.push((48u8 + digit) as char);
            }
        }
        println!("{}", self.temp_buf);
        self.temp_buf.clear();

        // last digit
        self.temp_buf.push_str("      ");
        for i in 0..self.surface.width {
            self.temp_buf.push((48u8 + ((i % 10) as u8)) as char);
        }
        println!("{}", self.temp_buf);
        self.temp_buf.clear();

        // separator line
        for i in 0..=5 + self.surface.width {
            self.temp_buf.push('-');
        }
        println!("{}", self.temp_buf);
        self.temp_buf.clear();
        for ch in &self.surface.chars {
            if ch.code < ' ' {
                self.temp_buf.push(' ');
            } else {
                self.temp_buf.push(ch.code);
            }
            x += 1;
            if x == self.surface.width {
                println!("{:>3} | {}", y, self.temp_buf);
                self.temp_buf.clear();
                x = 0;
                y += 1;
            }
        }
    }
}
impl Deref for SurfaceTester {
    type Target = Surface;

    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}
impl DerefMut for SurfaceTester {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.surface
    }
}

#[test]
fn check_clear() {
    let mut s = SurfaceTester::new(20, 5);
    s.clear(Character::new(
        'x',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.print();
}
