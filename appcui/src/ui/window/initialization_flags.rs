use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=16)]
pub enum Flags {
    Sizeable      = 0x0001,
    NotifyWindow  = 0x0002,
    ErrorWindow   = 0x0004,
    WarningWindow = 0x0008,
    NoCloseButton = 0x0010,
    FixedPosition = 0x0040,
    //Maximized     = 0x0080,
    //ProcessReturn = 0x0200,
}