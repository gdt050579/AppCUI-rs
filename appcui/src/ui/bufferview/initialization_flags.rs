use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    ScrollBars = 0x0001,
    SearchBar = 0x0002,
    HideHeader = 0x0004,
    ShowAddress = 0x0008,
    ShowIntervalNames = 0x0010,
    NoPanelDimming = 0x0020,
}

pub trait BufferAccess {
    fn len(&self) -> u64;
    fn byte(&self, pos: u64) -> Option<u8>;
    fn copy(&self, pos: u64, len: u64, output: &mut Vec<u8>);
}

impl BufferAccess for Vec<u8> {
    fn len(&self) -> u64 {
        self.len() as u64
    }
    fn byte(&self, pos: u64) -> Option<u8> {
        if pos < self.len() as u64 {
            Some(self[pos as usize])
        } else {
            None
        }
    }
    fn copy(&self, pos: u64, len: u64, output: &mut Vec<u8>) {
        output.clear();
        output.reserve(len as usize);
        if pos < self.len() as u64 {
            let end = (pos + len).min(self.len() as u64);
            output.extend_from_slice(&self[pos as usize..end as usize]);
        }
    }
}
