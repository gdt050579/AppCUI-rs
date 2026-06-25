use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x0001,
    HideHeader = 0x0002,
    ShowAddress = 0x0004,
    ShowLabels = 0x0008,
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
