use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
pub enum Flags {
    ScrollBars = 0x01,
    SearchBar = 0x02,
    CheckBoxes = 0x04,
    Groups = 0x08,
}

pub trait ListItem {
}