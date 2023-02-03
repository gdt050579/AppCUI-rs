use std::ops::{Deref, DerefMut};

use crate::graphics::text_format::TextWrap;

use super::CharAttribute;
use super::CharFlags;
use super::Character;
use super::Color;
use super::Image;
use super::ImageRenderingMethod;
use super::ImageScaleMethod;
use super::LineType;
use super::Surface;
use super::TextAlignament;
use super::TextFormat;

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
    assert_eq!(s.compute_hash(), 0xB2DEA1E9B27FD8B1);
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
    assert_eq!(s.compute_hash(), 0xBA48710BD060DFAB);
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
    assert_eq!(s.compute_hash(), 0xBA48710BD060DFAB);
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
    assert_eq!(s.compute_hash(), 0xC8627A5B784CE327);
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
    assert_eq!(s.compute_hash(), 0xC8627A5B784CE327);
}

#[test]
fn check_cursor() {
    let mut s = SurfaceTester::new(20, 15);
    s.set_cursor(10, 5);
    assert!((s.cursor.x == 10) && (s.cursor.y == 5));
    assert!(s.cursor.is_visible());
    s.hide_cursor();
    assert!(s.cursor.is_visible() == false);
    s.set_origin(3, 3);
    s.set_cursor(2, 2);
    assert!((s.cursor.x == 5) && (s.cursor.y == 5));
    s.set_cursor(-2, -2);
    assert!((s.cursor.x == 1) && (s.cursor.y == 1));
    s.set_clip(3, 3, 6, 6);
    s.set_cursor(-2, -2);
    assert!(s.cursor.is_visible() == false);
    s.set_cursor(4, 4);
    assert!(s.cursor.is_visible() == false);
    s.set_cursor(2, 2);
    assert!((s.cursor.x == 5) && (s.cursor.y == 5));
    assert!(s.cursor.is_visible());
}

#[test]
fn check_draw_surface() {
    let mut s = SurfaceTester::new(20, 15);
    let mut s2 = Surface::new(8, 6);
    s2.clear(Character::new(
        'X',
        Color::Yellow,
        Color::Black,
        CharFlags::None,
    ));
    s2.draw_rect(
        0,
        0,
        7,
        5,
        LineType::Double,
        CharAttribute::with_color(Color::White, Color::DarkRed),
    );
    s.draw_surface(2, 2, &s2);
    //s.print();
    assert_eq!(s.compute_hash(), 0x22F426820E128C0D);
    s.draw_surface(-2, -2, &s2);
    //s.print();
    assert_eq!(s.compute_hash(), 0x6D7FD783EB0FC3F0);
    s.clear(Character::with_char('.'));
    s.set_clip(3, 3, 5, 5);
    s.set_origin(3, 3);
    s.draw_surface(0, 0, &s2);
    //s.print();
    assert_eq!(s.compute_hash(), 0x3C4BAE452177CAE0);
    s.draw_surface(-5, -3, &s2);
    //s.print();
    assert_eq!(s.compute_hash(), 0xA18677AAC315ACE5);
    s.reset_clip();
    s.draw_surface(-5, -3, &s2);
    //s.print();
    assert_eq!(s.compute_hash(), 0x3E6031703919C392);
}

#[test]
fn check_colors() {
    let mut s = SurfaceTester::new(40, 5);
    s.set(
        1,
        1,
        Character::new('A', Color::White, Color::Red, CharFlags::None),
    );
    s.set(
        2,
        1,
        Character::new('A', Color::Red, Color::Black, CharFlags::None),
    );
    s.set(
        3,
        1,
        Character::new('B', Color::Yellow, Color::Blue, CharFlags::None),
    );
    s.set(
        4,
        1,
        Character::new('B', Color::Blue, Color::Yellow, CharFlags::None),
    );
    s.set(
        5,
        1,
        Character::new('C', Color::Yellow, Color::DarkBlue, CharFlags::None),
    );
    s.set(
        6,
        1,
        Character::new('B', Color::Black, Color::Magenta, CharFlags::None),
    );
    s.fill_rect(
        10,
        1,
        30,
        3,
        Character::new(' ', Color::Yellow, Color::DarkBlue, CharFlags::None),
    );
    s.draw_rect(
        10,
        1,
        30,
        3,
        LineType::Double,
        CharAttribute::with_color(Color::White, Color::DarkBlue),
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0xF47F25A9A2342269);
}

