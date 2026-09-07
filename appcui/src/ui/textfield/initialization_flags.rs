use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
/// Initialization flags for a [`struct@super::TextField`].
///
/// Combine values with `|`. `Flags::None` is an editable field that selects all
/// text on focus and lets Enter move to the next control.
pub enum Flags {
    /// Raise an event when Enter is pressed instead of moving focus.
    ProcessEnter             = 0x0001,
    /// Prevent editing; the text can only be viewed and selected.
    Readonly                 = 0x0002,
    /// Do not select the entire contents when the field receives focus.
    DisableAutoSelectOnFocus = 0x0004,
}