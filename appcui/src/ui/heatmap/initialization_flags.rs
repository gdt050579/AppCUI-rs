use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::HeatMap`].
///
/// Combine values with `|`. `Flags::None` creates a heat map with the default behavior.
pub enum Flags {
    /// Shows horizontal and vertical scroll bars when the heat map is larger than the control.
    ///
    /// The right and bottom margins grow while the heat map has focus so the bars stay visible.
    ScrollBars = 1,
}
