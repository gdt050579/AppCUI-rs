use EnumBitFlags::EnumBitFlags;


#[EnumBitFlags(bits=8)]
pub enum Flags {
    OnTop                      = 0x0001,
    DoubleLine                 = 0x0002
}