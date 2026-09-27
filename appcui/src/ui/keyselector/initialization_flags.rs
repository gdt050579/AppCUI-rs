use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
/// Initialization flags for a [`struct@super::KeySelector`].
///
/// Combine values with `|`. `Flags::None` records ordinary keys but leaves Enter,
/// Tab, and Escape for the parent (so they still confirm, move focus, or cancel).
pub enum Flags {
    /// Treat Enter as a key combination instead of confirming the parent dialog.
    AcceptEnter = 0x01,
    /// Treat Tab as a key combination instead of moving focus.
    AcceptTab = 0x02,
    /// Treat Escape as a key combination instead of canceling.
    AcceptEscape = 0x04,
    /// Display the current key but ignore further key presses.
    ReadOnly = 0x08,
}
