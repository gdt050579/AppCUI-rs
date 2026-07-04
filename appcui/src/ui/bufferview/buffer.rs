
pub trait BufferAccess {
    fn len(&self) -> u64;
    fn get(&mut self, pos: u64) -> Option<u8>;
    fn can_write(&self) -> bool;
    fn set(&mut self, pos: u64, value: u8) -> bool;
    fn can_resize(&self) -> bool;
    fn resize(&mut self, new_size: u64) -> bool;
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
    pub fn write_bytes(&mut self, pos: u64, bytes: &[u8]) -> bool {
        if pos + bytes.len() as u64 > self.len() {
            if !self.data.resize(pos + bytes.len() as u64) {
                return false;
            }
        }
        let mut p = pos;
        for b in bytes {
            if !self.data.set(p, *b) {
                return false;
            }
            p += 1;
        }
        true
    }
    #[inline(always)]
    pub(super) fn can_edit(&self) -> bool {
        self.data.can_write()
    }
    #[inline(always)]
    pub(super) fn can_resize(&self) -> bool {
        self.data.can_resize()
    }
    pub(super) fn delete(&mut self, pos: u64, count: u64) -> bool {
        if pos > self.data.len() || count == 0 || !self.data.can_resize() {
            return false;
        }
        let new_len = (self.data.len() - pos).min(count);
        todo!()
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
    fn can_resize(&self) -> bool {
        true
    }
    fn resize(&mut self, new_size: u64) -> bool {
        self.resize(new_size as usize, 0u8);
        true
    }
    fn can_write(&self) -> bool {
        true
    }
    fn set(&mut self, pos: u64, value: u8) -> bool {
        if pos < self.len() as u64 {
            self[pos as usize] = value;
            true
        } else {
            false
        }
    }
}