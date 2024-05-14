use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
pub enum Flags {
    AllowNoneVariant = 0x01,
}

pub trait EnumSelector {
    fn count() -> u32;
    fn from_index(index: u32) -> Option<Self> where Self: Sized;
    fn name(&self) -> &str;
    fn description(&self) -> &str {
        ""
    }
}
