use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
pub enum Flags {
    ShowLineNumber           = 0x0001,
    ReadOnly                 = 0x0002,
    // CodeWrap                 = 0x0004,
    // TextHighlight            = 0x0008,
    ScrollBars                = 0x0010,
    // SearchBar                = 0x0020,
    HighlightCursor          = 0x0040,
}