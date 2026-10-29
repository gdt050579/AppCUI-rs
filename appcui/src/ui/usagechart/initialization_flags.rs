use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x0001, // de vazut daca chiar are sens !!!!
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