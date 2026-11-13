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
    /// Draws every cell except the selected one with the inactive chart color.
    ///
    /// The selected cell keeps its own color and is outlined. The outline stays
    /// visible when the heat map does not have the focus.
    DimCellsOnSelection = 4,
}

/// What the hover tooltip of a cell shows.
///
/// [`LabelAndValue`](Self::LabelAndValue) is the default. A value is written with
/// the cell's number. A percentage is the cell's place between the smallest and
/// the largest value, from `0%` to `100%`. An empty tooltip (a [`Label`](Self::Label)
/// on a cell that has no label) is not shown. An empty cell is not shown either.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum TooltipFormat {
    /// The value of the cell.
    Value,
    /// The label of the cell.
    Label,
    /// The cell's place between the smallest and the largest value (for example `42%`).
    Percentage,
    /// The label and the value, as `label: value` (just the value if the label is empty). This is the default.
    #[default]
    LabelAndValue,
    /// The label and the percentage, as `label: percentage` (just the percentage if the label is empty).
    LabelAndPercentage,
}
