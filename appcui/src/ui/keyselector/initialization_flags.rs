use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    AcceptEnter = 0x01,
    AcceptTab = 0x02,
    AcceptEscape = 0x04,
    ReadOnly = 0x08,
}
