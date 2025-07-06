use appcui_proc_macro::*;

use crate::graphics::text_format::TextFormatBuilder;
use crate::graphics::Point;
use crate::graphics::Rect;
use crate::graphics::Size;
use crate::graphics::WrapType;
use crate::prelude::SpecialChar;

use super::CharAttribute;
use super::CharFlags;
use super::Character;
use super::Color;
use super::LineType;
use super::Surface;
use super::SurfaceTester;
use super::TextAlignament;

#[test]
fn check_clear() {
    let mut s = SurfaceTester::new(20, 5);
    s.clear(Character::new('x', Color::White, Color::Black, CharFlags::None));
    //s.print();
    assert_eq!(s.compute_hash(), 0x19B0E1632DAE6325);
}
#[test]
fn check_clear_with_clipping() {
    let mut s = SurfaceTester::new(40, 10);
    s.clear(Character::new('x', Color::White, Color::Black, CharFlags::None));
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
    s.clear(Character::new('.', Color::White, Color::Black, CharFlags::None));
    s.fill_rect(Rect::new(2, 2, 4, 4), Character::new('@', Color::Aqua, Color::Red, CharFlags::Bold));
    //s.print();
    assert_eq!(s.compute_hash(), 0x9E357B7ADEDEB720);
    s.fill_rect(Rect::with_size(4, 1, 10, 2), Character::with_char('X'));
    //s.print();
    assert_eq!(s.compute_hash(), 0xD897421A927A1A1);
}
#[test]
fn check_draw_rect() {
    let mut s = SurfaceTester::new(40, 10);
    s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
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
    s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
    s.draw_rect(Rect::with_size(1, 1, 20, 5), LineType::Double, CharAttribute::default());
    //s.print();
    assert_eq!(s.compute_hash(), 0xB2DEA1E9B27FD8B1);
}

#[test]
fn check_draw_vertical_line() {
    let mut s = SurfaceTester::new(15, 7);
    s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
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
    s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
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
    s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
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
    s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
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
    s2.clear(Character::new('X', Color::Yellow, Color::Black, CharFlags::None));
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
    s.write_char(1, 1, Character::new('A', Color::White, Color::Red, CharFlags::None));
    s.write_char(2, 1, Character::new('A', Color::Red, Color::Black, CharFlags::None));
    s.write_char(3, 1, Character::new('B', Color::Yellow, Color::Blue, CharFlags::None));
    s.write_char(4, 1, Character::new('B', Color::Blue, Color::Yellow, CharFlags::None));
    s.write_char(5, 1, Character::new('C', Color::Yellow, Color::DarkBlue, CharFlags::None));
    s.write_char(6, 1, Character::new('B', Color::Black, Color::Magenta, CharFlags::None));
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
fn check_write_string_single_line() {
    let mut s = SurfaceTester::new(40, 10);
    s.write_string(1, 1, "text", CharAttribute::with_color(Color::White, Color::DarkRed), false);
    s.set_clip(6, 1, 10, 1);
    s.set_origin(6, 1);
    s.write_string(0, 0, "A longer text", CharAttribute::with_color(Color::White, Color::DarkRed), false);
    s.reset_clip();
    s.write_string(0, 2, "A longer text", CharAttribute::with_color(Color::White, Color::DarkBlue), false);
    s.set_clip(6, 4, 10, 4);
    s.set_origin(6, 4);
    s.write_string(-2, 0, "A longer text", CharAttribute::with_color(Color::White, Color::Magenta), false);
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
    assert_eq!(s.size(), Size::new(20, 5));
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
    s.draw_vertical_line(30, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    let format = TextFormatBuilder::new()
        .position(30, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Left)
        .build();
    s.write_text("Left Align at 30", &format);
    let format = TextFormatBuilder::new()
        .position(30, 3)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkGreen))
        .align(TextAlignament::Center)
        .build();
    s.write_text("Centered! at 30", &format);
    let format = TextFormatBuilder::new()
        .position(30, 5)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkBlue))
        .align(TextAlignament::Right)
        .build();
    s.write_text("Right align ends at 30", &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0x8DFA95B692742714);
}

