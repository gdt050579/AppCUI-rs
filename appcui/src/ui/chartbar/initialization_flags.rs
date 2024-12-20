use EnumBitFlags::EnumBitFlags;

#[repr(u8)]
#[derive(Clone, Copy,PartialEq,Eq)]
pub enum Type{
    VerticalBar,
    HorizontalBar,
    Line
}

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x0001,
    SearchBar = 0x0002,
    CheckBoxes = 0x0004,
    AutoScroll = 0x0008,
    HighlightSelectedItemWhenInactive = 0x0010,
}


#[repr(u8)]
#[derive(Clone, Copy,PartialEq,Eq)]
pub enum LineDistance
{
    With(u8),
    Without
}

impl LineDistance
{
    pub fn extract_integer(v: &LineDistance) -> u8 
    {
     match v 
     {
        LineDistance::With(ivalue) => return *ivalue,
        _ => return 0,
     }
    }
} 