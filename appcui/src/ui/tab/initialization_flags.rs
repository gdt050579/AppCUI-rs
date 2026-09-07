use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
pub enum Flags {
    TransparentBackground = 0x01,
    TabsBar = 0x02,
}

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq)]
/// Where tab headers are drawn on a [`super::Tab`].
///
/// `HiddenTabs` shows only the page. The other variants place the header bar on
/// the top, bottom, or left edge.
pub enum Type {
    HiddenTabs,
    OnTop,
    OnBottom,
    OnLeft,
}