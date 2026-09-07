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
    /// No header bar; only the selected page is shown.
    HiddenTabs,
    /// Headers along the top edge, for example `[Page 1] Page 2 Page 3`.
    OnTop,
    /// Headers along the bottom edge.
    OnBottom,
    /// Headers stacked on the left side, one caption per row.
    OnLeft,
}