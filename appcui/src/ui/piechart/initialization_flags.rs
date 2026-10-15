use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::PieChart`].
///
/// Combine values with `|`. `Flags::None` draws the chart without extra behavior.
pub enum Flags {
    /// Draws every slice except the selected one with the inactive chart color.
    DimUnselected = 1,
}
