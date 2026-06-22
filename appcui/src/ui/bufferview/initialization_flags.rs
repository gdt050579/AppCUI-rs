use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x0001,
}

pub trait BufferAccess {
    fn len(&self) -> usize;
    fn copy(&self, pos: usize, len: usize, output: &Vec<u8>);
}