#[test]
fn check_write_text_single_line_width() {
    let mut s = SurfaceTester::new(60, 7);
    s.draw_vertical_line(30, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    let format = TextFormatBuilder::new()
        .position(30, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Left)
        .wrap_type(WrapType::SingleLineWrap(6))
        .build();
    s.write_text("123456xxxxxxx", &format);
    let format = TextFormatBuilder::new()
        .position(30, 3)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkGreen))
        .align(TextAlignament::Center)
        .wrap_type(WrapType::SingleLineWrap(6))
        .build();
    s.write_text("----123456----", &format);
    let format = TextFormatBuilder::new()
        .position(30, 5)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkBlue))
        .align(TextAlignament::Right)
        .wrap_type(WrapType::SingleLineWrap(6))
        .build();
    s.write_text("--------------------123456", &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0xC503745C2440B5F6);
}
#[test]
fn check_write_text_single_line_hot_key() {
    let mut s = SurfaceTester::new(60, 7);
    s.draw_vertical_line(30, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));

    let format = TextFormatBuilder::new()
        .position(30, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Left)
        .hotkey(CharAttribute::with_color(Color::Black, Color::Yellow), 4)
        .build();
    s.write_text("HotKey is 'E'", &format);
    let format = TextFormatBuilder::new()
        .position(30, 3)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkGreen))
        .align(TextAlignament::Center)
        .hotkey(CharAttribute::with_color(Color::Black, Color::Yellow), 0)
        .build();
    s.write_text("Centered (hotkey='C')", &format);
    let format = TextFormatBuilder::new()
        .position(30, 5)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkBlue))
        .align(TextAlignament::Right)
        .hotkey(CharAttribute::with_color(Color::Black, Color::Yellow), 20)
        .build();
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
    //let mut format = TextFormat::multi_line(2, 1, CharAttribute::with_color(Color::Yellow, Color::DarkRed), TextAlignament::Left);
    let mut format = TextFormatBuilder::new()
        .position(2, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Left)
        .wrap_type(WrapType::MultiLine)
        .build();
    s.write_text("This is a\nmulti-line text\nwith 4 lines\nall left-aligned !", &format);
    format.set_align(TextAlignament::Center);
    format.set_position(40, 1);
    s.write_text("This is a\nmulti-line text\nwith 5 lines\n\nall centered !", &format);
    format.set_align(TextAlignament::Right);
    format.set_position(78, 1);
    s.write_text("This is a\nmulti-line text\n\nwith 6 lines\n\nall alligned to the right", &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0x5CA9237E8FF59BAF);
}

