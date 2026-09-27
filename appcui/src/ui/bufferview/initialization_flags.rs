use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16)]
/// Initialization flags for a [`struct@super::BufferView`].
///
/// Combine values with `|`. `Flags::None` shows the hex dump without extra chrome
/// (no scroll bars, address column, or search bar).
pub enum Flags {
    /// Show vertical and horizontal scroll bars.
    ScrollBars = 0x0001,
    /// Show a search bar when the control is focused.
    SearchBar = 0x0002,
    /// Hide the column header above the dump.
    HideHeader = 0x0004,
    /// Show a left-hand address (offset) column.
    ShowAddress = 0x0008,
    /// Show a column with named intervals that cover the current offset.
    ShowIntervalNames = 0x0010,
    /// Do not dim the inactive hex or character panel.
    NoPanelDimming = 0x0020,
    /// Highlight printable ASCII runs in the character panel.
    ShowAsciiStrings = 0x0040,
    /// Highlight printable UTF-16 (ASCII-range) runs in the character panel.
    ShowUtf16AsciiStrings = 0x0080,
    /// Decode multi-byte UTF-8 sequences instead of treating each byte as a character.
    DecodeUTF8Characters = 0x0100,
    /// Prevent editing; the buffer can only be viewed.
    ReadOnly = 0x0200,
    /// Tab moves focus between the hex panel and the character panel.
    TabSwitchesActivePanel = 0x0400,
}
