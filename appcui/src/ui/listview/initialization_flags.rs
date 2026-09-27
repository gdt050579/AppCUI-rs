use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16)]
/// Initialization flags for a [`struct@super::ListView`].
///
/// Combine values with `|`. `Flags::None` is a selectable table without scroll bars,
/// search, check boxes, groups, or icons.
pub enum Flags {
    /// Show vertical and horizontal scroll bars.
    ScrollBars = 0x01,
    /// Show a search bar when the control is focused.
    SearchBar = 0x02,
    /// Show a check box on each row, for example `[√] Name`.
    CheckBoxes = 0x04,
    /// Group rows under collapsible group headers.
    ShowGroups = 0x08,
    /// Draw a small icon in front of each item.
    SmallIcons = 0x10,
    /// Draw a large (two-cell) icon in front of each item.
    LargeIcons = 0x20,
    /// Let the parent filter items instead of using the built-in search.
    CustomFilter = 0x40,
    /// Disable selecting rows (useful with check boxes only).
    NoSelection = 0x80,
    /// Join column and control borders with neighboring window or panel frames.
    MergeBorders = 0x100,
}

