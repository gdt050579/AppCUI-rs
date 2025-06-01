use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    AllowNoneSelection = 0x01,
    ShowDescription = 0x02,
}

pub trait DropDownListType {
    fn name(&self) -> &str;
    fn description(&self) -> &str {
        ""
    }
    fn symbol(&self) -> &str {
        ""
    }
}
