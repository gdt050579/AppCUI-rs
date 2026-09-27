use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits: 8)]
/// Initialization flags for a [`struct@super::ProgressBar`].
///
/// Combine values with `|`. `Flags::None` shows the fill bar and a percentage
/// such as `42%`.
pub enum Flags {
    /// Hide the percentage text and show only the fill bar.
    HidePercentage = 0x01,
}