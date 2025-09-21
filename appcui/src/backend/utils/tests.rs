use crate::backend::utils::{AnsiFlags, AnsiFormatter};
use crate::graphics::Color;
use crate::prelude::{CharFlags, Point, Surface};
use crate::prelude::*;

#[test]
fn check_ansi_methods() {
    let mut a = AnsiFormatter::new(128, AnsiFlags::None);
    a.enable_mouse_events();
    assert_eq!(a.text(),"\x1b[?1000h\x1b[?1002h\x1b[?1003h\x1b[?1006h");
    a.clear();
    a.disable_mouse_events();
    assert_eq!(a.text(),"\x1b[?1000l\x1b[?1002l\x1b[?1003l\x1b[?1006l");
    a.clear();
    a.write_char('a');
    assert_eq!(a.text(),"a");
    a.clear();
    a.write_string("Hello");
    assert_eq!(a.text(),"Hello");
    a.clear();
    a.set_foreground_color(Color::Red);
    assert_eq!(a.text(),"\x1b[38;2;255;0;0m");
    a.clear();
    a.set_background_color(Color::Blue);
    assert_eq!(a.text(),"\x1b[48;2;0;0;255m");
    a.clear();
    a.set_foreground_color(Color::Green);
    assert_eq!(a.text(),"\x1b[38;2;0;255;0m");
    a.clear();
    a.set_background_color(Color::Yellow);
    assert_eq!(a.text(),"\x1b[48;2;255;255;0m");
    a.clear();
    a.set_foreground_color(Color::Magenta);
    assert_eq!(a.text(),"\x1b[38;2;128;0;128m");
    a.clear();
    a.set_foreground_color(Color::White);
    assert_eq!(a.text(),"\x1b[38;2;255;255;255m");
    a.clear();
    a.set_background_color(Color::Black);
    assert_eq!(a.text(),"\x1b[48;2;0;0;0m");
    a.clear();
    a.set_foreground_color(Color::Gray);
    assert_eq!(a.text(),"\x1b[38;2;128;128;128m");
    a.clear();
    a.set_background_color(Color::Silver);
    assert_eq!(a.text(),"\x1b[48;2;196;196;196m");
    a.clear();
    a.set_foreground_color(Color::DarkRed);
    assert_eq!(a.text(),"\x1b[38;2;128;0;0m");
    a.clear();
    a.set_background_color(Color::DarkBlue);
    assert_eq!(a.text(),"\x1b[48;2;0;0;128m");
    a.clear();
    a.set_foreground_color(Color::DarkGreen);
    assert_eq!(a.text(),"\x1b[38;2;0;128;0m");
    a.clear();
    a.set_background_color(Color::Teal);
    assert_eq!(a.text(),"\x1b[48;2;0;128;128m");
    a.clear();
    a.set_foreground_color(Color::Olive);
    assert_eq!(a.text(),"\x1b[38;2;128;128;0m");
    a.clear();
    a.set_background_color(Color::Pink);
    assert_eq!(a.text(),"\x1b[48;2;255;0;255m");
    a.clear();
    a.set_foreground_color(Color::Aqua);
    assert_eq!(a.text(),"\x1b[38;2;0;255;255m");
    a.clear();
    a.set_background_color(Color::DarkRed);
    assert_eq!(a.text(),"\x1b[48;2;128;0;0m");
    a.clear();
    a.set_foreground_color(Color::DarkBlue);
    assert_eq!(a.text(),"\x1b[38;2;0;0;128m");
    a.clear();
    a.set_background_color(Color::DarkGreen);
    assert_eq!(a.text(),"\x1b[48;2;0;128;0m");
    a.clear();
    a.set_foreground_color(Color::Teal);
    assert_eq!(a.text(),"\x1b[38;2;0;128;128m");
    a.clear();
    a.set_background_color(Color::Olive);
    assert_eq!(a.text(),"\x1b[48;2;128;128;0m");
    a.clear();
    a.set_foreground_color(Color::Red);
    a.set_background_color(Color::Blue);
    assert_eq!(a.text(),"\x1b[38;2;255;0;0m\x1b[48;2;0;0;255m");
    a.clear();
    a.set_foreground_color(Color::Green);
    a.set_background_color(Color::Yellow);
    assert_eq!(a.text(),"\x1b[38;2;0;255;0m\x1b[48;2;255;255;0m");
    a.clear();
    a.set_foreground_color(Color::Magenta);
    a.set_background_color(Color::Aqua);
    assert_eq!(a.text(),"\x1b[38;2;128;0;128m\x1b[48;2;0;255;255m");
    a.clear();
    a.set_foreground_color(Color::White);
    a.set_background_color(Color::Black);
    assert_eq!(a.text(),"\x1b[38;2;255;255;255m\x1b[48;2;0;0;0m");
}

