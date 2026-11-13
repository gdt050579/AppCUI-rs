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
    /// Adds one empty character around the heat map.
    ///
    /// The border is part of the image. It shows at the start and at the end of the scroll, and it moves with the cells. The border uses the chart background.
    Margins = 2,
}
