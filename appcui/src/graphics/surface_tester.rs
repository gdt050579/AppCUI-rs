use std::ops::{Deref, DerefMut};

use crate::graphics::Color;

use super::Surface;

pub (crate) struct SurfaceTester {
    surface: Surface,
}
impl SurfaceTester {
    pub (crate) fn new(width: u32, height: u32) -> SurfaceTester {
        SurfaceTester {
            surface: Surface::new(width, height),
        }
    }
    #[allow(dead_code)]
    pub (crate) fn print(&mut self) {
        let mut temp_buf = String::with_capacity(1024);
        temp_buf.clear();
        let mut x = 0u32;
        let mut y = 0u32;

        // firt border
        for _ in 0..=6 + self.surface.width {
            temp_buf.push('=');
        }
        println!("+{}+", temp_buf);
        temp_buf.clear();
        // second digit
        temp_buf.push_str("|    | ");
        for i in 0..self.surface.width {
            let digit = ((i % 100) / 10) as u8;
            if digit == 0 {
                temp_buf.push(' ');
            } else {
                temp_buf.push((48u8 + digit) as char);
            }
        }
        println!("{} |", temp_buf);
        temp_buf.clear();

        // last digit
        temp_buf.push_str("|    | ");
        for i in 0..self.surface.width {
            temp_buf.push((48u8 + ((i % 10) as u8)) as char);
        }
        println!("{} |", temp_buf);
        temp_buf.clear();

        // separator line
        temp_buf.push('|');
        for _ in 0..=6 + self.surface.width {
            temp_buf.push('-');
        }
        temp_buf.push('|');
        println!("{}", temp_buf);

        temp_buf.clear();
        for ch in &self.surface.chars {
            temp_buf.push_str("\x1b[");
            match ch.foreground {
                Color::Black => temp_buf.push_str("30"),
                Color::DarkRed => temp_buf.push_str("31"),
                Color::DarkGreen => temp_buf.push_str("32"),
                Color::Olive => temp_buf.push_str("33"),
                Color::DarkBlue => temp_buf.push_str("34"),
                Color::Magenta => temp_buf.push_str("35"),
                Color::Teal => temp_buf.push_str("36"),
                Color::Silver => temp_buf.push_str("37"),
                Color::Gray => temp_buf.push_str("90"),
                Color::Red => temp_buf.push_str("91"),
                Color::Green => temp_buf.push_str("92"),
                Color::Yellow => temp_buf.push_str("93"),
                Color::Blue => temp_buf.push_str("94"),
                Color::Pink => temp_buf.push_str("95"),
                Color::Aqua => temp_buf.push_str("96"),
                Color::White => temp_buf.push_str("97"),
                _ => temp_buf.push_str("37"), /* default is white */
            }
            temp_buf.push(';');
            match ch.background {
                Color::Black => temp_buf.push_str("40"),
                Color::DarkRed => temp_buf.push_str("41"),
                Color::DarkGreen => temp_buf.push_str("42"),
                Color::Olive => temp_buf.push_str("43"),
                Color::DarkBlue => temp_buf.push_str("44"),
                Color::Magenta => temp_buf.push_str("45"),
                Color::Teal => temp_buf.push_str("46"),
                Color::Silver => temp_buf.push_str("47"),
                Color::Gray => temp_buf.push_str("100"),
                Color::Red => temp_buf.push_str("101"),
                Color::Green => temp_buf.push_str("102"),
                Color::Yellow => temp_buf.push_str("103"),
                Color::Blue => temp_buf.push_str("104"),
                Color::Pink => temp_buf.push_str("105"),
                Color::Aqua => temp_buf.push_str("106"),
                Color::White => temp_buf.push_str("107"),
                _ => temp_buf.push_str("40"), /* default is white */
            }
            temp_buf.push_str("m");
            if ch.code < ' ' {
                temp_buf.push(' ');
            } else {
                temp_buf.push(ch.code);
            }
            temp_buf.push_str("\x1b[0m"); // reset to default color
            x += 1;
            if x == self.surface.width {
                println!("|{:>3} | {} |", y, temp_buf);
                temp_buf.clear();
                x = 0;
                y += 1;
            }
        }
        // last border
        for _ in 0..=6 + self.surface.width {
            temp_buf.push('=');
        }
        println!("+{}+", temp_buf);
        println!("Hash: 0x{:X}", self.compute_hash());
    }

    pub (crate) fn compute_hash(&self) -> u64 {
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
            buf[6] = ((ch.flags.get_value() >> 8) & 0xFF) as u8;
            buf[7] = (ch.flags.get_value() & 0xFF) as u8;
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
