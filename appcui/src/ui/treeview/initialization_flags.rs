use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::TreeView`].
///
/// Combine values with `|`. `Flags::None` is a selectable tree with a column header
/// and no scroll bars, search, or icons.
pub enum Flags {
    /// Show vertical and horizontal scroll bars.
    ScrollBars = 0x01,
    /// Show a search bar when the control is focused.
    SearchBar = 0x02,
    /// Draw a small icon in front of each item.
    SmallIcons = 0x04,
    /// Draw a large (two-cell) icon in front of each item.
    LargeIcons = 0x08,
    /// Let the parent filter items instead of using the built-in search.
    CustomFilter = 0x10,
    /// Disable selecting items.
    NoSelection = 0x20,
    /// Hide the column header.
    HideHeader = 0x40,
    /// Join column and control borders with neighboring window or panel frames.
    MergeBorders = 0x80,
}