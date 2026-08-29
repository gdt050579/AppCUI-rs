#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Color {
    Black,
    DarkBlue,
    DarkGreen,
    Teal,
    DarkRed,
    Magenta,
    Olive,
    Silver,
    Gray,
    Blue,
    Green,
    Aqua,
    Red,
    Pink,
    Yellow,
    White,
    Transparent,
    RGB(u8, u8, u8),
}

impl Color {
    fn hex_nibble(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }
    fn extract_u8(s: &[u8], start: usize) -> Option<(u8, usize)> {
        if s.len() == 0 {
            return None;
        }
        let mut idx = start;
        let mut num = 0u64;
        while idx < s.len() && s[idx].is_ascii_digit() {
            num = num * 10 + (s[idx] - b'0') as u64;
            if num > 255 {
                return None;
            }
            idx += 1;
        }
        if idx == start {
            return None;
        }
        Some((num as u8, idx))
    }
    fn skip_whitespace(s: &[u8], start: usize) -> usize {
        let mut idx = start;
        while idx < s.len() && s[idx].is_ascii_whitespace() {
            idx += 1;
        }
        idx
    }
    fn check_comma(s: &[u8], start: usize) -> Option<usize> {
        let idx = Self::skip_whitespace(s, start);
        if idx >= s.len() {
            return None;
        }
        if s[idx] != b',' {
            return None;
        }
        let idx = Self::skip_whitespace(s, idx + 1);
        Some(idx)
    }
    fn from_diez_format(s: &str) -> Option<Color> {
        let s = s.as_bytes();
        // it is assumed that s[0] == '#'
        if s.len() == 7 {
            // #RRGGBB
            let r = (Self::hex_nibble(s[1])? << 4) | Self::hex_nibble(s[2])?;
            let g = (Self::hex_nibble(s[3])? << 4) | Self::hex_nibble(s[4])?;
            let b = (Self::hex_nibble(s[5])? << 4) | Self::hex_nibble(s[6])?;
            Some(Color::RGB(r, g, b))
        } else if s.len() == 4 {
            // #RGB -> each Self::hex_nibble is duplicated (0xF -> 0xFF)
            let r = Self::hex_nibble(s[1])?;
            let g = Self::hex_nibble(s[2])?;
            let b = Self::hex_nibble(s[3])?;
            Some(Color::RGB((r << 4) | r, (g << 4) | g, (b << 4) | b))
        } else {
            None
        }
    }
    fn from_rgb_format(s: &str) -> Option<Color> {
        // it is assumed that s starts with RGB
        let s = s.as_bytes();
        if *(s.last()?) != b')' {
            return None;
        }
        let mut idx = 3;
        while idx < s.len() && s[idx].is_ascii_whitespace() {
            idx += 1;
        }
        if idx >= s.len() {
            return None;
        }
        if s[idx] != b'(' {
            return None;
        }
        // text to vaildate is between idx+1 and s.len()-1
        let text_to_validate = &s[idx + 1..s.len() - 1];
        // text_to_validate is number, number, number with any number of whitespace characters between numbers
        let idx = Self::skip_whitespace(text_to_validate, 0);
        let (r, idx) = Self::extract_u8(text_to_validate, idx)?;
        let idx = Self::check_comma(text_to_validate, idx)?;
        let (g, idx) = Self::extract_u8(text_to_validate, idx)?;
        let idx = Self::check_comma(text_to_validate, idx)?;
        let (b, idx) = Self::extract_u8(text_to_validate, idx)?;
        let idx = Self::skip_whitespace(text_to_validate, idx);
        if idx < text_to_validate.len() {
            return None;
        }
        Some(Color::RGB(r, g, b))
    }
    fn from_rgb_repr(s: &str) -> Option<Color> {
        let s = s.trim();
        if s.starts_with('#') {
            Self::from_diez_format(s)
        } else if (s.len() > 3) && ((s.as_bytes()[0] | 0x20) == b'r') && ((s.as_bytes()[1] | 0x20) == b'g') && ((s.as_bytes()[2] | 0x20) == b'b') {
            Self::from_rgb_format(s)
        } else {
            None
        }
    }
    pub(crate) fn from_str(s: &str) -> Option<Color> {
        const MAX_COLOR_NAME_LENGTH: usize = 16;
        if s.len() > MAX_COLOR_NAME_LENGTH {
            return Self::from_rgb_repr(s);
        }
        let mut buf: [u8;MAX_COLOR_NAME_LENGTH] = [0;MAX_COLOR_NAME_LENGTH];
        for (i, b) in s.as_bytes().iter().enumerate() {
            let ch = (*b).to_ascii_lowercase();
            if ch > 127 {
                return None;
            }
            buf[i] = ch;
        }
        let lower = std::str::from_utf8(&buf[..s.len()]).unwrap();
        let named = match lower {
            "black" => Some(Color::Black),
            "darkblue" | "db" => Some(Color::DarkBlue),
            "darkgreen" | "dg" => Some(Color::DarkGreen),
            "teal" => Some(Color::Teal),
            "darkred" | "dr" => Some(Color::DarkRed),
            "magenta" => Some(Color::Magenta),
            "olive" => Some(Color::Olive),
            "silver" | "gray75" => Some(Color::Silver),
            "gray" | "gray50" => Some(Color::Gray),
            "blue" | "b" => Some(Color::Blue),
            "green" | "g" => Some(Color::Green),
            "aqua" | "a" => Some(Color::Aqua),
            "red" | "r" => Some(Color::Red),
            "pink" => Some(Color::Pink),
            "yellow" | "y" => Some(Color::Yellow),
            "white" | "w" => Some(Color::White),
            "transparent" | "invisible" | "?" => Some(Color::Transparent),
            _ => None,
        };
        named.or_else(|| Self::from_rgb_repr(s))
    }
    pub(crate) fn write_ctor(&self, output: &mut String) {
        output.push_str("Color::");
        match self {
            Color::RGB(r, g, b) => {
                output.push_str("RGB(");
                output.push_str(&r.to_string());
                output.push_str(", ");
                output.push_str(&g.to_string());
                output.push_str(", ");
                output.push_str(&b.to_string());
                output.push(')');
            }
            Color::Black => output.push_str("Black"),
            Color::DarkBlue => output.push_str("DarkBlue"),
            Color::DarkGreen => output.push_str("DarkGreen"),
            Color::Teal => output.push_str("Teal"),
            Color::DarkRed => output.push_str("DarkRed"),
            Color::Magenta => output.push_str("Magenta"),
            Color::Olive => output.push_str("Olive"),
            Color::Silver => output.push_str("Silver"),
            Color::Gray => output.push_str("Gray"),
            Color::Blue => output.push_str("Blue"),
            Color::Green => output.push_str("Green"),
            Color::Aqua => output.push_str("Aqua"),
            Color::Red => output.push_str("Red"),
            Color::Pink => output.push_str("Pink"),
            Color::Yellow => output.push_str("Yellow"),
            Color::White => output.push_str("White"),
            Color::Transparent => output.push_str("Transparent"),
        }
    }
}
