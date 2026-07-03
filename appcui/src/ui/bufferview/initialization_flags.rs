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
    fn get(&mut self, pos: u64) -> Option<u8>;
    fn read_bytes<const N: usize>(&mut self, pos: u64, output: &mut [u8; N]) -> bool {
        if pos + N as u64 > self.len() {
            false
        } else {
            for i in 0..N {
                if let Some(b) = self.get(pos + i as u64) { 
                    output[i] = b;
                } else {
                    return false;
                }
            }
            true
        }
    }
}

impl BufferAccess for Vec<u8> {
    fn len(&self) -> u64 {
        self.len() as u64
    }
    fn get(&mut self, pos: u64) -> Option<u8> {
        if pos < self.len() as u64 {
            Some(self[pos as usize])
        } else {
            None
        }
    }
}