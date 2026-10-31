use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    Margins = 0x0001,
    DimSegmentsOnSelection = 0x0002,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SortType {
    None,
    HighestToLowest,
    LowestToHighest,
    Alphabetical,
    ReverseAlphabetical,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TotalUsage {
    Auto,
    Fixed(u64),
}

/// What the tooltip of a segment shows.
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