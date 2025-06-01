use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x01,
    SearchBar = 0x02,
    SmallIcons = 0x04,
    LargeIcons = 0x08,
    CustomFilter = 0x10,
    NoSelection = 0x20,
    HideHeader = 0x40,
}
