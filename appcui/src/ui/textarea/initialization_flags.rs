use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
/// Initialization flags for a [`struct@super::TextArea`].
///
/// Combine values with `|`. `Flags::None` is an editable multi-line editor without
/// line numbers or scroll bars.
pub enum Flags {
    /// Show a gutter with line numbers on the left.
    ShowLineNumber           = 0x0001,
    /// Prevent editing; the text can only be viewed and selected.
    ReadOnly                 = 0x0002,
    // CodeWrap                 = 0x0004,
    // TextHighlight            = 0x0008,
    /// Show vertical and horizontal scroll bars when focused.
    ScrollBars                = 0x0010,
    // SearchBar                = 0x0020,
    /// Highlight the entire line that contains the caret.
    HighlightCursor          = 0x0040,
}