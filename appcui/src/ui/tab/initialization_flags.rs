use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    TransparentBackground = 0x01,
    TabsBar = 0x02,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Type {
    HiddenTabs,
    OnTop,
    OnBottom,
    OnLeft,
}
