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
    fn from_hash_color_id(hash_color_id: HashColorID) -> Color {
        match hash_color_id {
            HashColorID::Black => Color::Black,
            HashColorID::DarkBlue => Color::DarkBlue,
            HashColorID::DarkGreen => Color::DarkGreen,
            HashColorID::Teal => Color::Teal,
            HashColorID::DarkRed => Color::DarkRed,
            HashColorID::Magenta => Color::Magenta,
            HashColorID::Olive => Color::Olive,
            HashColorID::Silver => Color::Silver,
            HashColorID::Gray => Color::Gray,
            HashColorID::Blue => Color::Blue,
            HashColorID::Green => Color::Green,
            HashColorID::Aqua => Color::Aqua,
            HashColorID::Red => Color::Red,
            HashColorID::Pink => Color::Pink,
            HashColorID::Yellow => Color::Yellow,
            HashColorID::White => Color::White,
            HashColorID::Transparent => Color::Transparent,
        }
    }
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
        if idx>=s.len() {
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
        if idx>=s.len() {
            return None;
        }
        if s[idx] != b'(' {
            return None;
        }
        // text to vaildate is between idx+1 and s.len()-1
        let text_to_validate = &s[idx+1..s.len()-1];
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
        if let Some(col_id) = HashColorID::from_hash(crate::utils::compute_hash(s)) {
            return Some(Self::from_hash_color_id(col_id));
        }
        Self::from_rgb_repr(s)
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

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum HashColorID {
    Black = 0,
    DarkBlue = 1,
    DarkGreen = 2,
    Teal = 3,
    DarkRed = 4,
    Magenta = 5,
    Olive = 6,
    Silver = 7,
    Gray = 8,
    Blue = 9,
    Green = 10,
    Aqua = 11,
    Red = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
    Transparent = 16,
}

static HASH_TO_ALIGNAMENT: [Option<HashColorID>; 127] = [
    None,
    Some(HashColorID::Silver),
    None,
    Some(HashColorID::Magenta),
    None,
    None,
    Some(HashColorID::White),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(HashColorID::Transparent),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(HashColorID::Blue),
    None,
    Some(HashColorID::Green),
    None,
    None,
    None,
    Some(HashColorID::DarkGreen),
    Some(HashColorID::DarkBlue),
    None,
    None,
    None,
    Some(HashColorID::DarkGreen),
    None,
    None,
    None,
    Some(HashColorID::Gray),
    None,
    None,
    None,
    None,
    Some(HashColorID::Pink),
    Some(HashColorID::DarkBlue),
    None,
    None,
    None,
    Some(HashColorID::DarkRed),
    None,
    None,
    Some(HashColorID::DarkRed),
    None,
    Some(HashColorID::Red),
    Some(HashColorID::Green),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(HashColorID::Red),
    None,
    Some(HashColorID::Silver),
    None,
    None,
    Some(HashColorID::Aqua),
    Some(HashColorID::Teal),
    Some(HashColorID::Black),
    None,
    Some(HashColorID::Blue),
    None,
    Some(HashColorID::Gray),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(HashColorID::Transparent),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(HashColorID::Transparent),
    None,
    None,
    None,
    Some(HashColorID::Aqua),
    None,
    Some(HashColorID::Olive),
    Some(HashColorID::Pink),
    Some(HashColorID::Yellow),
    Some(HashColorID::Yellow),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(HashColorID::White),
    None,
    None,
    None,
    None,
    None,
    None,
];

static HASH_COLISION_VALIDATOR: [u64; 127] = [
    0x0,
    0xDB7D47CB10B772A0,
    0x0,
    0x6C90E772EDBC8708,
    0x0,
    0x0,
    0xAF63EA4C86020456,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0xAF63B24C8601A52E,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0xC5CCD29BC2DDA64D,
    0x0,
    0xAF63DA4C8601E926,
    0x0,
    0x0,
    0x0,
    0xDFB890FA554DCD7C,
    0x8914E07B53BA1E3,
    0x0,
    0x0,
    0x0,
    0x8915107B53BA6FC,
    0x0,
    0x0,
    0x0,
    0xFB6FAA7243F4459A,
    0x0,
    0x0,
    0x0,
    0x0,
    0xBF30EC0DC5331C0D,
    0xA1A82D75FD18630D,
    0x0,
    0x0,
    0x0,
    0x9489A9902E6C5BDC,
    0x0,
    0x0,
    0x8913E07B53B86B3,
    0x0,
    0xAF63EF4C86020CD5,
    0xF40F029637FECBC,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x89E9BE1960F4C21C,
    0x0,
    0xC2B3DF77074135A6,
    0x0,
    0x0,
    0xAF63DC4C8601EC8C,
    0xFA23DAEF19AFC4DF,
    0x4B5DD0ABBC6FC1E4,
    0x0,
    0xAF63DF4C8601F1A5,
    0x0,
    0xC2BAF07707477137,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0xEB0A3EBE378076F0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0xBDBDA3148488BB71,
    0x0,
    0x0,
    0x0,
    0x7E2F198437C28B35,
    0x0,
    0x73D4BFA38E3F676C,
    0xCBF29CE484222325,
    0xAF63F44C86021554,
    0x8346A574925E75A9,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0xCED973885856E206,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
];

impl HashColorID {
    pub(super) fn from_hash(hash: u64) -> Option<HashColorID> {
        let entry_index = (hash % 127) as usize;
        if HASH_COLISION_VALIDATOR[entry_index] != hash {
            return None;
        }
        HASH_TO_ALIGNAMENT[entry_index]
    }
}
