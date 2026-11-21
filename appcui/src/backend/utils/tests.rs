use crate::backend::utils::{AnsiFlags, AnsiFormatter};
use crate::backend::{ColorPalette, Palette};
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

const DEFAULT_PALETTE: [(Color, &str); 16] = [
    (Color::Black, "0;0;0"),
    (Color::DarkBlue, "0;0;128"),
    (Color::DarkGreen, "0;128;0"),
    (Color::Teal, "0;128;128"),
    (Color::DarkRed, "128;0;0"),
    (Color::Magenta, "128;0;128"),
    (Color::Olive, "128;128;0"),
    (Color::Silver, "196;196;196"),
    (Color::Gray, "128;128;128"),
    (Color::Blue, "0;0;255"),
    (Color::Green, "0;255;0"),
    (Color::Aqua, "0;255;255"),
    (Color::Red, "255;0;0"),
    (Color::Pink, "255;0;255"),
    (Color::Yellow, "255;255;0"),
    (Color::White, "255;255;255"),
];

/// AppCUI color order mapped to the ANSI slot each color must reprogram.
const BUILTIN_OSC_SLOTS: [(u8, u8, u8, u8); 16] = [
    (0, 0, 0, 0),         // Black
    (4, 0, 0, 128),       // DarkBlue
    (2, 0, 128, 0),       // DarkGreen
    (6, 0, 128, 128),     // Teal
    (1, 128, 0, 0),       // DarkRed
    (5, 128, 0, 128),     // Magenta
    (3, 128, 128, 0),     // Olive
    (7, 192, 192, 192),   // Silver
    (8, 128, 128, 128),   // Gray
    (12, 0, 0, 255),      // Blue
    (10, 0, 255, 0),      // Green
    (14, 0, 255, 255),    // Aqua
    (9, 255, 0, 0),       // Red
    (13, 255, 0, 255),    // Pink
    (11, 255, 255, 0),    // Yellow
    (15, 255, 255, 255),  // White
];

fn osc_palette(slots: &[(u8, u8, u8, u8)]) -> String {
    let mut out = String::new();
    for (idx, r, g, b) in slots {
        out.push_str(&format!("\x1b]4;{idx};rgb:{r:02x}/{g:02x}/{b:02x}\x1b\\"));
    }
    out
}

fn palette_reset_sequence() -> String {
    let mut out = String::new();
    for slot in 0..16 {
        out.push_str(&format!("\x1b]104;{slot}\x1b\\"));
    }
    out.push_str("\x1b]104\x1b\\");
    out
}

#[test]
fn check_default_palette_rgb() {
    for (color, rgb) in DEFAULT_PALETTE {
        let mut a = AnsiFormatter::new(64, AnsiFlags::None);
        a.set_foreground_color(color);
        assert_eq!(a.text(), format!("\x1b[38;2;{rgb}m"), "{color:?} foreground", color = color);
        a.clear();
        a.set_background_color(color);
        assert_eq!(a.text(), format!("\x1b[48;2;{rgb}m"), "{color:?} background", color = color);
    }
}

#[test]
fn check_terminal_palette_keeps_default_colors() {
    let mut a = AnsiFormatter::new(64, AnsiFlags::None);
    a.set_palette(Palette::Terminal);
    assert_eq!(a.text(), "");
    a.set_foreground_color(Color::Silver);
    assert_eq!(a.text(), "\x1b[38;2;196;196;196m");

    let mut indexed = AnsiFormatter::new(64, AnsiFlags::Use16ColorSchema);
    indexed.set_palette(Palette::Terminal);
    assert_eq!(indexed.text(), "");
    indexed.set_foreground_color(Color::Red);
    assert_eq!(indexed.text(), "\x1b[91m");
}

