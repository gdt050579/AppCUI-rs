use std::ops::{Deref, DerefMut};

use super::CharAttribute;
use super::CharFlags;
use super::Character;
use super::Color;
use super::LineType;
use super::Surface;

struct SurfaceTester {
    surface: Surface,
}
impl SurfaceTester {
    fn new(width: u32, height: u32) -> SurfaceTester {
        SurfaceTester {
            surface: Surface::new(width, height),
        }
    }
    #[allow(dead_code)]
    fn print(&mut self) {
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
                Color::Gray=> temp_buf.push_str("90"),
                Color::Red=> temp_buf.push_str("91"),
                Color::Green=> temp_buf.push_str("92"),
                Color::Yellow=> temp_buf.push_str("93"),
                Color::Blue=> temp_buf.push_str("94"),
                Color::Pink=> temp_buf.push_str("95"),
                Color::Aqua=> temp_buf.push_str("96"),
                Color::White=> temp_buf.push_str("97"),
                _ => temp_buf.push_str("37") /* default is white */
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
                Color::Gray=> temp_buf.push_str("100"),
                Color::Red=> temp_buf.push_str("101"),
                Color::Green=> temp_buf.push_str("102"),
                Color::Yellow=> temp_buf.push_str("103"),
                Color::Blue=> temp_buf.push_str("104"),
                Color::Pink=> temp_buf.push_str("105"),
                Color::Aqua=> temp_buf.push_str("106"),
                Color::White=> temp_buf.push_str("107"),
                _ => temp_buf.push_str("40") /* default is white */
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
fn check_clear_with_clipping() {
    let mut s = SurfaceTester::new(40, 10);
    s.clear(Character::new(
        'x',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    //s.print();
    assert_eq!(s.compute_hash(), 0xD82E620861132325);
    s.set_clip(2, 2, 10, 6);
    s.clear(Character::with_char(' '));
    //s.print();
    assert_eq!(s.compute_hash(), 0x4556A89C009CADFD);
    s.set_clip(8, 4, 20, 9);
    s.clear(Character::with_char('.'));
    //s.print();
    assert_eq!(s.compute_hash(), 0xC0F23672210DB085);
    s.reset_clip();
    s.clear(Character::with_char('+'));
    //s.print();
    assert_eq!(s.compute_hash(), 0x6D177D1CC0356225);
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
#[test]
fn check_draw_rect_with_size() {
    let mut s = SurfaceTester::new(40, 10);
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.draw_rect_with_size(1, 1, 20, 5, LineType::Double, CharAttribute::default());
    //s.print();
    assert_eq!(s.compute_hash(),0xB2DEA1E9B27FD8B1);
}

#[test]
fn check_draw_vertical_line() {
    let mut s = SurfaceTester::new(15, 7);
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.draw_vertical_line(1, 1, 5, LineType::Single, CharAttribute::default());
    s.draw_vertical_line(3, 1, 5, LineType::Double, CharAttribute::default());
    s.draw_vertical_line(5, 1, 5, LineType::SingleThick, CharAttribute::default());
    s.draw_vertical_line(7, 1, 5, LineType::Border, CharAttribute::default());
    s.draw_vertical_line(9, 1, 5, LineType::Ascii, CharAttribute::default());
    s.draw_vertical_line(11, 1, 5, LineType::AsciiRound, CharAttribute::default());
    s.draw_vertical_line(13, 1, 5, LineType::SingleRound, CharAttribute::default());
    //s.print();
    assert_eq!(s.compute_hash(),0xBA48710BD060DFAB);
}

#[test]
fn check_draw_vertical_line_with_size() {
    let mut s = SurfaceTester::new(15, 7);
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.draw_vertical_line_with_size(1, 1, 5, LineType::Single, CharAttribute::default());
    s.draw_vertical_line_with_size(3, 1, 5, LineType::Double, CharAttribute::default());
    s.draw_vertical_line_with_size(5, 1, 5, LineType::SingleThick, CharAttribute::default());
    s.draw_vertical_line_with_size(7, 1, 5, LineType::Border, CharAttribute::default());
    s.draw_vertical_line_with_size(9, 1, 5, LineType::Ascii, CharAttribute::default());
    s.draw_vertical_line_with_size(11, 1, 5, LineType::AsciiRound, CharAttribute::default());
    s.draw_vertical_line_with_size(13, 1, 5, LineType::SingleRound, CharAttribute::default());
    //s.print();
    assert_eq!(s.compute_hash(),0xBA48710BD060DFAB);
}


#[test]
fn check_draw_horizontal_line() {
    let mut s = SurfaceTester::new(20, 15);
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.draw_horizontal_line(1, 1, 15, LineType::Single, CharAttribute::default());
    s.draw_horizontal_line(1, 3, 15, LineType::Double, CharAttribute::default());
    s.draw_horizontal_line(1, 5, 15, LineType::SingleThick, CharAttribute::default());
    s.draw_horizontal_line(1, 7, 15, LineType::Border, CharAttribute::default());
    s.draw_horizontal_line(1, 9, 15, LineType::Ascii, CharAttribute::default());
    s.draw_horizontal_line(1, 11, 15, LineType::AsciiRound, CharAttribute::default());
    s.draw_horizontal_line(1, 13, 15, LineType::SingleRound, CharAttribute::default());
    //s.print();
    assert_eq!(s.compute_hash(),0xC8627A5B784CE327);
}

#[test]
fn check_draw_horizontal_line_with_size() {
    let mut s = SurfaceTester::new(20, 15);
    s.clear(Character::new(
        ' ',
        Color::White,
        Color::Black,
        CharFlags::None,
    ));
    s.draw_horizontal_line_with_size(1, 1, 15, LineType::Single, CharAttribute::default());
    s.draw_horizontal_line_with_size(1, 3, 15, LineType::Double, CharAttribute::default());
    s.draw_horizontal_line_with_size(1, 5, 15, LineType::SingleThick, CharAttribute::default());
    s.draw_horizontal_line_with_size(1, 7, 15, LineType::Border, CharAttribute::default());
    s.draw_horizontal_line_with_size(1, 9, 15, LineType::Ascii, CharAttribute::default());
    s.draw_horizontal_line_with_size(1, 11, 15, LineType::AsciiRound, CharAttribute::default());
    s.draw_horizontal_line_with_size(1, 13, 15, LineType::SingleRound, CharAttribute::default());
    //s.print();
    assert_eq!(s.compute_hash(),0xC8627A5B784CE327);
}

#[test]
fn check_cursor() {
    let mut s = SurfaceTester::new(20, 15);
    s.set_cursor(10, 5);
    assert!((s.cursor.x == 10) && (s.cursor.y==5));
    assert!(s.cursor.is_visible());
    s.hide_cursor();
    assert!(s.cursor.is_visible()==false);
    s.set_origin(3, 3);
    s.set_cursor(2, 2);
    assert!((s.cursor.x == 5) && (s.cursor.y==5));
    s.set_cursor(-2, -2);
    assert!((s.cursor.x == 1) && (s.cursor.y==1));
    s.set_clip(3, 3, 6, 6);
    s.set_cursor(-2, -2);
    assert!(s.cursor.is_visible()==false);
    s.set_cursor(4, 4);
    assert!(s.cursor.is_visible()==false);
    s.set_cursor(2, 2);
    assert!((s.cursor.x == 5) && (s.cursor.y==5));  
    assert!(s.cursor.is_visible());
}


#[test]
fn check_draw_surface() {
    let mut s = SurfaceTester::new(20, 15);
    let mut s2 = Surface::new(8,6);
    s2.clear(Character::new('X', Color::Yellow, Color::Black, CharFlags::None));
    s2.draw_rect(0, 0, 7, 5, LineType::Double, CharAttribute::with_color(Color::White, Color::DarkRed));
    s.draw_surface(2, 2, &s2);
    //s.print();
    assert_eq!(s.compute_hash(),0x22F426820E128C0D);
    s.draw_surface(-2, -2, &s2);
    //s.print();
    assert_eq!(s.compute_hash(),0x6D7FD783EB0FC3F0);
    s.clear(Character::with_char('.'));
    s.set_clip(3, 3, 5, 5);
    s.set_origin(3, 3);
    s.draw_surface(0, 0, &s2);
    //s.print();
    assert_eq!(s.compute_hash(),0x3C4BAE452177CAE0);
    s.draw_surface(-5, -3, &s2);
    //s.print();
    assert_eq!(s.compute_hash(),0xA18677AAC315ACE5);
    s.reset_clip();
    s.draw_surface(-5, -3, &s2);
    //s.print();
    assert_eq!(s.compute_hash(),0x3E6031703919C392);
}