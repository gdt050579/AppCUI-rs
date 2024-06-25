use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
pub enum Flags {
    CheckBoxes = 0x01,
}

pub trait ListItem {
}