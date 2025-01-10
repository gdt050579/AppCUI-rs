use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ReadOnly = 0x01,
    CaseSensitive = 0x02
}