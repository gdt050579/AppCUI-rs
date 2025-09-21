use std::io::Write;
use crate::graphics::{CharFlags, Color, Point, Surface};
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags]
pub(crate) enum AnsiFlags {
    Use16ColorSchema = 1,
}

pub(crate) struct AnsiFormatter {
    text: String,
    flags: AnsiFlags,
}

impl AnsiFormatter {
    pub(crate) fn new(capacity: usize, flags: AnsiFlags) -> Self {
        Self {
            text: String::with_capacity(capacity),
            flags,
        }
    }
    #[inline(always)]
    pub(crate) fn text(&self) -> &str {
        &self.text
    }
    pub(crate) fn clear(&mut self) {
        self.text.clear();
    }
    #[allow(dead_code)]
    pub(crate) fn move_to_next_line(&mut self) {
        self.text.push('\n');
    }
    pub(crate) fn write_char(&mut self, ch: char) {
        if ch < ' ' {
            self.text.push(' ');
        } else {
            self.text.push(ch);
        }
    }
    pub(crate) fn write_string(&mut self, s: &str) {
        self.text.push_str(s);
    }

    pub(crate) fn enable_mouse_events(&mut self) {
        self.text.push_str("\x1b[?1000h\x1b[?1002h\x1b[?1003h\x1b[?1006h");
    }

    pub(crate) fn disable_mouse_events(&mut self) {
        self.text.push_str("\x1b[?1000l\x1b[?1002l\x1b[?1003l\x1b[?1006l");
    }
    #[inline(always)]
    pub(crate) fn set_foreground_color(&mut self, color: Color) {
        if self.flags.contains_one(AnsiFlags::Use16ColorSchema) {
            self.write_forenground_color_from_scheme(color);
        } else {
            self.text.push_str("\x1b[38;2;");
            self.write_color_as_rgb(color);
            self.text.push('m');
        }
    }
    #[inline(always)]
    pub(crate) fn set_background_color(&mut self, color: Color) {
        if self.flags.contains_one(AnsiFlags::Use16ColorSchema) {
            self.write_background_color_from_scheme(color);
        } else {
            self.text.push_str("\x1b[48;2;");
            self.write_color_as_rgb(color);
            self.text.push('m');
        }
    }
    #[inline(always)]
    pub(crate) fn set_color(&mut self, foreground: Color, background: Color) {
        self.set_foreground_color(foreground);
        self.set_background_color(background);
    }
    pub(crate) fn reset_color(&mut self) {
        self.text.push_str("\x1b[0m");
    }
    pub(crate) fn set_cursor_position(&mut self, x: i32, y: i32) {
        // x,y are 0-based, ANSI cursor position is 1-based
        self.text.push_str("\x1b[");
        self.write_number(y + 1);
        self.text.push(';');
        self.write_number(x + 1);
        self.text.push('H');
    }
    pub(crate) fn update_char_flags(&mut self, flags: CharFlags, old_flags: CharFlags) {
        macro_rules! update_ansi_flag {
            ($flag:ident, $set_value:expr, $reset_value:expr) => {
                if flags.contains_one(CharFlags::$flag) {
                    if !old_flags.contains_one(CharFlags::$flag) {
                        self.text.push_str($set_value);
                    }
                } else {
                    if old_flags.contains_one(CharFlags::$flag) {
                        self.text.push_str($reset_value);
                    }
                }
            };
        }

        update_ansi_flag!(Bold, "\x1b[1m", "\x1b[22m");
        update_ansi_flag!(Italic, "\x1b[3m", "\x1b[23m");
        update_ansi_flag!(Underline, "\x1b[4m", "\x1b[24m");
        update_ansi_flag!(StrikeThrough, "\x1b[9m", "\x1b[29m");
        update_ansi_flag!(DoubleUnderline, "\x1b[21m", "\x1b[24m");
        update_ansi_flag!(DottedUnderline, "\x1b[4:4m", "\x1b[24m");
        update_ansi_flag!(CurlyUnderline, "\x1b[4:3m", "\x1b[24m");        
    }
    pub(crate) fn set_char_flags(&mut self, flags: CharFlags) {
        macro_rules! set_ansi_flag {
            ($flag:ident, $set_value:expr, $reset_value:expr) => {
                if flags.contains_one(CharFlags::$flag) {
                    self.text.push_str($set_value);
                } else {
                    self.text.push_str($reset_value);
                }
            };
        }
        set_ansi_flag!(Bold, "\x1b[1m", "\x1b[22m");
        set_ansi_flag!(Italic, "\x1b[3m", "\x1b[23m");
        set_ansi_flag!(Underline, "\x1b[4m", "\x1b[24m");        
        set_ansi_flag!(StrikeThrough, "\x1b[9m", "\x1b[29m");
        set_ansi_flag!(DoubleUnderline, "\x1b[21m", "\x1b[24m");
        set_ansi_flag!(DottedUnderline, "\x1b[4:4m", "\x1b[24m");
        set_ansi_flag!(CurlyUnderline, "\x1b[4:3m", "\x1b[24m");        
    }
    pub(crate) fn hide_cursor(&mut self) {
        self.text.push_str("\x1b[?25l");
    }
    pub(crate) fn show_cursor(&mut self) {
        self.text.push_str("\x1b[?25h");
    }