#[test]
fn check_write_text_multi_line_no_wrap_hot_key() {
    let mut s = SurfaceTester::new(80, 7);
    s.draw_vertical_line(2, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(40, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(78, 0, 7, LineType::Double, CharAttribute::with_fore_color(Color::White));
    let format = TextFormatBuilder::new()
        .position(2, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkBlue))
        .align(TextAlignament::Left)
        .wrap_type(WrapType::MultiLine)
        .hotkey(CharAttribute::with_color(Color::Yellow, Color::DarkRed), 11)
        .build();
    s.write_text(
        "This is a\nmulti-line text\nwith 5 lines\nall left-aligned !\nand with hot key 'u'",
        &format,
    );
    let format = TextFormatBuilder::new()
        .position(40, 1)
        .attribute(CharAttribute::with_color(Color::White, Color::Gray))
        .align(TextAlignament::Center)
        .wrap_type(WrapType::MultiLine)
        .hotkey(CharAttribute::with_color(Color::Yellow, Color::DarkRed), 26)
        .build();
    s.write_text(
        "This is a\nmulti-line text\nwith 5 lines\nall centered at y=40\nand with hot key 'w'",
        &format,
    );
    let format = TextFormatBuilder::new()
        .position(78, 1)
        .attribute(CharAttribute::with_color(Color::White, Color::DarkGreen))
        .align(TextAlignament::Right)
        .wrap_type(WrapType::MultiLine)
        .hotkey(CharAttribute::with_color(Color::Yellow, Color::DarkRed), 75)
        .build();
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
    s.draw_vertical_line(2, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(40, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));
    s.draw_vertical_line(78, 0, 10, LineType::Double, CharAttribute::with_fore_color(Color::White));

    let format = TextFormatBuilder::new()
        .position(2, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Left)
        .wrap_type(WrapType::CharacterWrap(10))
        .build();

    s.write_text(txt, &format);

    let format = TextFormatBuilder::new()
        .position(40, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Center)
        .wrap_type(WrapType::CharacterWrap(30))
        .build();

    s.write_text(txt, &format);

    let format = TextFormatBuilder::new()
        .position(78, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Right)
        .wrap_type(WrapType::CharacterWrap(7))
        .build();

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
    let format = TextFormatBuilder::new()
        .position(2, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Left)
        .wrap_type(WrapType::CharacterWrap(10))
        .build();
    s.write_text(txt, &format);

    let format = TextFormatBuilder::new()
        .position(40, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Center)
        .wrap_type(WrapType::CharacterWrap(30))
        .build();
    s.write_text(txt, &format);

    let format = TextFormatBuilder::new()
        .position(78, 1)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkRed))
        .align(TextAlignament::Right)
        .wrap_type(WrapType::CharacterWrap(7))
        .build();
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
    let format = TextFormatBuilder::new()
        .position(2, 1)
        .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
        .align(TextAlignament::Left)
        .wrap_type(WrapType::CharacterWrap(10))
        .hotkey(CharAttribute::with_color(Color::Yellow, Color::DarkRed), 17)
        .build();
    s.write_text("This is a line\nthat will be wrapped on multiple lines\n\nHot key is 'a'", &format);

    let format = TextFormatBuilder::new()
        .position(40, 1)
        .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
        .align(TextAlignament::Center)
        .wrap_type(WrapType::CharacterWrap(30))
        .hotkey(CharAttribute::with_color(Color::Yellow, Color::DarkRed), 28)
        .build();
    s.write_text("This is a line\nthat will be wrapped on multiple lines\n\nHot key is 'w'", &format);

    let format = TextFormatBuilder::new()
        .position(78, 1)
        .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
        .align(TextAlignament::Right)
        .wrap_type(WrapType::CharacterWrap(15))
        .hotkey(CharAttribute::with_color(Color::Yellow, Color::DarkRed), 67)
        .build();
    s.write_text("This is a line\nthat will be wrapped on multiple lines\n\nHot key is 'x'", &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0x3CE81721E1BB64FA);
}

