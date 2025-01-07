use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x01,
    SearchBar = 0x02,
    CheckBoxes = 0x04,
    SmallIcons = 0x08,
    LargeIcons = 0x10,
    CustomFilter = 0x20,
    NoSelection = 0x40,
}