use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    Seconds     = 0x0001,
    Miliseconds = 0x0002,
    AMPMFormat  = 0x0004,
}