fn print_word_wrapped(txt: &str, width: u32, height: u32, hotkey_pos: usize) -> SurfaceTester {
    let mut s = SurfaceTester::new(width, height);
    s.write_string(0, 0, "Hotkey is: ", CharAttribute::with_color(Color::Yellow, Color::Red), false);
    let ch = txt.chars().nth(hotkey_pos).unwrap();
    s.write_char(12, 0, Character::new(ch, Color::White, Color::DarkBlue, CharFlags::None));

    let mut print = |x: i32, w: u16| {
        let format = TextFormatBuilder::new()
            .position(x, 1)
            .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
            .align(TextAlignament::Left)
            .wrap_type(WrapType::WordWrap(w))
            .hotkey(CharAttribute::with_color(Color::White, Color::DarkGreen), hotkey_pos as u32)
            .build();
        s.write_text(txt, &format);
    };

    print(2, 10);
    print(14, 11);
    print(27, 12);
    print(41, 13);
    print(56, 14);
    print(72, 6);
    print(80, 3);
    print(85, 1);

    s
}
#[test]
fn check_write_text_multi_line_word_wrap_1() {
    let s = print_word_wrapped(
        "This is     a line that       will be wrapped    on multiple lines on a given long-character width",
        90,
        20,
        16,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0x4F03DE6BBFE0E049);
}
#[test]
fn check_write_text_multi_line_word_wrap_2() {
    let s = print_word_wrapped("+abc+ 123456789   1+2+3+4+5+6+7+8 abc123=+-*123abc", 90, 20, 12);
    //s.print();
    assert_eq!(s.compute_hash(), 0xB123507C204CAE78);
}
#[test]
fn check_write_text_multi_line_word_wrap_3() {
    let s = print_word_wrapped("Hello world, from\nRust\n\ncode\n 1. one\n 2. two\n 3. a really long line", 90, 20, 18);
    //s.print();
    assert_eq!(s.compute_hash(), 0x6FBCFB162D486AEE);
}

#[test]
fn check_write_text_multi_line_word_wrap_4() {
    let s = print_word_wrapped("Hello world, from\nRust\n\ncode\n 1. one\n 2. two\n 3. a really long line", 90, 20, 42);
    //s.print();
    assert_eq!(s.compute_hash(), 0x1C1D4F88EB2DFAEB);
}

#[test]
fn check_write_text_multi_line_word_wrap_aligned() {
    let txt = "This is     a line that       will be wrapped    on multiple lines on a given long-character width";
    let mut s = SurfaceTester::new(90, 15);
    s.write_string(0, 0, "Hotkey is: ", CharAttribute::with_color(Color::Yellow, Color::Red), false);
    let ch = txt.chars().nth(16).unwrap();
    s.write_char(12, 0, Character::new(ch, Color::White, Color::DarkBlue, CharFlags::None));

    let format = TextFormatBuilder::new()
        .position(2, 1)
        .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
        .align(TextAlignament::Left)
        .wrap_type(WrapType::WordWrap(12))
        .hotkey(CharAttribute::with_color(Color::White, Color::DarkGreen), 16)
        .build();
    s.write_text(txt, &format);

    let format = TextFormatBuilder::new()
        .position(45, 1)
        .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
        .align(TextAlignament::Center)
        .wrap_type(WrapType::WordWrap(20))
        .hotkey(CharAttribute::with_color(Color::White, Color::DarkGreen), 16)
        .build();
    s.write_text(txt, &format);

    let format = TextFormatBuilder::new()
        .position(88, 1)
        .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
        .align(TextAlignament::Right)
        .wrap_type(WrapType::WordWrap(15))
        .hotkey(CharAttribute::with_color(Color::White, Color::DarkGreen), 16)
        .build();
    s.write_text(txt, &format);

    //s.print();
    assert_eq!(s.compute_hash(), 0x70526C060A7E28C6);
}