#[test]
fn check_builtin_palette_truecolor() {
    let mut a = AnsiFormatter::new(64, AnsiFlags::None);
    a.set_palette(Palette::BuiltIn);
    assert_eq!(a.text(), "");
    a.set_background_color(Color::Silver);
    assert_eq!(a.text(), "\x1b[48;2;192;192;192m");
    a.clear();
    a.set_foreground_color(Color::DarkBlue);
    assert_eq!(a.text(), "\x1b[38;2;0;0;128m");
    a.clear();
    a.set_foreground_color(Color::White);
    assert_eq!(a.text(), "\x1b[38;2;255;255;255m");
}

#[test]
fn check_custom_palette_truecolor() {
    let palette = ColorPalette::from_pairs(&[
        (Color::Black, 0, 9, 99),
        (Color::DarkBlue, 10, 100, 255),
        (Color::Red, 1, 2, 3),
        (Color::Silver, 196, 196, 196),
        (Color::White, 255, 255, 255),
    ]);
    let mut a = AnsiFormatter::new(128, AnsiFlags::None);
    a.set_palette(Palette::Custom(palette));
    assert_eq!(a.text(), "");

    a.set_foreground_color(Color::Black);
    assert_eq!(a.text(), "\x1b[38;2;0;9;99m");
    a.clear();
    a.set_background_color(Color::DarkBlue);
    assert_eq!(a.text(), "\x1b[48;2;10;100;255m");
    a.clear();
    a.set_foreground_color(Color::Red);
    a.set_background_color(Color::Silver);
    assert_eq!(a.text(), "\x1b[38;2;1;2;3m\x1b[48;2;196;196;196m");
    a.clear();
    a.set_foreground_color(Color::White);
    assert_eq!(a.text(), "\x1b[38;2;255;255;255m");
    // Colors omitted from the pair list keep the built-in value. Green is (0, 255, 0).
    a.clear();
    a.set_background_color(Color::Green);
    assert_eq!(a.text(), "\x1b[48;2;0;255;0m");
    // Transparent is not a palette slot.
    a.clear();
    a.set_foreground_color(Color::Transparent);
    assert_eq!(a.text(), "\x1b[38;2;0;0;0m");
}

#[test]
fn check_custom_palette_replaces_previous_colors() {
    let mut a = AnsiFormatter::new(64, AnsiFlags::None);
    a.set_palette(Palette::Custom(ColorPalette::from_pairs(&[(Color::Red, 1, 2, 3)])));
    a.set_palette(Palette::Custom(ColorPalette::from_pairs(&[(Color::Red, 9, 8, 7)])));
    a.set_foreground_color(Color::Red);
    assert_eq!(a.text(), "\x1b[38;2;9;8;7m");
}

#[test]
fn check_custom_palette_render() {
    let mut a = AnsiFormatter::new(128, AnsiFlags::None);
    a.set_palette(Palette::Custom(ColorPalette::from_pairs(&[
        (Color::Red, 10, 20, 30),
        (Color::Blue, 40, 50, 60),
    ])));
    let mut s = Surface::new(1, 1);
    s.write_ascii(0, 0, b"Z", charattr!("red, blue"), false);
    a.render(&s, Point::ORIGIN);
    assert_eq!(
        a.text(),
        "\u{1b}[0m\u{1b}[?25l\u{1b}[1;1H\u{1b}[38;2;10;20;30m\u{1b}[48;2;40;50;60mZ\u{1b}[?25l"
    );
}

#[test]
fn check_16color_palette_osc() {
    let mut a = AnsiFormatter::new(1024, AnsiFlags::Use16ColorSchema);
    a.set_palette(Palette::BuiltIn);
    assert_eq!(a.text(), osc_palette(&BUILTIN_OSC_SLOTS));

    a.clear();
    a.set_palette(Palette::Custom(ColorPalette::from_pairs(&[
        (Color::Red, 1, 2, 15),
        (Color::DarkBlue, 255, 0, 10),
    ])));
    let text = a.text();
    // Red is AppCUI index 12 and ANSI slot 9. DarkBlue is ANSI slot 4.
    // Blue (ANSI slot 12) was omitted, so it keeps the built-in value (0, 0, 255).
    assert!(text.contains("\x1b]4;9;rgb:01/02/0f\x1b\\"));
    assert!(text.contains("\x1b]4;4;rgb:ff/00/0a\x1b\\"));
    assert!(text.contains("\x1b]4;12;rgb:00/00/ff\x1b\\"));
    assert!(!text.contains("\x1b]4;12;rgb:01/02/0f\x1b\\"));

    // Indexed SGR codes stay the same; the terminal palette carries the RGB.
    a.clear();
    a.set_foreground_color(Color::Red);
    a.set_background_color(Color::DarkBlue);
    assert_eq!(a.text(), "\x1b[91m\x1b[44m");
}

