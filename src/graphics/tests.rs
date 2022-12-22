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

        // firt border
        for i in 0..=6 + self.surface.width {
            self.temp_buf.push('=');
        }
        println!("+{}+", self.temp_buf);
        self.temp_buf.clear();
        // second digit
        self.temp_buf.push_str("|    | ");
        for i in 0..self.surface.width {
            let digit = ((i % 100) / 10) as u8;
            if digit == 0 {
                self.temp_buf.push(' ');
            } else {
                self.temp_buf.push((48u8 + digit) as char);
            }
        }
        println!("{} |", self.temp_buf);
        self.temp_buf.clear();

        // last digit
        self.temp_buf.push_str("|    | ");
        for i in 0..self.surface.width {
            self.temp_buf.push((48u8 + ((i % 10) as u8)) as char);
        }
        println!("{} |", self.temp_buf);
        self.temp_buf.clear();

        // separator line
        self.temp_buf.push('|');
        for i in 0..=6 + self.surface.width {
            self.temp_buf.push('-');
        }
        self.temp_buf.push('|');
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
                println!("|{:>3} | {} |", y, self.temp_buf);
                self.temp_buf.clear();
                x = 0;
                y += 1;
            }
        }
        // last border
        for i in 0..=6 + self.surface.width {
            self.temp_buf.push('=');
        }
        println!("+{}+", self.temp_buf);
        println!("Hash: 0x{:X}", self.compute_hash());
    }

    fn compute_hash(&self) -> u64 {
        // use FNV algorithm ==> https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
        let mut hash = 0xcbf29ce484222325u64;
        let mut buf = [0u8; 8];
        for ch in &self.chars {
            buf[0] = ((ch.code as u32) & 0xFF) as u8;
            buf[1] = (((ch.code as u32) >> 8) & 0xFF) as u8;
            buf[2] = (((ch.code as u32) >> 16) & 0xFF) as u8;
            buf[3] = (((ch.code as u32) >> 24) & 0xFF) as u8;
            buf[4] = ch.foreground as u8;
            buf[5] = ch.background as u8;
            buf[6] = 0;
            buf[7] = 0;
            for b in buf {
                hash = hash ^ (b as u64);
                hash = hash.wrapping_mul(0x00000100000001B3u64);
            }
        }
        return hash;
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