#[test]
fn check_write_text_multi_line_word_wrap_aligned_v2() {
    let txt = "This is     a line that       will be wrapped    on multiple lines on a given long-character width";
    let mut s = SurfaceTester::new(100, 15);
    s.set_origin(2, 2);
    s.write_string(0, 0, "Hotkey is: ", CharAttribute::with_color(Color::Yellow, Color::Red), false);
    let ch = txt.chars().nth(16).unwrap();
    s.write_char(12, 0, Character::new(ch, Color::White, Color::DarkBlue, CharFlags::None));

    let format = TextFormatBuilder::new()
        .position(2, 1)
        .align(TextAlignament::Left)
        .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
        .wrap_type(WrapType::WordWrap(12))
        .hotkey(CharAttribute::with_color(Color::White, Color::DarkGreen), 16)
        .build();
    s.write_text(txt, &format);

    let format = TextFormatBuilder::new()
        .position(45, 1)
        .align(TextAlignament::Center)
        .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
        .wrap_type(WrapType::WordWrap(20))
        .hotkey(CharAttribute::with_color(Color::White, Color::DarkGreen), 16)
        .build();
    s.write_text(txt, &format);

    let format = TextFormatBuilder::new()
        .position(88, 1)
        .align(TextAlignament::Right)
        .attribute(CharAttribute::with_color(Color::Black, Color::Silver))
        .wrap_type(WrapType::WordWrap(15))
        .hotkey(CharAttribute::with_color(Color::White, Color::DarkGreen), 16)
        .build();
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
    assert_eq!(char!("X,Red,Green"), Character::new('X', Color::Red, Color::Green, CharFlags::None));
    assert_eq!(char!("'+',white,pinK"), Character::new('+', Color::White, Color::Pink, CharFlags::None));
    assert_eq!(char!("A,g,r"), Character::new('A', Color::Green, Color::Red, CharFlags::None));
    assert_eq!(
        char!("'!',back=dr"),
        Character::new('!', Color::Transparent, Color::DarkRed, CharFlags::None)
    );
    assert_eq!(char!("'',fore=w"), Character::new(0, Color::White, Color::Transparent, CharFlags::None));
    assert_eq!(
        char!("char=X,fore=Olive,back=Aqua"),
        Character::new('X', Color::Olive, Color::Aqua, CharFlags::None)
    );
    assert_eq!(char!("B"), Character::new('B', Color::Transparent, Color::Transparent, CharFlags::None));
    assert_eq!(
        char!("B,attr=Bold"),
        Character::new('B', Color::Transparent, Color::Transparent, CharFlags::Bold)
    );
    assert_eq!(
        char!("X,attr=Italic+Bold"),
        Character::new('X', Color::Transparent, Color::Transparent, CharFlags::Bold | CharFlags::Italic)
    );
    assert_eq!(
        char!("Y,attr=[Italic,Underline]"),
        Character::new('Y', Color::Transparent, Color::Transparent, CharFlags::Underline | CharFlags::Italic)
    );
    assert_eq!(
        char!("code=41"),
        Character::new('A', Color::Transparent, Color::Transparent, CharFlags::None)
    );
    assert_eq!(char!("->"), Character::with_char(SpecialChar::ArrowRight));
    assert_eq!(char!("<->"), Character::with_char(SpecialChar::ArrowLeftRight));
    assert_eq!(char!("<-"), Character::with_char(SpecialChar::ArrowLeft));
    assert_eq!(char!("'=='"), Character::with_char(SpecialChar::BoxHorizontalDoubleLine));
    assert_eq!(char!("||"), Character::with_char(SpecialChar::BoxVerticalDoubleLine));
    assert_eq!(char!("|_"), Character::with_char(SpecialChar::BoxBottomLeftCornerSingleLine));
    assert_eq!(char!("_|"), Character::with_char(SpecialChar::BoxBottomRightCornerSingleLine));
}

