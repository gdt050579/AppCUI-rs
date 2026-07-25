use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    DoubleLine = 0x0001,
    HasTitle = 0x0002,
    MergeBorders = 0x0004,
}