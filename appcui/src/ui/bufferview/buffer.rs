pub trait BufferAccess {
    fn len(&self) -> u64;
    fn get(&mut self, pos: u64) -> Option<u8>;
    fn can_write(&self) -> bool;
    fn set(&mut self, pos: u64, value: u8) -> bool;
    fn can_resize(&self) -> bool;
    fn resize(&mut self, new_size: u64, fill_byte: u8) -> bool;
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
    pub fn read_bytes_exact<const N: usize>(&mut self, pos: u64, output: &mut [u8; N]) -> bool {
        if let Some(new_addr) = pos.checked_add(N as u64) {
            if new_addr > self.len() {
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
        } else {
            false
        }
    }
    pub fn read_bytes(&mut self, pos: u64, output: &mut [u8]) -> u64 {
        let len = self.len();
        let mut i = 0;
        let mut pos = pos;
        while pos < len && i < output.len() {
            if let Some(b) = self.data.get(pos) {
                output[i] = b;
                i += 1;
                pos += 1;
            } else {
                break;
            }
        }
        i as u64
    }
    pub fn overwrite_bytes(&mut self, pos: u64, bytes: &[u8]) -> bool {
        if pos.saturating_add(bytes.len() as u64) > self.len() {
            return false;
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
    pub fn write_bytes(&mut self, pos: u64, bytes: &[u8]) -> bool {
        let new_len = pos.saturating_add(bytes.len() as u64);
        if new_len > self.len() {
            if !self.can_resize() {
                return false;
            }
            if !self.data.resize(new_len, 0u8) {
                return false;
            }
        }
        self.overwrite_bytes(pos, bytes)
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
        let len = self.data.len();
        if !self.data.can_resize() {
            return false;
        }
        if count == 0 || pos >= len {
            return true;                
        }
        let count = count.min(len - pos);
        let mut src = pos + count;
        let mut dst = pos;
        let mut temp = [0u8; 4096];
        while src < len {
            let n = self.read_bytes(src, &mut temp);
            if n == 0 {
                break;
            }
            if !self.overwrite_bytes(dst, &temp[..n as usize]) {
                return false;
            }
            src += n;
            dst += n;
        }
        self.data.resize(len - count,0u8)
    }
    pub(super) fn insert(&mut self, pos: u64, bytes: &[u8]) -> bool {
        let old_len = self.data.len();
        if pos > old_len || !self.data.can_resize() {
            return false;
        }
        if bytes.is_empty() {
            return true;
        }
        let k = bytes.len() as u64;
        if !self.data.resize(old_len + k, 0u8) {
            return false;
        }
        let mut remaining = old_len - pos;
        let mut temp = [0u8; 4096];
        while remaining > 0 {
            let chunk = remaining.min(4096);
            let src = pos + remaining - chunk;
            let dst = src + k;
            let n = self.read_bytes(src, &mut temp[..chunk as usize]);
            if n != chunk {
                return false;
            }
            if !self.overwrite_bytes(dst, &temp[..n as usize]) {
                return false;
            }
            remaining -= chunk;
        }
        self.overwrite_bytes(pos, bytes)
    }
    #[inline(always)]
    pub(super) fn resize(&mut self, new_size: u64, fill_byte: u8) -> bool {
        if self.data.can_resize() {
            self.data.resize(new_size, fill_byte)
        } else {
            false
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
    fn can_resize(&self) -> bool {
        true
    }
    fn resize(&mut self, new_size: u64, fill_byte: u8) -> bool {
        self.resize(new_size as usize, fill_byte);
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