#[test]
fn check_charattr_macro() {
    assert_eq!(charattr!("Red,Green"), CharAttribute::new(Color::Red, Color::Green, CharFlags::None));
    assert_eq!(charattr!("white,pinK"), CharAttribute::new(Color::White, Color::Pink, CharFlags::None));
    assert_eq!(charattr!("g,r"), CharAttribute::new(Color::Green, Color::Red, CharFlags::None));
    assert_eq!(charattr!("g"), CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None));
    assert_eq!(
        charattr!("g,attr: Bold+Italic"),
        CharAttribute::new(Color::Green, Color::Transparent, CharFlags::Bold | CharFlags::Italic)
    );
    assert_eq!(
        charattr!("?,r,attr: Bold+Italic"),
        CharAttribute::new(Color::Transparent, Color::Red, CharFlags::Bold | CharFlags::Italic)
    );
    assert_eq!(
        charattr!("?,r,attr: Bold+Italic+StrikeThrough"),
        CharAttribute::new(
            Color::Transparent,
            Color::Red,
            CharFlags::Bold | CharFlags::Italic | CharFlags::StrikeThrough
        )
    );
    assert_eq!(
        charattr!("?,r,attr: Bold+DoubleUnderline+StrikeThrough"),
        CharAttribute::new(
            Color::Transparent,
            Color::Red,
            CharFlags::Bold | CharFlags::DoubleUnderline | CharFlags::StrikeThrough
        )
    );
    assert_eq!(
        charattr!("?,r,attr: Bold+CurlyUnderline+StrikeThrough"),
        CharAttribute::new(
            Color::Transparent,
            Color::Red,
            CharFlags::Bold | CharFlags::CurlyUnderline | CharFlags::StrikeThrough
        )
    );
    assert_eq!(
        charattr!("aqua,pink,attr: Bold+DottedUnderline+StrikeThrough"),
        CharAttribute::new(
            Color::Aqua,
            Color::Pink,
            CharFlags::Bold | CharFlags::DottedUnderline | CharFlags::StrikeThrough
        )
    );
}

#[test]
fn check_size_reduce_by() {
    let s = Size::new(100, 100);
    let s = s.reduce_by(10);
    assert_eq!(s, Size::new(90, 90));
    let s = s.reduce_by(91);
    assert_eq!(s, Size::new(0, 0));
}

#[test]
fn check_create_charattributi() {
    let a = CharAttribute::new(Color::Red, Color::Green, CharFlags::Bold);
    assert_eq!(a.foreground, Color::Red);
    assert_eq!(a.background, Color::Green);
    assert_eq!(a.flags, CharFlags::Bold);

    let a = CharAttribute::with_color(Color::Blue, Color::White);
    assert_eq!(a.foreground, Color::Blue);
    assert_eq!(a.background, Color::White);
    assert_eq!(a.flags, CharFlags::None);

    let a = CharAttribute::with_fore_color(Color::Pink);
    assert_eq!(a.foreground, Color::Pink);
    assert_eq!(a.background, Color::Transparent);
    assert_eq!(a.flags, CharFlags::None);

    let a = CharAttribute::with_back_color(Color::DarkRed);
    assert_eq!(a.foreground, Color::Transparent);
    assert_eq!(a.background, Color::DarkRed);
    assert_eq!(a.flags, CharFlags::None);
}

#[test]
fn check_surface_read_char() {
    let mut s = SurfaceTester::new(10, 10);
    s.write_char(3, 3, char!("A,Red,Green,flags: Bold"));
    s.write_char(30, 30, char!("A,Red,Green,flags: Bold"));
    let c = s.char(3, 3);
    assert!(c.is_some());
    let c = c.unwrap();
    assert_eq!(c.code, 'A');
    assert_eq!(c.foreground, Color::Red);
    assert_eq!(c.background, Color::Green);
    assert_eq!(c.flags, CharFlags::Bold);

    assert_eq!(s.char(11, 11), None);
}

#[test]
fn check_fill_vertical_line_with_size() {
    let mut s = SurfaceTester::new(10, 10);
    s.fill_vertical_line_with_size(1, 1, 5, char!("A,Red,Green"));
    s.fill_vertical_line_with_size(3, 3, 5, char!("B,Red,Green"));
    s.fill_vertical_line_with_size(5, 5, 5, char!("C,Red,Green"));
    s.fill_vertical_line_with_size(7, 7, 5, char!("D,Red,Green"));
    s.fill_vertical_line_with_size(0, 0, 0, char!("E,Red,Green"));
    //s.print();
    assert_eq!(s.compute_hash(), 0xEACCDD8CC9B5BDE9);
}

#[test]
fn check_write_ascii_multi_line() {
    let mut s = SurfaceTester::new(10, 10);
    s.write_ascii(
        1,
        1,
        b"Hello \nWorld!\nfrom\nRust",
        CharAttribute::with_color(Color::White, Color::Black),
        true,
    );
    //s.print();
    assert_eq!(s.compute_hash(), 0xDE250FB0D21B6412);
}