    pub(crate) fn render(&mut self, surface: &Surface, offset: Point) {
        // draw characters using ANSI formatter
        self.clear();
        self.reset_color();
        self.hide_cursor();
        let mut x = 0;
        let mut y = 0;
        let w = surface.size.width;
        let h = surface.size.height;
        let start_y = offset.y;
        let mut f = None;
        let mut b = None;
        let mut c_flags = CharFlags::None;
        let chars = &surface.chars;
        while y < h {
            self.set_cursor_position(0, y as i32 + start_y);
            let ofs = y * w;
            while x < w {
                let ch = &chars[(ofs + x) as usize];
                if Some(ch.foreground) != f {
                    self.set_foreground_color(ch.foreground);
                    f = Some(ch.foreground);
                }
                if Some(ch.background) != b {
                    self.set_background_color(ch.background);
                    b = Some(ch.background);
                }
                if ch.flags != c_flags {
                    self.update_char_flags(ch.flags, c_flags);
                    c_flags = ch.flags;
                }
                if Self::is_wide_char(ch.code) {
                    // 1. write two spaces
                    self.write_string("  ");
                    // 2. reposition the cursor
                    self.set_cursor_position(x as i32, y as i32 + start_y);
                    // 3. write the character
                    self.write_char(ch.code);
                    // 4. skip next position and reposition the cursor
                    x += 2;
                    self.set_cursor_position(x as i32, y as i32 + start_y);
                } else {
                    self.write_char(ch.code);
                    x += 1;
                }
            }
            y += 1;
            x = 0;
        }
        // update the cursor
        if surface.cursor.is_visible() {
            self.set_cursor_position(surface.cursor.x as i32 + offset.x, surface.cursor.y as i32 + offset.y);
            self.show_cursor();
        } else {
            self.hide_cursor();
        }
    }

    #[inline(always)]
    fn write_forenground_color_from_scheme(&mut self, color: Color) {
        match color {
            Color::Black => self.text.push_str("\x1b[30m"),
            Color::DarkBlue => self.text.push_str("\x1b[34m"),
            Color::DarkGreen => self.text.push_str("\x1b[32m"),
            Color::Teal => self.text.push_str("\x1b[36m"),
            Color::DarkRed => self.text.push_str("\x1b[31m"),
            Color::Magenta => self.text.push_str("\x1b[35m"),
            Color::Olive => self.text.push_str("\x1b[33m"),
            Color::Silver => self.text.push_str("\x1b[37m"),
            Color::Gray => self.text.push_str("\x1b[90m"),
            Color::Blue => self.text.push_str("\x1b[94m"),
            Color::Green => self.text.push_str("\x1b[92m"),
            Color::Aqua => self.text.push_str("\x1b[96m"),
            Color::Red => self.text.push_str("\x1b[91m"),
            Color::Pink => self.text.push_str("\x1b[95m"),
            Color::Yellow => self.text.push_str("\x1b[93m"),
            Color::White => self.text.push_str("\x1b[97m"),
            Color::Transparent => {}
            #[cfg(feature = "TRUE_COLORS")]
            Color::RGB(r, g, b) => {
                self.text.push_str("\x1b[38;2;");
                self.write_number(r as i32);
                self.text.push(';');
                self.write_number(g as i32);
                self.text.push(';');
                self.write_number(b as i32);
                self.text.push('m');
            }
        }
    }

