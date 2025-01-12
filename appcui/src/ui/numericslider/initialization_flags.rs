use EnumBitFlags::EnumBitFlags;


#[EnumBitFlags(bits=8)]
pub enum Flags {
    ValuesUp                   = 0x0001,
    SingleLine                 = 0x0002,
    DoubleLine                 = 0x0004
}