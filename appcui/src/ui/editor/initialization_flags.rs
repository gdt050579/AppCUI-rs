use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x0001,
    ReadOnly = 0x0002,
    ShowLineNumbers = 0x0004,
    ShowFoldMarkers = 0x0008,
}