#[test]
fn check_ansi_cursor_methods() {
    let mut a = AnsiFormatter::new(128, AnsiFlags::None);
    a.hide_cursor();
    assert_eq!(a.text(),"\x1b[?25l");
    a.clear();
    a.show_cursor();
    assert_eq!(a.text(),"\x1b[?25h");
    a.clear();
    a.set_cursor_position(1,1);
    assert_eq!(a.text(),"\x1b[2;2H");
    a.clear();
    a.set_cursor_position(35,14);
    assert_eq!(a.text(),"\x1b[15;36H");
    a.clear();
    a.set_cursor_position(-2,-3);
    assert_eq!(a.text(),"\x1b[-2;-1H");
}

#[test]
fn check_ansi_char_flags() {
    let mut a = AnsiFormatter::new(128, AnsiFlags::None);
    a.update_char_flags(CharFlags::Bold,CharFlags::None);
    a.update_char_flags(CharFlags::None, CharFlags::Bold);
    assert_eq!(a.text(),"\x1b[1m\x1b[22m");

    a.clear();
    a.update_char_flags(CharFlags::Italic, CharFlags::None);
    a.update_char_flags(CharFlags::None, CharFlags::Italic);
    assert_eq!(a.text(),"\x1b[3m\x1b[23m");

    a.clear();
    a.update_char_flags(CharFlags::Underline, CharFlags::None);
    a.update_char_flags(CharFlags::None, CharFlags::Underline);
    assert_eq!(a.text(),"\x1b[4m\x1b[24m");

    a.clear();
    a.update_char_flags(CharFlags::DoubleUnderline, CharFlags::None);
    a.update_char_flags(CharFlags::None, CharFlags::DoubleUnderline);
    assert_eq!(a.text(),"\x1b[21m\x1b[24m");

    a.clear();
    a.update_char_flags(CharFlags::CurlyUnderline, CharFlags::None);
    a.update_char_flags(CharFlags::None, CharFlags::CurlyUnderline);
    assert_eq!(a.text(),"\x1b[4:3m\x1b[24m");

    a.clear();
    a.update_char_flags(CharFlags::DottedUnderline, CharFlags::None);
    a.update_char_flags(CharFlags::None, CharFlags::DottedUnderline);
    assert_eq!(a.text(),"\x1b[4:4m\x1b[24m");

    a.clear();
    a.update_char_flags(CharFlags::StrikeThrough, CharFlags::None);
    a.update_char_flags(CharFlags::None, CharFlags::StrikeThrough);
    assert_eq!(a.text(),"\x1b[9m\x1b[29m");

}

#[test]
fn check_ansi_renderer() {
    let mut a = AnsiFormatter::new(128, AnsiFlags::None);
    let mut s = Surface::new(5,1);
    s.write_ascii(0, 0, b"Hello", charattr!("red, blue"), false);
    assert_eq!(s.chars[0].foreground, Color::Red);
    assert_eq!(s.chars[0].background, Color::Blue);
    assert_eq!(s.chars[0].code, 'H');
    assert_eq!(s.chars[1].foreground, Color::Red);
    assert_eq!(s.chars[1].background, Color::Blue);
    assert_eq!(s.chars[1].code, 'e');
    assert_eq!(s.chars[2].code, 'l');
    assert_eq!(s.chars[2].foreground, Color::Red);
    assert_eq!(s.chars[2].background, Color::Blue);
    assert_eq!(s.chars[2].code, 'l');
    assert_eq!(s.chars[3].foreground, Color::Red);
    assert_eq!(s.chars[3].background, Color::Blue);
    assert_eq!(s.chars[3].code, 'l');
    assert_eq!(s.chars[4].code, 'o');
    assert_eq!(s.chars[4].background, Color::Blue);
    assert_eq!(s.chars[4].foreground, Color::Red);
    a.render(&s, Point::ORIGIN);
    assert_eq!(a.text(),"\u{1b}[0m\u{1b}[?25l\u{1b}[1;1H\u{1b}[38;2;255;0;0m\u{1b}[48;2;0;0;255mHello\u{1b}[?25l");
}

#[test]
fn check_ansi_renderer_with_cursor() {
    let mut a = AnsiFormatter::new(128, AnsiFlags::None);
    let mut s = Surface::new(5,1);
    s.write_ascii(0, 0, b"Hello", charattr!("red, blue"), false);
    s.write_ascii(2, 0, b"ll", charattr!("pink, dg, flags:[Bold]"),false);
    s.set_cursor(2, 0);
    a.render(&s, Point::ORIGIN);
    assert_eq!(a.text(),"\u{1b}[0m\u{1b}[?25l\u{1b}[1;1H\u{1b}[38;2;255;0;0m\u{1b}[48;2;0;0;255mHe\u{1b}[38;2;255;0;255m\u{1b}[48;2;0;128;0m\u{1b}[1mll\u{1b}[38;2;255;0;0m\u{1b}[48;2;0;0;255m\u{1b}[22mo\u{1b}[1;3H\u{1b}[?25h");
}