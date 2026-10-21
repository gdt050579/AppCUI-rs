use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::LineChart`].
///
/// Combine values with `|`. `Flags::None` draws the chart without extra decorations.
pub enum Flags {
    /// Shows a legend that maps each series color to its label.
    ShowLegend = 1,
}
