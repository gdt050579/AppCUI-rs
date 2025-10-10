use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=16)]
pub enum Flags {
    Sizeable      = 0x0001,
    NoCloseButton = 0x0002,
    FixedPosition = 0x0004,
}
#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,Default)]
pub enum Type {
    #[default]
    Classic,
    Rounded,
    Panel,
}


#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq,Default)]
pub enum Background {
    #[default]
    Normal,
    Error,
    Warning,
    Notification
}