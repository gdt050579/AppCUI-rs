    use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    AMPM = 0x0001,
    Seconds = 0x0002,
}
