use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
/// Initialization flags for a [`struct@super::ComboBox`].
///
/// Combine values with `|`. `Flags::None` shows only each item's name in the
/// closed control.
pub enum Flags {
    /// Show each item's description next to its name.
    ShowDescription = 0x01,
}