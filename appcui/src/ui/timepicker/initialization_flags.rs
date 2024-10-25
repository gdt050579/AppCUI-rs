use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    HM           = 0x0001,
    HMS          = 0x0002,
    HMS_MS       = 0x0004,
    HM_AMPM      = 0x0008,
    HMS_AMPM     = 0x0010,
    HMS_MS_AMPM  = 0x0020,
}