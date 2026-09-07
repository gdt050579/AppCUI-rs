use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
/// Initialization flags for a [`struct@super::Tab`].
///
/// Combine values with `|`. `Flags::None` paints the tab background and does not
/// draw a filled bar behind the headers.
pub enum Flags {
    /// Do not fill the tab background, so the parent control shows through.
    TransparentBackground = 0x01,
    /// Draw a solid bar behind the tab headers.
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