    #[inline(always)]
    fn write_background_color_from_scheme(&mut self, color: Color) {
        match color {
            Color::Black => self.text.push_str("\x1b[40m"),
            Color::DarkBlue => self.text.push_str("\x1b[44m"),
            Color::DarkGreen => self.text.push_str("\x1b[42m"),
            Color::Teal => self.text.push_str("\x1b[46m"),
            Color::DarkRed => self.text.push_str("\x1b[41m"),
            Color::Magenta => self.text.push_str("\x1b[45m"),
            Color::Olive => self.text.push_str("\x1b[43m"),
            Color::Silver => self.text.push_str("\x1b[47m"),
            Color::Gray => self.text.push_str("\x1b[100m"),
            Color::Blue => self.text.push_str("\x1b[104m"),
            Color::Green => self.text.push_str("\x1b[102m"),
            Color::Aqua => self.text.push_str("\x1b[106m"),
            Color::Red => self.text.push_str("\x1b[101m"),
            Color::Pink => self.text.push_str("\x1b[105m"),
            Color::Yellow => self.text.push_str("\x1b[103m"),
            Color::White => self.text.push_str("\x1b[107m"),
            Color::Transparent => {}
            #[cfg(feature = "TRUE_COLORS")]
            Color::RGB(r, g, b) => {
                self.text.push_str("\x1b[48;2;");
                self.write_number(r as i32);
                self.text.push(';');
                self.write_number(g as i32);
                self.text.push(';');
                self.write_number(b as i32);
                self.text.push('m');
            }
        }
    }

    #[inline(always)]
    fn write_color_as_rgb(&mut self, color: Color) {
        match color {
            Color::Black => self.text.push_str("0;0;0"),
            Color::DarkRed => self.text.push_str("128;0;0"),
            Color::DarkGreen => self.text.push_str("0;128;0"),
            Color::Olive => self.text.push_str("128;128;0"),
            Color::DarkBlue => self.text.push_str("0;0;128"),
            Color::Magenta => self.text.push_str("128;0;128"),
            Color::Teal => self.text.push_str("0;128;128"),
            Color::Silver => self.text.push_str("196;196;196"),
            Color::Gray => self.text.push_str("128;128;128"),
            Color::Red => self.text.push_str("255;0;0"),
            Color::Green => self.text.push_str("0;255;0"),
            Color::Yellow => self.text.push_str("255;255;0"),
            Color::Blue => self.text.push_str("0;0;255"),
            Color::Pink => self.text.push_str("255;0;255"),
            Color::Aqua => self.text.push_str("0;255;255"),
            Color::White => self.text.push_str("255;255;255"),
            Color::Transparent => self.text.push_str("0;0;0"),
            #[cfg(feature = "TRUE_COLORS")]
            Color::RGB(r, g, b) => {
                self.write_number(r as i32);
                self.text.push(';');
                self.write_number(g as i32);
                self.text.push(';');
                self.write_number(b as i32);
            }
        }
    }
    fn write_number(&mut self, n: i32) {
        let mut n = n;
        if n < 0 {
            self.text.push('-');
            n = -n;
        }
        let mut buffer = [0u8; 16];
        let mut i = 15;
        loop {
            buffer[i] = (n % 10) as u8 + b'0';
            n /= 10;
            if n == 0 {
                break;
            }
            i -= 1;
        }
        let txt = unsafe { std::str::from_utf8_unchecked(&buffer[i..]) };
        self.text.push_str(txt);
    }

    #[inline(always)]
    fn is_wide_char(ch: char) -> bool {
        matches!(ch as u32, 0x1100..=0x115F
            | 0x2329..=0x232A
            | 0x2E80..=0x303E
            | 0x3040..=0xA4CF
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
            | 0x1F300..=0x1F64F
            | 0x1F900..=0x1F9FF
            | 0x20000..=0x2FFFD
            | 0x30000..=0x3FFFD)
    }
}
