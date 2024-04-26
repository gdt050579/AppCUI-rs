use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
pub enum Flags {
    ProcessEnter             = 0x0001,
    Readonly                 = 0x0002,
    DisableAutoSelectOnFocus = 0x0004,
}