use AppCUIProcMacro::*;

use crate::graphics::Size;
use crate::graphics::TextWrap;
use crate::graphics::Point;
use crate::graphics::Rect;
use crate::prelude::SpecialChar;

use super::CharAttribute;
use super::CharFlags;
use super::Character;
use super::Color;
use super::Image;
use super::image;
use super::LineType;
use super::Surface;
use super::TextAlignament;
use super::TextFormat;
use super::SurfaceTester;


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
        Rect::new(2, 2, 4, 4),
        Character::new('@', Color::Aqua, Color::Red, CharFlags::Bold),
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x9E357B7ADEDEB720);
    s.fill_rect(Rect::with_size(4, 1, 10, 2), Character::with_char('X'));
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
        Rect::new(2, 2, 10, 4),
        LineType::Single,
        CharAttribute::with_color(Color::Yellow, Color::Blue),
    );
    s.draw_rect(
        Rect::new(12, 1, 18, 5),
        LineType::Double,
        CharAttribute::with_color(Color::White, Color::Green),
    );
    s.draw_rect(
        Rect::new(20, 0, 28, 3),
        LineType::SingleThick,
        CharAttribute::with_color(Color::Aqua, Color::Black),
    );
    s.draw_rect(
        Rect::new(29, 0, 39, 3),
        LineType::Border,
        CharAttribute::with_color(Color::Aqua, Color::Black),
    );
    s.draw_rect(
        Rect::new(20, 4, 30, 8),
        LineType::Ascii,
        CharAttribute::with_color(Color::Green, Color::White),
    );
    s.draw_rect(
        Rect::new(31, 4, 38, 8),
        LineType::AsciiRound,
        CharAttribute::with_color(Color::Green, Color::White),
    );
    s.draw_rect(
        Rect::new(1, 6, 17, 9),
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
    s.draw_rect(
        Rect::with_size(1, 1, 20, 5),
        LineType::Double,
        CharAttribute::default(),
    );
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
    assert!(!s.cursor.is_visible());
    s.set_origin(3, 3);
    s.set_cursor(2, 2);
    assert!((s.cursor.x == 5) && (s.cursor.y == 5));
    s.set_cursor(-2, -2);
    assert!((s.cursor.x == 1) && (s.cursor.y == 1));
    s.set_clip(3, 3, 6, 6);
    s.set_cursor(-2, -2);
    assert!(!s.cursor.is_visible());
    s.set_cursor(4, 4);
    assert!(!s.cursor.is_visible());
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
        Rect::new(0, 0, 7, 5),
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
    s.write_char(
        1,
        1,
        Character::new('A', Color::White, Color::Red, CharFlags::None),
    );
    s.write_char(
        2,
        1,
        Character::new('A', Color::Red, Color::Black, CharFlags::None),
    );
    s.write_char(
        3,
        1,
        Character::new('B', Color::Yellow, Color::Blue, CharFlags::None),
    );
    s.write_char(
        4,
        1,
        Character::new('B', Color::Blue, Color::Yellow, CharFlags::None),
    );
    s.write_char(
        5,
        1,
        Character::new('C', Color::Yellow, Color::DarkBlue, CharFlags::None),
    );
    s.write_char(
        6,
        1,
        Character::new('B', Color::Black, Color::Magenta, CharFlags::None),
    );
    s.fill_rect(
        Rect::new(10, 1, 30, 3),
        Character::new(' ', Color::Yellow, Color::DarkBlue, CharFlags::None),
    );
    s.draw_rect(
        Rect::new(10, 1, 30, 3),
        LineType::Double,
        CharAttribute::with_color(Color::White, Color::DarkBlue),
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0xF47F25A9A2342269);
}

