use EnumBitFlags::EnumBitFlags;

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Type {
    Standard,
    ProgressBar,
}


#[EnumBitFlags(bits=16)]
pub enum Flags {
    None      = 0x0000,
    ShowValue = 0x0001,
    Ticks     = 0x0004,
}
