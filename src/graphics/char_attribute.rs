use super::Color;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=16)]
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
    pub fn new(fore: Color, back: Color, flags: CharFlags)->CharAttribute {
        CharAttribute{
            foreground: fore,
            background: back,
            flags: flags
        }
    }
}