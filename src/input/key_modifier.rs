use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
pub enum KeyModifier {
    Alt = 0x01,
    Ctrl = 0x02,
    Shift = 0x04
}