#[test]
fn check_serialization_to_buffer() {
    let mut s = Surface::new(5, 2);
    s.write_string(0, 0, "Hello", CharAttribute::with_color(Color::White, Color::DarkRed), false);
    s.write_string(0, 1, "World", CharAttribute::new(Color::Green, Color::DarkRed, CharFlags::Bold), false);
    let mut buffer = Vec::new();
    s.serialize_to_buffer(&mut buffer);
    const RESULT: [u8; 92] = [
        // Magic
        83, 82, 70, // Version
        1,  // Width and Height
        5, 0, 0, 0, 2, 0, 0, 0, // Character data
        72, 0, 0, 0, 0, 0, 15, 4, 101, 0, 0, 0, 0, 0, 15, 4, 108, 0, 0, 0, 0, 0, 15, 4, 108, 0, 0, 0, 0, 0, 15, 4, 111, 0, 0, 0, 0, 0, 15, 4, 87, 0,
        0, 0, 1, 0, 10, 4, 111, 0, 0, 0, 1, 0, 10, 4, 114, 0, 0, 0, 1, 0, 10, 4, 108, 0, 0, 0, 1, 0, 10, 4, 100, 0, 0, 0, 1, 0, 10, 4,
    ];
    assert_eq!(buffer, RESULT);
}

#[test]
fn check_rect_contains() {
    let r = Rect::new(1, 2, 3, 4);
    assert!(r.contains(Point::new(1, 2)));
    assert!(r.contains(Point::new(2, 3)));
    assert!(r.contains(Point::new(3, 4)));
    assert!(r.contains(Point::new(1, 4)));
    assert!(r.contains(Point::new(3, 2)));
    assert!(!r.contains(Point::new(0, 2)));
    assert!(!r.contains(Point::new(4, 2)));
}

#[test]
fn check_rect_inflate_width() {
    let mut r = Rect::new(1, 2, 3, 4);
    r.inflate_width(1, 1, 1, 1);
    assert_eq!(r, Rect::new(0, 1, 4, 5));
    let mut r = Rect::new(1, 2, 3, 4);
    r.inflate_width(-1, -1, -1, -1);
    assert_eq!(r, Rect::new(2, 3, 2, 3));
    r.inflate_width(-1, -1, -1, -1);
    assert_eq!(r, Rect::new(2, 3, 2, 3));
    r.inflate_width(-1, -1, -1, -1);
    assert_eq!(r, Rect::new(2, 3, 2, 3));
}

#[test]
fn check_rect_top_left() {
    let r = Rect::new(1, 2, 3, 4);
    assert_eq!(r.top_left(), Point::new(1, 2));
}

#[test]
fn check_rect_top_right() {
    let r = Rect::new(1, 2, 3, 4);
    assert_eq!(r.top_right(), Point::new(3, 2));
}

#[test]
fn check_rect_bottom_right() {
    let r = Rect::new(1, 2, 3, 4);
    assert_eq!(r.bottom_right(), Point::new(3, 4));
}

#[test]
fn check_rect_bottom_left() {
    let r = Rect::new(1, 2, 3, 4);
    assert_eq!(r.bottom_left(), Point::new(1, 4));
}

#[test]
fn check_rect_center() {
    let r = Rect::new(1, 1, 5, 5);
    assert_eq!(r.center(), Point::new(3, 3));
}

