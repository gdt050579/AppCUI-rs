use super::Color;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16,debug=true)]
pub enum CharFlags {
    Bold = 0x0001,
    Italic = 0x0002,
    Underline = 0x0004,
}

pub struct CharAttribute {
    pub foreground: Color,
    pub background: Color,
    pub flags: CharFlags,
}

impl CharAttribute {
    pub fn new(fore: Color, back: Color, flags: CharFlags) -> CharAttribute {
        CharAttribute {
            foreground: fore,
            background: back,
            flags: flags,
        }
    }
    pub fn with_color(fore: Color, back: Color) -> CharAttribute {
        CharAttribute {
            foreground: fore,
            background: back,
            flags: CharFlags::None,
        }
    }
}
impl Default for CharAttribute {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
            flags: CharFlags::None,
        }
    }
}
