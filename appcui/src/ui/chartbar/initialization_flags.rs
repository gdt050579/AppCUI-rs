use EnumBitFlags::EnumBitFlags;

#[repr(u8)]
#[derive(Clone, Copy,PartialEq,Eq)]
pub enum Type{
    VerticalBar,
    Line
}

#[repr(u8)]
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Fit
{
    FitToHeight,
    None
}

#[repr(u8)]
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum YAxes
{
    MinMax(i32,i32),
    Auto,
    Visible,
}

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x0001,
    AutoScroll = 0x0002,
    FitToHeight = 0x0004,
    Line = 0x008,
    VerticalBar = 0x0010,
    ManualYAxesSize = 0x0020,
    AdaptivYAXesOnView = 0x0040,
    AdaptivYAXesOnData = 0x0080,
}
