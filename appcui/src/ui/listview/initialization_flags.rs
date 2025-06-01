use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x01,
    SearchBar = 0x02,
    CheckBoxes = 0x04,
    ShowGroups = 0x08,
    SmallIcons = 0x10,
    LargeIcons = 0x20,
    CustomFilter = 0x40,
    NoSelection = 0x80,
}
