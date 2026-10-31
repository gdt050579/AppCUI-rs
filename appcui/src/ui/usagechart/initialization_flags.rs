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