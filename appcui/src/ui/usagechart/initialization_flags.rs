use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    Margins = 0x0001,
    DimSegmentsOnSelection = 0x0002,
}

pub enum SortType {
    None,
    HighestToLowest,
    LowestToHighest,
    Alphabetical,
    ReverseAlphabetical,
}

pub enum TotalUsage {
    Auto,
    Fixed(u64)
}