#[test]
fn check_draw_imge() {
    let mut s = SurfaceTester::new(40, 10);
    let i = Image::from_str(
        r#"
        |BB.........BB|
        |B..rr...rr..B|
        |..rrrr.rrrr..|
        |.rrrrrrrrrrr.|
        |.raaaaaaaaar.|
        |..ryyyyyyyr..|
        |   rwwwwwr   |
        |....rwwwr....|
        |G....rwr....G|
        |GG....r....GG|
    "#,
    )
    .unwrap();
    s.draw_image(
        1,
        1,
        &i,
        ImageRenderingMethod::PixelTo16ColorsSmallBlock,
        ImageScaleMethod::NoScale,
    );
    s.draw_image(
        20,
        1,
        &i,
        ImageRenderingMethod::PixelTo16ColorsSmallBlock,
        ImageScaleMethod::Scale50,
    );
    s.draw_image(
        30,
        1,
        &i,
        ImageRenderingMethod::PixelTo16ColorsSmallBlock,
        ImageScaleMethod::Scale25,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0xFD04064498933DB);
    s.clear(Character::default());
    s.draw_image(
        0,
        0,
        &i,
        ImageRenderingMethod::PixelTo64ColorsLargeBlock,
        ImageScaleMethod::NoScale,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x7BAAAA0605CBFA25);
    s.clear(Character::default());
    s.draw_image(
        0,
        0,
        &i,
        ImageRenderingMethod::GrayScale,
        ImageScaleMethod::NoScale,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x9803283450732669);
}

#[test]
fn check_write_string_single_line() {
    let mut s = SurfaceTester::new(40, 10);
    s.write_string(
        1,
        1,
        "text",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        false,
    );
    s.set_clip(6, 1, 10, 1);
    s.set_origin(6, 1);
    s.write_string(
        0,
        0,
        "A longer text",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        false,
    );
    s.reset_clip();
    s.write_string(
        0,
        2,
        "A longer text",
        CharAttribute::with_color(Color::White, Color::DarkBlue),
        false,
    );
    s.set_clip(6, 4, 10, 4);
    s.set_origin(6, 4);
    s.write_string(
        -2,
        0,
        "A longer text",
        CharAttribute::with_color(Color::White, Color::Magenta),
        false,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x8D311DA4D1D1666E);
}

#[test]
fn check_write_string_multi_line() {
    let mut s = SurfaceTester::new(40, 10);
    s.write_string(
        1,
        1,
        "Hello, world\nThis is a multi-line\nString",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        true,
    );
    s.set_clip(10, 3, 20, 4);
    s.write_string(
        9,
        3,
        "Hello, world\nThis is a multi-line\nString",
        CharAttribute::with_color(Color::White, Color::DarkGreen),
        true,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0xFD638AC7F26D347A);
}

#[test]
fn check_resize() {
    let mut s = SurfaceTester::new(40, 10);
    s.write_string(
        1,
        1,
        "Hello, world\nThis is a multi-line\nString",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        true,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0xB015E3D08D4D238B);
    s.resize(20, 5);
    assert_eq!(s.get_width(), 20);
    assert_eq!(s.get_height(), 5);
    s.write_string(
        1,
        1,
        "Hello, world\nThis is a multi-line\nString",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        true,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x5CA6952034D223D2);
    s.resize(100, 30);
    s.write_string(
        1,
        1,
        "Hello, world\nThis is a multi-line\nString",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        true,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x9891C34A4738FD0B);
}

#[test]
fn check_write_text_single_line_simple() {
    let mut s = SurfaceTester::new(60, 7);
    s.draw_vertical_line(30, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));

    let mut format = TextFormat::single_line(
        30,
        1,
        CharAttribute::with_color(Color::Yellow, Color::DarkRed),
        TextAlignament::Left,
    );
    s.write_text("Left Align at 30", &format);
    format.align = TextAlignament::Center;
    format.y = 3;
    format.char_attr = CharAttribute::with_color(Color::Yellow, Color::DarkGreen);
    s.write_text("Centered! at 30", &format);
    format.align = TextAlignament::Right;
    format.y = 5;
    format.char_attr = CharAttribute::with_color(Color::Yellow, Color::DarkBlue);    
    s.write_text("Right align ends at 30", &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0x8DFA95B692742714);
}