#[test]
fn check_color_contrast() {
    assert_eq!(Color::Black.contrast_color(), Color::White);
    assert_eq!(Color::White.contrast_color(), Color::Black);
    assert_eq!(Color::Red.contrast_color(), Color::White);
    assert_eq!(Color::Green.contrast_color(), Color::Black);
    assert_eq!(Color::Blue.contrast_color(), Color::White);
    assert_eq!(Color::Yellow.contrast_color(), Color::Black);
    assert_eq!(Color::Magenta.contrast_color(), Color::White);
    assert_eq!(Color::Aqua.contrast_color(), Color::Black);
    assert_eq!(Color::Gray.contrast_color(), Color::White);
    assert_eq!(Color::Silver.contrast_color(), Color::Black);
    assert_eq!(Color::Transparent.contrast_color(), Color::Transparent);
    assert_eq!(Color::DarkRed.contrast_color(), Color::White);
    assert_eq!(Color::DarkGreen.contrast_color(), Color::White);
    assert_eq!(Color::DarkBlue.contrast_color(), Color::White);
    assert_eq!(Color::Transparent.contrast_color(), Color::Transparent);
    #[cfg(feature = "TRUE_COLORS")]
    {
        assert_eq!(Color::RGB(1, 1, 1).contrast_color(), Color::White);
        assert_eq!(Color::RGB(254, 254, 254).contrast_color(), Color::Black);
    }
}

#[test]
fn check_color_inverse() {
    assert_eq!(Color::Black.inverse_color(), Color::White);
    assert_eq!(Color::DarkBlue.inverse_color(), Color::Yellow);
    assert_eq!(Color::DarkGreen.inverse_color(), Color::Pink);
    assert_eq!(Color::Teal.inverse_color(), Color::Red);
    assert_eq!(Color::DarkRed.inverse_color(), Color::Aqua);
    assert_eq!(Color::Magenta.inverse_color(), Color::Green);
    assert_eq!(Color::Olive.inverse_color(), Color::Blue);
    assert_eq!(Color::Silver.inverse_color(), Color::Gray);
    assert_eq!(Color::Gray.inverse_color(), Color::Silver);
    assert_eq!(Color::Blue.inverse_color(), Color::Olive);
    assert_eq!(Color::Green.inverse_color(), Color::Magenta);
    assert_eq!(Color::Aqua.inverse_color(), Color::DarkRed);
    assert_eq!(Color::Red.inverse_color(), Color::Teal);
    assert_eq!(Color::Pink.inverse_color(), Color::DarkGreen);
    assert_eq!(Color::Yellow.inverse_color(), Color::DarkBlue);
    assert_eq!(Color::White.inverse_color(), Color::Black);
    assert_eq!(Color::Transparent.inverse_color(), Color::Transparent);
    #[cfg(feature = "TRUE_COLORS")]
    {
        assert_eq!(Color::RGB(1, 1, 1).inverse_color(), Color::RGB(254, 254, 254));
        assert_eq!(Color::RGB(254, 254, 254).inverse_color(), Color::RGB(1, 1, 1));
    }
}

#[test]
fn check_color_as_color_index() {
    assert_eq!(Color::Black.as_color_index(), 0);
    assert_eq!(Color::DarkBlue.as_color_index(), 1);
    assert_eq!(Color::DarkGreen.as_color_index(), 2);
    assert_eq!(Color::Teal.as_color_index(), 3);
    assert_eq!(Color::DarkRed.as_color_index(), 4);
    assert_eq!(Color::Magenta.as_color_index(), 5);
    assert_eq!(Color::Olive.as_color_index(), 6);
    assert_eq!(Color::Silver.as_color_index(), 7);
    assert_eq!(Color::Gray.as_color_index(), 8);
    assert_eq!(Color::Blue.as_color_index(), 9);
    assert_eq!(Color::Green.as_color_index(), 10);
    assert_eq!(Color::Aqua.as_color_index(), 11);
    assert_eq!(Color::Red.as_color_index(), 12);
    assert_eq!(Color::Pink.as_color_index(), 13);
    assert_eq!(Color::Yellow.as_color_index(), 14);
    assert_eq!(Color::White.as_color_index(), 15);
    assert_eq!(Color::Transparent.as_color_index(), 16);
}