#[test]
fn check_16color_indexed_codes() {
    let foreground = [
        (Color::Black, "\x1b[30m"),
        (Color::DarkBlue, "\x1b[34m"),
        (Color::DarkGreen, "\x1b[32m"),
        (Color::Teal, "\x1b[36m"),
        (Color::DarkRed, "\x1b[31m"),
        (Color::Magenta, "\x1b[35m"),
        (Color::Olive, "\x1b[33m"),
        (Color::Silver, "\x1b[37m"),
        (Color::Gray, "\x1b[90m"),
        (Color::Blue, "\x1b[94m"),
        (Color::Green, "\x1b[92m"),
        (Color::Aqua, "\x1b[96m"),
        (Color::Red, "\x1b[91m"),
        (Color::Pink, "\x1b[95m"),
        (Color::Yellow, "\x1b[93m"),
        (Color::White, "\x1b[97m"),
    ];
    let background = [
        (Color::Black, "\x1b[40m"),
        (Color::DarkBlue, "\x1b[44m"),
        (Color::DarkGreen, "\x1b[42m"),
        (Color::Teal, "\x1b[46m"),
        (Color::DarkRed, "\x1b[41m"),
        (Color::Magenta, "\x1b[45m"),
        (Color::Olive, "\x1b[43m"),
        (Color::Silver, "\x1b[47m"),
        (Color::Gray, "\x1b[100m"),
        (Color::Blue, "\x1b[104m"),
        (Color::Green, "\x1b[102m"),
        (Color::Aqua, "\x1b[106m"),
        (Color::Red, "\x1b[101m"),
        (Color::Pink, "\x1b[105m"),
        (Color::Yellow, "\x1b[103m"),
        (Color::White, "\x1b[107m"),
    ];
    for (color, seq) in foreground {
        let mut a = AnsiFormatter::new(32, AnsiFlags::Use16ColorSchema);
        a.set_foreground_color(color);
        assert_eq!(a.text(), seq, "{color:?} foreground", color = color);
    }
    for (color, seq) in background {
        let mut a = AnsiFormatter::new(32, AnsiFlags::Use16ColorSchema);
        a.set_background_color(color);
        assert_eq!(a.text(), seq, "{color:?} background", color = color);
    }
}

#[test]
fn check_reset_palette() {
    let mut a = AnsiFormatter::new(256, AnsiFlags::None);
    a.reset_palette();
    assert_eq!(a.text(), palette_reset_sequence());

    let mut indexed = AnsiFormatter::new(256, AnsiFlags::Use16ColorSchema);
    indexed.set_palette(Palette::BuiltIn);
    indexed.reset_palette();
    let mut expected = osc_palette(&BUILTIN_OSC_SLOTS);
    expected.push_str(&palette_reset_sequence());
    assert_eq!(indexed.text(), expected);
}

#[cfg(feature = "TRUE_COLORS")]
#[test]
fn check_rgb_color_ignores_named_palette() {
    let mut a = AnsiFormatter::new(64, AnsiFlags::None);
    a.set_palette(Palette::Custom(ColorPalette::from_pairs(&[(Color::Red, 1, 2, 3)])));
    a.clear();
    a.set_foreground_color(Color::RGB(7, 8, 9));
    assert_eq!(a.text(), "\x1b[38;2;7;8;9m");
    a.clear();
    a.set_background_color(Color::RGB(255, 128, 0));
    assert_eq!(a.text(), "\x1b[48;2;255;128;0m");
}