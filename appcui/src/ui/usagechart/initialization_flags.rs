use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::UsageChart`].
///
/// Combine values with `|`. `Flags::None` draws the bar without a margin and without dimming.
pub enum Flags {
    /// Insets the bar by one cell on every side.
    ///
    /// The chart's minimum size becomes 3×3. Without this flag the minimum size is 3×1
    /// and the bar fills the control.
    Margins = 0x0001,
    /// Draws every segment except the selected one with the inactive chart color.
    ///
    /// The selected segment keeps its own color and is outlined. The outline stays
    /// visible when the chart does not have the focus.
    DimSegmentsOnSelection = 0x0002,
}

/// Order of the segments in a [`struct@super::UsageChart`], from left to right.
///
/// [`None`](Self::None) is the default. Lengths are still given to the largest values
/// first, so a segment that does not fit is omitted whatever this order says.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SortType {
    /// Keeps the order in which the segments were added.
    None,
    /// Largest value first.
    HighestToLowest,
    /// Smallest value first.
    LowestToHighest,
    /// Compares labels with the usual string order. An empty label comes first.
    Alphabetical,
    /// The reverse of [`Alphabetical`](Self::Alphabetical).
    ReverseAlphabetical,
}

/// What the full width of a [`struct@super::UsageChart`] represents.
///
/// [`Auto`](Self::Auto) is the default.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TotalUsage {
    /// The total is the sum of the segment values, so the segments fill the bar.
    Auto,
    /// The total is a fixed quantity. The unused part of the bar stays empty.
    ///
    /// A total of `0` draws no segments.
    Fixed(u64),
}

/// What the hover tooltip of a segment shows.
///
/// [`LabelAndValue`](Self::LabelAndValue) is the default. A value is written with the
/// chart's number format. A percentage always has two decimals and a `%` suffix, and
/// does not use that format. An empty tooltip (a [`Label`](Self::Label) on a segment
/// that has no label) is not shown.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum LabelContent {
    /// The value of the segment.
    Value,
    /// The label of the segment.
    Label,
    /// The segment's share of the total usage, with two decimals (for example `18.75%`).
    Percentage,
    /// The label and the value, as `label: value` (just the value if the label is empty). This is the default.
    #[default]
    LabelAndValue,
    /// The label and the percentage, as `label: percentage` (just the percentage if the label is empty).
    LabelAndPercentage,
}