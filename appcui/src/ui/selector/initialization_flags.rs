use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
pub enum Flags {
    AllowNoneVariant = 0x01,
}

pub trait EnumSelector {
    const COUNT: u32;
    fn from_index(index: u32) -> Option<Self> where Self: Sized;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str {
        ""
    }
}