#[test]
fn check_write_text_single_line_width() {
    let mut s = SurfaceTester::new(60, 7);
    s.draw_vertical_line(30, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));

    let mut format = TextFormat::single_line(
        30,
        1,
        CharAttribute::with_color(Color::Yellow, Color::DarkRed),
        TextAlignament::Left,
    );
    format.width = Some(6);
    s.write_text("123456xxxxxxx", &format);
    format.align = TextAlignament::Center;
    format.y = 3;
    format.char_attr = CharAttribute::with_color(Color::Yellow, Color::DarkGreen);
    s.write_text("----123456----", &format);
    format.align = TextAlignament::Right;
    format.y = 5;
    format.char_attr = CharAttribute::with_color(Color::Yellow, Color::DarkBlue);    
    s.write_text("--------------------123456", &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0xC503745C2440B5F6);
}
#[test]
fn check_write_text_single_line_hot_key() {
    let mut s = SurfaceTester::new(60, 7);
    s.draw_vertical_line(30, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));

    let mut format = TextFormat::single_line_with_hotkey(
        30,
        1,
        CharAttribute::with_color(Color::Yellow, Color::DarkRed),
        CharAttribute::with_color(Color::Black, Color::Yellow),
        4,
        TextAlignament::Left,
    );
    s.write_text("HotKey is 'E'", &format);
    format.align = TextAlignament::Center;
    format.y = 3;
    format.char_attr = CharAttribute::with_color(Color::Yellow, Color::DarkGreen);
    format.hotkey_pos = Some(0);
    s.write_text("Centered (hotkey='C')", &format);
    format.align = TextAlignament::Right;
    format.y = 5;
    format.char_attr = CharAttribute::with_color(Color::Yellow, Color::DarkBlue);  
    format.hotkey_pos = Some(20);  
    s.write_text("Right align ends at 30", &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0x19AE9890D9B9E3AF);
}

