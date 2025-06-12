use crate::graphics::Color;

pub(crate) struct AnsiFormatter {
    text: String,
}

impl AnsiFormatter {  
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            text: String::with_capacity(capacity),
        }
    }
    #[inline(always)]
    pub(crate) fn text(&self) -> &str {
        &self.text
    }
    pub(crate) fn clear(&mut self) {
        self.text.clear();
    }
    pub(crate) fn write_char(&mut self, ch: char) {
        self.text.push(ch);
    }
    pub(crate) fn write_string(&mut self, s: &str) {
        self.text.push_str(s);
    }
    pub(crate) fn set_foreground_color(&mut self, color: Color) {
        self.text.push_str("\x1b[38;2;");
        self.write_color_as_rgb(color);
        self.text.push('m');
    }
    pub(crate) fn set_background_color(&mut self, color: Color) {
        self.text.push_str("\x1b[48;2;");
        self.write_color_as_rgb(color);
        self.text.push('m');
    }
    pub(crate) fn set_color(&mut self, foreground: Color, background: Color) {
        self.set_foreground_color(foreground);
        self.set_background_color(background);
    }
    pub(crate) fn reset_color(&mut self) {
        self.text.push_str("\x1b[0m");
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
            },
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
}



