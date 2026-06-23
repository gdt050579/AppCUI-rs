use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x0001,
}


pub enum ViewMode {
    Hex(u8),
    HexU16(u8),
    UnsignedDecimal(u8),
    SignedDecimal(u8),
    UnsignedDecimalU16(u8),
    SignedDecimalU16(u8),
    UnsignedDecimalU32(u8),
    SignedDecimalU32(u8),
    UnsignedDecimalU64(u8),
    SignedDecimalU64(u8),
    UnsignedDecimalU128(u8),
    SignedDecimalU128(u8),
}

pub trait BufferAccess {
    fn len(&self) -> usize;
    fn byte(&self, pos: usize) -> Option<u8>;
    fn copy(&self, pos: usize, len: usize, output: &mut Vec<u8>);
}

impl BufferAccess for Vec<u8> {
    fn len(&self) -> usize {
        self.len()
    }
    fn byte(&self, pos: usize) -> Option<u8> {
        if pos < self.len() {
            Some(self[pos])
        } else {
            None
        }
    }
    fn copy(&self, pos: usize, len: usize, output: &mut Vec<u8>) {
        output.clear();
        output.reserve(len);
        if pos < self.len() {
            let end = (pos + len).min(self.len());
            output.extend_from_slice(&self[pos..end]);
        }
    }
}
