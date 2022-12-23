use std::ops::{Deref, DerefMut};

use super::CharAttribute;
use super::CharFlags;
use super::Character;
use super::Color;
use super::LineType;
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
        for _ in 0..=6 + self.surface.width {
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
        for _ in 0..=6 + self.surface.width {
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
        for _ in 0..=6 + self.surface.width {
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

#[test]
fn check_clear() {
    let mut s = SurfaceTester::new(20, 5);
    s.clear(Character::new(
        'x',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    //s.print();
    assert_eq!(s.compute_hash(), 0x19B0E1632DAE6325);
}
#[test]
fn check_fill_rect() {
    let mut s = SurfaceTester::new(20, 5);
    s.clear(Character::new(
        '.',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.fill_rect(
        2,
        2,
        4,
        4,
        Character::new('@', Color::Aqua, Color::Red, CharFlags::Bold),
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x9E357B7ADEDEB720);
    s.fill_rect_with_size(4, 1, 10, 2, Character::with_char('X'));
    //s.print();
    assert_eq!(s.compute_hash(), 0xD897421A927A1A1);
}
#[test]
fn check_draw_rect() {
    let mut s = SurfaceTester::new(40, 10);
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.draw_rect(
        2,
        2,
        10,
        4,
        LineType::Single,
        CharAttribute::with_color(Color::Yellow, Color::Blue),
    );
    s.draw_rect(
        12,
        1,
        18,
        5,
        LineType::Double,
        CharAttribute::with_color(Color::White, Color::Green),
    );
    s.draw_rect(
        20,
        0,
        28,
        3,
        LineType::SingleThick,
        CharAttribute::with_color(Color::Aqua, Color::Black),
    );
    s.draw_rect(
        29,
        0,
        39,
        3,
        LineType::Border,
        CharAttribute::with_color(Color::Aqua, Color::Black),
    );   
    s.draw_rect(
        20,
        4,
        30,
        8,
        LineType::Ascii,
        CharAttribute::with_color(Color::Green, Color::White),
    );     
    s.draw_rect(
        31,
        4,
        38,
        8,
        LineType::AsciiRound,
        CharAttribute::with_color(Color::Green, Color::White),
    );   
    s.draw_rect(
        1,
        6,
        17,
        9,
        LineType::SingleRound,
        CharAttribute::with_color(Color::Green, Color::White),
    );       
    //s.print();
    assert_eq!(s.compute_hash(), 0xD99DB2F59085FE71);
}
