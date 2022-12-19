use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=16)]
pub enum Attribute {
    Bold = 0x0001,
    Italic = 0x0002,
    Underline = 0x0004,
}