#[test]
fn check_draw_imge() {
    let mut s = SurfaceTester::new(40, 10);
    let i = Image::with_str(
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
        image::RenderMethod::SmallBlocks,
        image::Scale::NoScale,
    );
    s.draw_image(
        20,
        1,
        &i,
        image::RenderMethod::SmallBlocks,
        image::Scale::Scale50,
    );
    s.draw_image(
        30,
        1,
        &i,
        image::RenderMethod::SmallBlocks,
        image::Scale::Scale25,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0xFD04064498933DB);
    s.clear(Character::default());
    s.draw_image(
        0,
        0,
        &i,
        image::RenderMethod::LargeBlocks64Colors,
        image::Scale::NoScale,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x7BAAAA0605CBFA25);
    s.clear(Character::default());
    s.draw_image(
        0,
        0,
        &i,
        image::RenderMethod::GrayScale,
        image::Scale::NoScale,
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
    s.resize(Size::new(20, 5));
    assert_eq!(s.size(), Size::new(20,5));
    s.write_string(
        1,
        1,
        "Hello, world\nThis is a multi-line\nString",
        CharAttribute::with_color(Color::White, Color::DarkRed),
        true,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x5CA6952034D223D2);
    s.resize(Size::new(100, 30));
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
    s.draw_vertical_line(
        30,
        0,
        7,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );

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
    s.draw_vertical_line(
        30,
        0,
        7,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );

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
    s.draw_vertical_line(
        30,
        0,
        7,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );

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
    s.draw_vertical_line(
        2,
        0,
        7,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        40,
        0,
        7,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        78,
        0,
        7,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    let mut format = TextFormat::multi_line(
        2,
        1,
        CharAttribute::with_color(Color::Yellow, Color::DarkRed),
        TextAlignament::Left,
    );
    s.write_text(
        "This is a\nmulti-line text\nwith 4 lines\nall left-aligned !",
        &format,
    );
    format.align = TextAlignament::Center;
    format.x = 40;
    s.write_text(
        "This is a\nmulti-line text\nwith 5 lines\n\nall centered !",
        &format,
    );
    format.align = TextAlignament::Right;
    format.x = 78;
    s.write_text(
        "This is a\nmulti-line text\n\nwith 6 lines\n\nall alligned to the right",
        &format,
    );

    //s.print();
    assert_eq!(s.compute_hash(), 0x5CA9237E8FF59BAF);
}

#[test]
fn check_write_text_multi_line_no_wrap_how_key() {
    let mut s = SurfaceTester::new(80, 7);
    s.draw_vertical_line(
        2,
        0,
        7,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        40,
        0,
        7,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        78,
        0,
        7,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    let mut format = TextFormat::multi_line(
        2,
        1,
        CharAttribute::with_color(Color::Yellow, Color::DarkBlue),
        TextAlignament::Left,
    );
    format.hotkey_attr = Some(CharAttribute::with_color(Color::Yellow, Color::DarkRed));
    format.hotkey_pos = Some(11);
    s.write_text(
        "This is a\nmulti-line text\nwith 5 lines\nall left-aligned !\nand with hot key 'u'",
        &format,
    );
    format.align = TextAlignament::Center;
    format.x = 40;
    format.hotkey_pos = Some(26);
    format.char_attr = CharAttribute::with_color(Color::White, Color::Gray);
    s.write_text(
        "This is a\nmulti-line text\nwith 5 lines\nall centered at y=40\nand with hot key 'w'",
        &format,
    );
    format.align = TextAlignament::Right;
    format.x = 78;
    format.hotkey_pos = Some(75);
    format.char_attr = CharAttribute::with_color(Color::White, Color::DarkGreen);
    s.write_text(
        "This is a\nmulti-line text\nwith 6 lines\naligned to right\n\nand with hot key 'x'",
        &format,
    );

    //s.print();
    assert_eq!(s.compute_hash(), 0xCED1C065F1F2053B);
}

#[test]
fn check_write_text_multi_line_character_wrap() {
    let mut s = SurfaceTester::new(80, 10);
    let txt = "This is a line that will be wrapped on multiple lines on a given character width";
    s.draw_vertical_line(
        2,
        0,
        10,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        40,
        0,
        10,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        78,
        0,
        10,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
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
    s.draw_vertical_line(
        2,
        0,
        10,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        40,
        0,
        10,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        78,
        0,
        10,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
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
    s.draw_vertical_line(
        2,
        0,
        10,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        40,
        0,
        10,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
    s.draw_vertical_line(
        78,
        0,
        10,
        LineType::Double,
        CharAttribute::with_fore_color(Color::White),
    );
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
    s.write_text(
        "This is a line\nthat will be wrapped on multiple lines\n\nHot key is 'a'",
        &format,
    );
    format.align = TextAlignament::Center;
    format.x = 40;
    format.width = Some(30);
    format.hotkey_pos = Some(28);
    s.write_text(
        "This is a line\nthat will be wrapped on multiple lines\n\nHot key is 'w'",
        &format,
    );
    format.align = TextAlignament::Right;
    format.x = 78;
    format.width = Some(15);
    format.hotkey_pos = Some(67);
    s.write_text(
        "This is a line\nthat will be wrapped on multiple lines\n\nHot key is 'x'",
        &format,
    );

    //s.print();
    assert_eq!(s.compute_hash(), 0x3CE81721E1BB64FA);
}

fn print_word_wrapped(txt: &str, width: u32, height: u32, hotkey_pos: usize) -> SurfaceTester {
    let mut s = SurfaceTester::new(width, height);
    s.write_string(
        0,
        0,
        "Hotkey is: ",
        CharAttribute::with_color(Color::Yellow, Color::Red),
        false,
    );
    let ch = txt.chars().nth(hotkey_pos).unwrap();
    s.write_char(
        12,
        0,
        Character::new(ch, Color::White, Color::DarkBlue, CharFlags::None),
    );
    let mut format = TextFormat::multi_line_with_text_wrap(
        2,
        1,
        10,
        CharAttribute::with_color(Color::Black, Color::Silver),
        TextAlignament::Left,
        TextWrap::Word,
    );
    format.hotkey_attr = Some(CharAttribute::with_color(Color::White, Color::DarkGreen));
    format.hotkey_pos = Some(hotkey_pos);
    s.write_text(txt, &format);

    format.width = Some(11);
    format.x = 14;
    s.write_text(txt, &format);

    format.width = Some(12);
    format.x = 27;
    s.write_text(txt, &format);

    format.width = Some(13);
    format.x = 41;
    s.write_text(txt, &format);

    format.width = Some(14);
    format.x = 56;
    s.write_text(txt, &format);

    format.width = Some(6);
    format.x = 72;
    s.write_text(txt, &format);

    format.width = Some(3);
    format.x = 80;
    s.write_text(txt, &format);

    format.width = Some(1);
    format.x = 85;
    s.write_text(txt, &format);

    s
}
#[test]
fn check_write_text_multi_line_word_wrap_1() {
    let s = print_word_wrapped("This is     a line that       will be wrapped    on multiple lines on a given long-character width", 90, 20, 16);
    //s.print();
    assert_eq!(s.compute_hash(), 0x4F03DE6BBFE0E049);
}
#[test]
fn check_write_text_multi_line_word_wrap_2() {
    let s = print_word_wrapped(
        "+abc+ 123456789   1+2+3+4+5+6+7+8 abc123=+-*123abc",
        90,
        20,
        12,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0xB123507C204CAE78);
}
#[test]
fn check_write_text_multi_line_word_wrap_3() {
    let s = print_word_wrapped(
        "Hello world, from\nRust\n\ncode\n 1. one\n 2. two\n 3. a really long line",
        90,
        20,
        18,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x6FBCFB162D486AEE);
}

#[test]
fn check_write_text_multi_line_word_wrap_4() {
    let s = print_word_wrapped(
        "Hello world, from\nRust\n\ncode\n 1. one\n 2. two\n 3. a really long line",
        90,
        20,
        42,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x1C1D4F88EB2DFAEB);
}

#[test]
fn check_write_text_multi_line_word_wrap_aligned() {
    let txt = "This is     a line that       will be wrapped    on multiple lines on a given long-character width";
    let mut s = SurfaceTester::new(90, 15);
    s.write_string(
        0,
        0,
        "Hotkey is: ",
        CharAttribute::with_color(Color::Yellow, Color::Red),
        false,
    );
    let ch = txt.chars().nth(16).unwrap();
    s.write_char(
        12,
        0,
        Character::new(ch, Color::White, Color::DarkBlue, CharFlags::None),
    );
    let mut format = TextFormat::multi_line_with_text_wrap(
        2,
        1,
        12,
        CharAttribute::with_color(Color::Black, Color::Silver),
        TextAlignament::Left,
        TextWrap::Word,
    );
    format.hotkey_attr = Some(CharAttribute::with_color(Color::White, Color::DarkGreen));
    format.hotkey_pos = Some(16);
    s.write_text(txt, &format);

    format.width = Some(20);
    format.x = 45;
    format.align = TextAlignament::Center;
    s.write_text(txt, &format);

    format.width = Some(15);
    format.x = 88;
    format.align = TextAlignament::Right;
    s.write_text(txt, &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0x70526C060A7E28C6);
}

#[test]
fn check_write_text_multi_line_word_wrap_aligned_v2() {
    let txt = "This is     a line that       will be wrapped    on multiple lines on a given long-character width";
    let mut s = SurfaceTester::new(100, 15);
    s.set_origin(2, 2);
    s.write_string(
        0,
        0,
        "Hotkey is: ",
        CharAttribute::with_color(Color::Yellow, Color::Red),
        false,
    );
    let ch = txt.chars().nth(16).unwrap();
    s.write_char(
        12,
        0,
        Character::new(ch, Color::White, Color::DarkBlue, CharFlags::None),
    );
    let mut format = TextFormat::multi_line_with_text_wrap(
        2,
        1,
        12,
        CharAttribute::with_color(Color::Black, Color::Silver),
        TextAlignament::Left,
        TextWrap::Word,
    );
    format.hotkey_attr = Some(CharAttribute::with_color(Color::White, Color::DarkGreen));
    format.hotkey_pos = Some(16);
    s.write_text(txt, &format);

    format.width = Some(20);
    format.x = 45;
    format.align = TextAlignament::Center;
    s.write_text(txt, &format);

    format.width = Some(15);
    format.x = 88;
    format.align = TextAlignament::Right;
    s.write_text(txt, &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0xB7682D58B284C726);
}

#[test]
fn check_point() {
    let p = Point::default();
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
    let p = Point::new(1, 2);
    assert_eq!(p.x, 1);
    assert_eq!(p.y, 2);
}

#[test]
fn check_rect_new() {
    let r = Rect::new(1, 2, 3, 4);
    assert_eq!(r.left(), 1);
    assert_eq!(r.top(), 2);
    assert_eq!(r.right(), 3);
    assert_eq!(r.bottom(), 4);
    assert_eq!(r.width(), 3);
    assert_eq!(r.height(), 3);
    assert_eq!(r.center_x(), 2);
    assert_eq!(r.center_y(), 3);

    let r = Rect::new(3, 4, 1, 2);
    assert_eq!(r.left(), 1);
    assert_eq!(r.top(), 2);
    assert_eq!(r.right(), 3);
    assert_eq!(r.bottom(), 4);
    assert_eq!(r.width(), 3);
    assert_eq!(r.height(), 3);
    let r = Rect::new(1, 1, 1, 1);
    assert_eq!(r.left(), 1);
    assert_eq!(r.top(), 1);
    assert_eq!(r.right(), 1);
    assert_eq!(r.bottom(), 1);
    assert_eq!(r.width(), 1);
    assert_eq!(r.height(), 1);
    let r = Rect::new(1, 1, 9, 1);
    assert_eq!(r.left(), 1);
    assert_eq!(r.top(), 1);
    assert_eq!(r.right(), 9);
    assert_eq!(r.bottom(), 1);
    assert_eq!(r.width(), 9);
    assert_eq!(r.height(), 1);
}

#[test]
fn check_rect_with_size() {
    let r = Rect::with_size(1, 2, 5, 8);
    assert_eq!(r.left(), 1);
    assert_eq!(r.top(), 2);
    assert_eq!(r.right(), 5);
    assert_eq!(r.bottom(), 9);
    assert_eq!(r.width(), 5);
    assert_eq!(r.height(), 8);
    let r = Rect::with_size(1, 2, 0, 0);
    assert_eq!(r.left(), 1);
    assert_eq!(r.top(), 2);
    assert_eq!(r.right(), 1);
    assert_eq!(r.bottom(), 2);
    assert_eq!(r.width(), 1);
    assert_eq!(r.height(), 1);
}
#[test]
fn check_rect_with_alignament() {
    let r = Rect::with_alignament(10, 10, 4, 6, crate::graphics::rect::Alignament::TopLeft);
    assert_eq!(r.left(), 10);
    assert_eq!(r.top(), 10);
    assert_eq!(r.right(), 13);
    assert_eq!(r.bottom(), 15);
    assert_eq!(r.width(), 4);
    assert_eq!(r.height(), 6);
    let r = Rect::with_alignament(10, 10, 4, 6, crate::graphics::rect::Alignament::Top);
    assert_eq!(r.left(), 8);
    assert_eq!(r.top(), 10);
    assert_eq!(r.right(), 11);
    assert_eq!(r.bottom(), 15);
    assert_eq!(r.width(), 4);
    assert_eq!(r.height(), 6);
    let r = Rect::with_alignament(10, 10, 4, 6, crate::graphics::rect::Alignament::TopRight);
    assert_eq!(r.left(), 7);
    assert_eq!(r.top(), 10);
    assert_eq!(r.right(), 10);
    assert_eq!(r.bottom(), 15);
    assert_eq!(r.width(), 4);
    assert_eq!(r.height(), 6);
    let r = Rect::with_alignament(10, 10, 4, 6, crate::graphics::rect::Alignament::Right);
    assert_eq!(r.left(), 7);
    assert_eq!(r.top(), 7);
    assert_eq!(r.right(), 10);
    assert_eq!(r.bottom(), 12);
    assert_eq!(r.width(), 4);
    assert_eq!(r.height(), 6);
    let r = Rect::with_alignament(10, 10, 4, 6, crate::graphics::rect::Alignament::BottomRight);
    assert_eq!(r.left(), 7);
    assert_eq!(r.top(), 5);
    assert_eq!(r.right(), 10);
    assert_eq!(r.bottom(), 10);
    assert_eq!(r.width(), 4);
    assert_eq!(r.height(), 6);
    let r = Rect::with_alignament(10, 10, 4, 6, crate::graphics::rect::Alignament::Bottom);
    assert_eq!(r.left(), 8);
    assert_eq!(r.top(), 5);
    assert_eq!(r.right(), 11);
    assert_eq!(r.bottom(), 10);
    assert_eq!(r.width(), 4);
    assert_eq!(r.height(), 6);
    let r = Rect::with_alignament(10, 10, 4, 6, crate::graphics::rect::Alignament::BottomLeft);
    assert_eq!(r.left(), 10);
    assert_eq!(r.top(), 5);
    assert_eq!(r.right(), 13);
    assert_eq!(r.bottom(), 10);
    assert_eq!(r.width(), 4);
    assert_eq!(r.height(), 6);
    let r = Rect::with_alignament(10, 10, 4, 6, crate::graphics::rect::Alignament::Left);
    assert_eq!(r.left(), 10);
    assert_eq!(r.top(), 7);
    assert_eq!(r.right(), 13);
    assert_eq!(r.bottom(), 12);
    assert_eq!(r.width(), 4);
    assert_eq!(r.height(), 6);
    let r = Rect::with_alignament(10, 10, 4, 6, crate::graphics::rect::Alignament::Center);
    assert_eq!(r.left(), 8);
    assert_eq!(r.top(), 7);
    assert_eq!(r.right(), 11);
    assert_eq!(r.bottom(), 12);
    assert_eq!(r.width(), 4);
    assert_eq!(r.height(), 6);
}

#[test]
fn check_char_macro() {
    assert_eq!(char!("X,Red,Green"),Character::new('X',Color::Red, Color::Green,CharFlags::None));
    assert_eq!(char!("'+',white,pinK"),Character::new('+',Color::White, Color::Pink,CharFlags::None));
    assert_eq!(char!("A,g,r"),Character::new('A',Color::Green, Color::Red,CharFlags::None));
    assert_eq!(char!("'!',back=dr"),Character::new('!',Color::Transparent, Color::DarkRed,CharFlags::None));
    assert_eq!(char!("'',fore=w"),Character::new(0,Color::White, Color::Transparent,CharFlags::None));
    assert_eq!(char!("char=X,fore=Olive,back=Aqua"),Character::new('X',Color::Olive, Color::Aqua,CharFlags::None));
    assert_eq!(char!("B"),Character::new('B',Color::Transparent, Color::Transparent,CharFlags::None));
    assert_eq!(char!("B,attr=Bold"),Character::new('B',Color::Transparent, Color::Transparent,CharFlags::Bold));
    assert_eq!(char!("X,attr=Italic+Bold"),Character::new('X',Color::Transparent, Color::Transparent,CharFlags::Bold|CharFlags::Italic));
    assert_eq!(char!("Y,attr=[Italic,Underline]"),Character::new('Y',Color::Transparent, Color::Transparent,CharFlags::Underline|CharFlags::Italic));
    assert_eq!(char!("code=41"),Character::new('A',Color::Transparent, Color::Transparent,CharFlags::None));
    assert_eq!(char!("->"),Character::with_char(SpecialChar::ArrowRight));
    assert_eq!(char!("<->"),Character::with_char(SpecialChar::ArrowLeftRight));
    assert_eq!(char!("<-"),Character::with_char(SpecialChar::ArrowLeft));
    assert_eq!(char!("'=='"),Character::with_char(SpecialChar::BoxHorizontalDoubleLine));
    assert_eq!(char!("||"),Character::with_char(SpecialChar::BoxVerticalDoubleLine));
    assert_eq!(char!("|_"),Character::with_char(SpecialChar::BoxBottomLeftCornerSingleLine));
    assert_eq!(char!("_|"),Character::with_char(SpecialChar::BoxBottomRightCornerSingleLine));
}
