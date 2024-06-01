use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    HideButtons = 0x0001,
    ReadOnly = 0x0002,
}


#[derive(Clone, Copy)]
pub enum Format {
    Decimal,
    Percentage,
    DigitGrouping,    
}