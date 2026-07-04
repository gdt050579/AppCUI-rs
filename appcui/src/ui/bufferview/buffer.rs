
pub trait BufferAccess {
    fn len(&self) -> u64;
    fn get(&mut self, pos: u64) -> Option<u8>;

}

pub struct Buffer<T: BufferAccess> {
    data: T,
}
impl<T: BufferAccess> Buffer<T> {
    pub(super) fn new(data: T) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub fn len(&self) -> u64 {
        self.data.len()
    }
    #[inline(always)]
    pub fn get(&mut self, pos: u64) -> Option<u8> {
        self.data.get(pos)
    }
    pub fn read_bytes<const N: usize>(&mut self, pos: u64, output: &mut [u8; N]) -> bool {
        if pos + N as u64 > self.len() {
            false
        } else {
            for i in 0..N {
                if let Some(b) = self.data.get(pos + i as u64) { 
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