#[test]
fn check_write_text_multi_line_no_wrap() {
    let mut s = SurfaceTester::new(80, 7);
    s.draw_vertical_line(2, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(40, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(78, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    let mut format = TextFormat::multi_line(
        2,
        1,
        CharAttribute::with_color(Color::Yellow, Color::DarkRed),
        TextAlignament::Left,
    );
    s.write_text("This is a\nmulti-line text\nwith 4 lines\nall left-aligned !", &format);    
    format.align = TextAlignament::Center;
    format.x = 40;
    s.write_text("This is a\nmulti-line text\nwith 5 lines\n\nall centered !", &format); 
    format.align = TextAlignament::Right;
    format.x = 78;
    s.write_text("This is a\nmulti-line text\n\nwith 6 lines\n\nall alligned to the right", &format); 

    //s.print();
    assert_eq!(s.compute_hash(), 0x5CA9237E8FF59BAF);
}

#[test]
fn check_write_text_multi_line_no_wrap_how_key() {
    let mut s = SurfaceTester::new(80, 7);
    s.draw_vertical_line(2, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(40, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(78, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    let mut format = TextFormat::multi_line(
        2,
        1,
        CharAttribute::with_color(Color::Yellow, Color::DarkBlue),
        TextAlignament::Left,
    );
    format.hotkey_attr = Some(CharAttribute::with_color(Color::Yellow, Color::DarkRed));
    format.hotkey_pos = Some(11);
    s.write_text("This is a\nmulti-line text\nwith 5 lines\nall left-aligned !\nand with hot key 'u'", &format);    
    format.align = TextAlignament::Center;
    format.x = 40;
    format.hotkey_pos = Some(26);
    format.char_attr = CharAttribute::with_color(Color::White, Color::Gray);
    s.write_text("This is a\nmulti-line text\nwith 5 lines\nall centered at y=40\nand with hot key 'w'", &format); 
    format.align = TextAlignament::Right;
    format.x = 78;
    format.hotkey_pos = Some(75);
    format.char_attr = CharAttribute::with_color(Color::White, Color::DarkGreen);
    s.write_text("This is a\nmulti-line text\nwith 6 lines\naligned to right\n\nand with hot key 'x'", &format); 

    //s.print();
    assert_eq!(s.compute_hash(), 0xCED1C065F1F2053B);
}

#[test]
fn check_write_text_multi_line_character_wrap() {
    let mut s = SurfaceTester::new(80, 10);
    let txt = "This is a line that will be wrapped on multiple lines on a given character width";
    s.draw_vertical_line(2, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(40, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(78, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    let mut format = TextFormat::multi_line_with_text_wrap(
        2,
        1,
        10,
        CharAttribute::with_color(Color::Yellow, Color::DarkRed),
        TextAlignament::Left,
        TextWrap::Character,
    );
    s.write_text(txt, &format);    
    format.align = TextAlignament::Center;
    format.x = 40;
    format.width = Some(30);
    s.write_text(txt, &format);
    format.align = TextAlignament::Right;
    format.x = 78;
    format.width = Some(7);
    s.write_text(txt, &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0x5C5090CB807A653);
}

#[test]
fn check_write_text_multi_line_character_wrap_new_lines() {
    let mut s = SurfaceTester::new(80, 10);
    let txt = "This is a line\nthat will be wrapped on multiple lines\non a\ngiven character width";
    s.draw_vertical_line(2, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(40, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(78, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    let mut format = TextFormat::multi_line_with_text_wrap(
        2,
        1,
        10,
        CharAttribute::with_color(Color::Yellow, Color::DarkRed),
        TextAlignament::Left,
        TextWrap::Character,
    );
    s.write_text(txt, &format);    
    format.align = TextAlignament::Center;
    format.x = 40;
    format.width = Some(30);
    s.write_text(txt, &format);
    format.align = TextAlignament::Right;
    format.x = 78;
    format.width = Some(7);
    s.write_text(txt, &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0xB7A3A6A4DED903CC);
}

#[test]
fn check_write_text_multi_line_character_wrap_new_lines_hotkey() {
    let mut s = SurfaceTester::new(80, 10);
    s.draw_vertical_line(2, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(40, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(78, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    let mut format = TextFormat::multi_line_with_text_wrap(
        2,
        1,
        10,
        CharAttribute::with_color(Color::Black, Color::Silver),
        TextAlignament::Left,
        TextWrap::Character,
    );
    format.hotkey_attr = Some(CharAttribute::with_color(Color::Yellow, Color::DarkRed));
    format.hotkey_pos = Some(17);
    s.write_text("This is a line\nthat will be wrapped on multiple lines\n\nHot key is 'a'", &format);    
    format.align = TextAlignament::Center;
    format.x = 40;
    format.width = Some(30);
    format.hotkey_pos = Some(28);
    s.write_text("This is a line\nthat will be wrapped on multiple lines\n\nHot key is 'w'", &format); 
    format.align = TextAlignament::Right;
    format.x = 78;
    format.width = Some(15);
    format.hotkey_pos = Some(67);
    s.write_text("This is a line\nthat will be wrapped on multiple lines\n\nHot key is 'x'", &format); 

    //s.print();
    assert_eq!(s.compute_hash(), 0x3CE81721E1BB64FA);
}

#[test]
fn check_write_text_multi_line_word_wrap() {
    let mut s = SurfaceTester::new(80, 10);
    let txt = "This is     a line that       will be wrapped    on multiple lines on a given long-character width";
    println!("{txt}");
    s.draw_vertical_line(2, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(40, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(78, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    let mut format = TextFormat::multi_line_with_text_wrap(
        2,
        1,
        10,
        CharAttribute::with_color(Color::Yellow, Color::DarkRed),
        TextAlignament::Left,
        TextWrap::Word,
    );
    s.write_text(txt, &format);    
    // format.align = TextAlignament::Center;
    // format.x = 40;
    // format.width = Some(30);
    // s.write_text(txt, &format);
    // format.align = TextAlignament::Right;
    // format.x = 78;
    // format.width = Some(7);
    // s.write_text(txt, &format);

    s.print();
    //assert_eq!(s.compute_hash(), 0x5C5090CB807A653);
}