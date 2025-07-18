use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x0001,
    SearchBar = 0x0002,
    CheckBoxes = 0x0004,
    AutoScroll = 0x0008,
    HighlightSelectedItemWhenInactive = 0x0010,
}
