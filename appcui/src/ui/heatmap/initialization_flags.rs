use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::HeatMap`].
///
/// Combine values with `|`. `Flags::None` creates a heat map with the default behavior.
pub enum Flags {}
