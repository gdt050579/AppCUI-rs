use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    HideButtons = 0x0001,
    Percentage = 0x0002,
}
