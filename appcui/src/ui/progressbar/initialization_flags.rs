use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits: 8)]
pub enum Flags {
    HidePercentage = 0x01,
}