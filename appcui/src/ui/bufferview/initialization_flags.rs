use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16)]
pub enum Flags {
    ScrollBars = 0x0001,
    SearchBar = 0x0002,
    HideHeader = 0x0004,
    ShowAddress = 0x0008,
    ShowIntervalNames = 0x0010,
    NoPanelDimming = 0x0020,
    ShowAsciiStrings = 0x0040,
    ShowUnicodeStrings = 0x0080,
    DecodeUTF8Characters = 0x0100,
    ReadOnly = 0x0200,
}
