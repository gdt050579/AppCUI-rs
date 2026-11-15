use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 1,
    DimBarsOnSelection = 2,
    ShowZeroLineOnYAxis = 4,    
}