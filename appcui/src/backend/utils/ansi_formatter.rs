use crate::graphics::{CharFlags, Color, Point, Surface};
use std::io::Write;
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

    #[cfg_attr(windows, allow(dead_code))]
    pub(crate) fn enable_mouse_events(&mut self) {
        self.text.push_str("\x1b[?1000h\x1b[?1002h\x1b[?1003h\x1b[?1006h");
    }

    #[cfg_attr(windows, allow(dead_code))]
    pub(crate) fn disable_mouse_events(&mut self) {
        self.text.push_str("\x1b[?1000l\x1b[?1002l\x1b[?1003l\x1b[?1006l");
    }

    #[inline(always)]
    pub(crate) fn reset_screen(&mut self) {
        self.text.push_str("\x1b[0m\x1b[2J\x1b[3J\x1b[H");
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

    pub(crate) fn execute(&mut self) {
        // write to stdout
        let _ = std::io::stdout().write_all(self.text.as_bytes());
        let _ = std::io::stdout().flush();
        // clear the text
        self.text.clear();
    }

    #[inline(always)]
    fn is_wide_char(ch: char) -> bool {
        matches!(ch as u32, 0x1100..=0x115F | 0x231A..=0x231B | 0x2329 | 0x232A | 0x23E9..=0x23EC | 0x23F0 | 0x23F3 | 0x25FD..=0x25FE | 0x2614..=0x2615 | 0x2630..=0x2637 | 0x2648..=0x2653 | 0x267F | 0x268A..=0x268F | 0x2693 | 0x26A1 | 0x26AA..=0x26AB | 0x26BD..=0x26BE | 0x26C4..=0x26C5 | 0x26CE | 0x26D4 | 0x26EA | 0x26F2..=0x26F3 | 0x26F5 | 0x26FA | 0x26FD | 0x2705 | 0x270A..=0x270B | 0x2728 | 0x274C | 0x274E | 0x2753..=0x2755 | 0x2757 | 0x2795..=0x2797 | 0x27B0 | 0x27BF | 0x2B1B..=0x2B1C | 0x2B50 | 0x2B55 | 0x2E80..=0x2E99 | 0x2E9B..=0x2EF3 | 0x2F00..=0x2FD5 | 0x2FF0..=0x2FFF | 0x3000 | 0x3001..=0x3003 | 0x3004 | 0x3005 | 0x3006 | 0x3007 | 0x3008 | 0x3009 | 0x300A | 0x300B | 0x300C | 0x300D | 0x300E | 0x300F | 0x3010 | 0x3011 | 0x3012..=0x3013 | 0x3014 | 0x3015 | 0x3016 | 0x3017 | 0x3018 | 0x3019 | 0x301A | 0x301B | 0x301C | 0x301D | 0x301E..=0x301F | 0x3020 | 0x3021..=0x3029 | 0x302A..=0x302D | 0x302E..=0x302F | 0x3030 | 0x3031..=0x3035 | 0x3036..=0x3037 | 0x3038..=0x303A | 0x303B | 0x303C | 0x303D | 0x303E | 0x3041..=0x3096 | 0x3099..=0x309A | 0x309B..=0x309C | 0x309D..=0x309E | 0x309F | 0x30A0 | 0x30A1..=0x30FA | 0x30FB | 0x30FC..=0x30FE | 0x30FF | 0x3105..=0x312F | 0x3131..=0x318E | 0x3190..=0x3191 | 0x3192..=0x3195 | 0x3196..=0x319F | 0x31A0..=0x31BF | 0x31C0..=0x31E5 | 0x31EF | 0x31F0..=0x31FF | 0x3200..=0x321E | 0x3220..=0x3229 | 0x322A..=0x3247 | 0x3250 | 0x3251..=0x325F | 0x3260..=0x327F | 0x3280..=0x3289 | 0x328A..=0x32B0 | 0x32B1..=0x32BF | 0x32C0..=0x32FF | 0x3300..=0x33FF | 0x3400..=0x4DBF | 0x4DC0..=0x4DFF | 0x4E00..=0x9FFF | 0xA000..=0xA014 | 0xA015 | 0xA016..=0xA48C | 0xA490..=0xA4C6 | 0xA960..=0xA97C | 0xAC00..=0xD7A3 | 0xF900..=0xFA6D | 0xFA6E..=0xFA6F | 0xFA70..=0xFAD9 | 0xFADA..=0xFAFF | 0xFE10..=0xFE16 | 0xFE17 | 0xFE18 | 0xFE19 | 0xFE30 | 0xFE31..=0xFE32 | 0xFE33..=0xFE34 | 0xFE35 | 0xFE36 | 0xFE37 | 0xFE38 | 0xFE39 | 0xFE3A | 0xFE3B | 0xFE3C | 0xFE3D | 0xFE3E | 0xFE3F | 0xFE40 | 0xFE41 | 0xFE42 | 0xFE43 | 0xFE44 | 0xFE45..=0xFE46 | 0xFE47 | 0xFE48 | 0xFE49..=0xFE4C | 0xFE4D..=0xFE4F | 0xFE50..=0xFE52 | 0xFE54..=0xFE57 | 0xFE58 | 0xFE59 | 0xFE5A | 0xFE5B | 0xFE5C | 0xFE5D | 0xFE5E | 0xFE5F..=0xFE61 | 0xFE62 | 0xFE63 | 0xFE64..=0xFE66 | 0xFE68 | 0xFE69 | 0xFE6A..=0xFE6B | 0xFF01..=0xFF03 | 0xFF04 | 0xFF05..=0xFF07 | 0xFF08 | 0xFF09 | 0xFF0A | 0xFF0B | 0xFF0C | 0xFF0D | 0xFF0E..=0xFF0F | 0xFF10..=0xFF19 | 0xFF1A..=0xFF1B | 0xFF1C..=0xFF1E | 0xFF1F..=0xFF20 | 0xFF21..=0xFF3A | 0xFF3B | 0xFF3C | 0xFF3D | 0xFF3E | 0xFF3F | 0xFF40 | 0xFF41..=0xFF5A | 0xFF5B | 0xFF5C | 0xFF5D | 0xFF5E | 0xFF5F | 0xFF60 | 0xFFE0..=0xFFE1 | 0xFFE2 | 0xFFE3 | 0xFFE4 | 0xFFE5..=0xFFE6 | 0x16FE0..=0x16FE1 | 0x16FE2 | 0x16FE3 | 0x16FE4 | 0x16FF0..=0x16FF1 | 0x16FF2..=0x16FF3 | 0x16FF4..=0x16FF6 | 0x17000..=0x187FF | 0x18800..=0x18AFF | 0x18B00..=0x18CD5 | 0x18CFF | 0x18D00..=0x18D1E | 0x18D80..=0x18DF2 | 0x1AFF0..=0x1AFF3 | 0x1AFF5..=0x1AFFB | 0x1AFFD..=0x1AFFE | 0x1B000..=0x1B0FF | 0x1B100..=0x1B122 | 0x1B132 | 0x1B150..=0x1B152 | 0x1B155 | 0x1B164..=0x1B167 | 0x1B170..=0x1B2FB | 0x1D300..=0x1D356 | 0x1D360..=0x1D376 | 0x1F004 | 0x1F0CF | 0x1F18E | 0x1F191..=0x1F19A | 0x1F200..=0x1F202 | 0x1F210..=0x1F23B | 0x1F240..=0x1F248 | 0x1F250..=0x1F251 | 0x1F260..=0x1F265 | 0x1F300..=0x1F320 | 0x1F32D..=0x1F335 | 0x1F337..=0x1F37C | 0x1F37E..=0x1F393 | 0x1F3A0..=0x1F3CA | 0x1F3CF..=0x1F3D3 | 0x1F3E0..=0x1F3F0 | 0x1F3F4 | 0x1F3F8..=0x1F3FA | 0x1F3FB..=0x1F3FF | 0x1F400..=0x1F43E | 0x1F440 | 0x1F442..=0x1F4FC | 0x1F4FF..=0x1F53D | 0x1F54B..=0x1F54E | 0x1F550..=0x1F567 | 0x1F57A | 0x1F595..=0x1F596 | 0x1F5A4 | 0x1F5FB..=0x1F5FF | 0x1F600..=0x1F64F | 0x1F680..=0x1F6C5 | 0x1F6CC | 0x1F6D0..=0x1F6D2 | 0x1F6D5..=0x1F6D8 | 0x1F6DC..=0x1F6DF | 0x1F6EB..=0x1F6EC | 0x1F6F4..=0x1F6FC | 0x1F7E0..=0x1F7EB | 0x1F7F0 | 0x1F90C..=0x1F93A | 0x1F93C..=0x1F945 | 0x1F947..=0x1F9FF | 0x1FA70..=0x1FA7C | 0x1FA80..=0x1FA8A | 0x1FA8E..=0x1FAC6 | 0x1FAC8 | 0x1FACD..=0x1FADC | 0x1FADF..=0x1FAEA | 0x1FAEF..=0x1FAF8 | 0x20000..=0x2A6DF | 0x2A6E0..=0x2A6FF | 0x2A700..=0x2B81D | 0x2B81E..=0x2B81F | 0x2B820..=0x2CEAD | 0x2CEAE..=0x2CEAF | 0x2CEB0..=0x2EBE0 | 0x2EBE1..=0x2EBEF | 0x2EBF0..=0x2EE5D | 0x2EE5E..=0x2F7FF | 0x2F800..=0x2FA1D | 0x2FA1E..=0x2FA1F | 0x2FA20..=0x2FFFD | 0x30000..=0x3134A | 0x3134B..=0x3134F | 0x31350..=0x33479 | 0x3347A..=0x3FFFD)
    }
}
