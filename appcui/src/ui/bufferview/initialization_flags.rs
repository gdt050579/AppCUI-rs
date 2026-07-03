use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16)]
pub enum Flags {
    ScrollBars = 0x0001,
    SearchBar = 0x0002,
    HideHeader = 0x0004,
    ShowAddress = 0x0008,
    ShowIntervalNames = 0x0010,
    NoPanelDimming = 0x0020,
    ShowAsciiStrings = 0x0040,
    ShowUnicodeStrings = 0x0080,
    DecodeUTF8Characters = 0x0100,
}

pub trait BufferAccess {
    fn len(&self) -> u64;
    fn byte(&mut self, pos: u64) -> Option<u8>;
    fn slice(&mut self, pos: u64, len: u16) -> Option<&[u8]>;
    fn copy(&mut self, pos: u64, len: u16, output: &mut Vec<u8>) {
        output.clear();
        if let Some(slice) = self.slice(pos, len) {
            output.extend_from_slice(slice);
        }
    }
}

impl BufferAccess for Vec<u8> {
    fn len(&self) -> u64 {
        self.len() as u64
    }
    fn byte(&mut self, pos: u64) -> Option<u8> {
        if pos < self.len() as u64 {
            Some(self[pos as usize])
        } else {
            None
        }
    }
    fn slice(&mut self, pos: u64, len: u16) -> Option<&[u8]> {
        let buffer_len = self.len() as u64;
        if pos >= buffer_len || len == 0 {
            return None;
        }
        let slice_len = (buffer_len - pos).min(len as u64) as usize;
        Some(&self[pos as usize..pos as usize + slice_len])
    }
}