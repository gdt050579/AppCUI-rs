use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::MarkdownComposer`].
///
/// Combine values with `|`. `Flags::None` is an editable composer that hides the markdown
/// markers and leaves the text exactly as it is typed.
pub enum Flags {
    /// Show the markdown markers (`**`, `_`, `` ` `` and ` ``` `) instead of hiding them.
    ShowMarkers = 0x01,
    /// Replace text emoticons and emoji names written between colons with the matching emoji
    /// while typing, for example `:B):` becomes 😎 and `:smile:` becomes 😀.
    Emoticons = 0x02,
    /// Allow the text to be viewed, selected and copied, but not modified.
    ReadOnly = 0x04,
}
