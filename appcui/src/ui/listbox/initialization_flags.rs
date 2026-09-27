use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::ListBox`].
///
/// Combine values with `|`. `Flags::None` is a plain selectable list without
/// scroll bars, search, or check boxes.
pub enum Flags {
    /// Show vertical and horizontal scroll bars.
    ScrollBars = 0x0001,
    /// Show a search bar when the control is focused.
    SearchBar = 0x0002,
    /// Show a check box on each row, for example `[√] Item`.
    CheckBoxes = 0x0004,
    /// Scroll to the last item automatically when a new item is added.
    AutoScroll = 0x0008,
    /// Keep the selected row highlighted even when the list box is not focused.
    HighlightSelectedItemWhenInactive = 0x0010,
}
