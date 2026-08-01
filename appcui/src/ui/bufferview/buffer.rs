/// Random-access storage backing a [`super::BufferView`].
///
/// Implement this trait to connect arbitrary data sources (files, memory-mapped regions,
/// sparse buffers, and so on) to the control. A [`Vec<u8>`] implementation is provided.
pub trait BufferAccess : Default {
    /// Returns the number of bytes currently available in the buffer.
    fn count(&self) -> u64;
    /// Returns the byte at `pos`, or `None` if the position is not readable.
    fn get(&mut self, pos: u64) -> Option<u8>;
    /// Returns whether individual bytes can be modified through [`Self::set`].
    fn can_write(&self) -> bool;
    /// Writes `value` at `pos`. Returns `false` if the write is not allowed or out of range.
    fn set(&mut self, pos: u64, value: u8) -> bool;
    /// Returns whether the buffer length can be changed through [`Self::resize`].
    fn can_resize(&self) -> bool;
    /// Changes the buffer length to `new_size`, filling any newly added bytes with `fill_byte`.
    /// Returns `false` if resizing is not supported.
    fn resize(&mut self, new_size: u64, fill_byte: u8) -> bool;
}

pub(super) struct Buffer<T: BufferAccess> {
    data: T,
}
impl<T: BufferAccess> Buffer<T> {
    pub(super) fn new(data: T) -> Self {
        Self { data }
    }
    #[inline(always)]
    pub(super) fn take(&mut self) -> T {
        std::mem::take(&mut self.data)
    }
    #[inline(always)]
    pub(super) fn len(&self) -> u64 {
        self.data.count()
    }
    #[inline(always)]
    pub(super) fn get(&mut self, pos: u64) -> Option<u8> {
        self.data.get(pos)
    }
    pub(super) fn read_bytes_exact<const N: usize>(&mut self, pos: u64, output: &mut [u8; N]) -> bool {
        if let Some(new_addr) = pos.checked_add(N as u64) {
            if new_addr > self.len() {
                false
            } else {
                let mut pos = pos;
                for o in output.iter_mut() {
                    if let Some(b) = self.data.get(pos) {
                        *o = b;
                        pos += 1;
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
    pub(super) fn read_bytes(&mut self, pos: u64, output: &mut [u8]) -> u64 {
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
    pub(super) fn overwrite_bytes(&mut self, pos: u64, bytes: &[u8]) -> bool {
        if pos.saturating_add(bytes.len() as u64) > self.len() {
            return false;
        }
        for (p, b) in (pos..).zip(bytes.iter()) {
            if !self.data.set(p, *b) {
                return false;
            }
        }
        true
    }
    pub(super) fn write_bytes(&mut self, pos: u64, bytes: &[u8]) -> bool {
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
        let len = self.data.count();
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
        self.data.resize(len - count, 0u8)
    }
    pub(super) fn insert(&mut self, pos: u64, bytes: &[u8]) -> bool {
        let old_len = self.data.count();
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
    pub(super) fn fill(&mut self, pos: u64, count: u64, value: u8) -> bool {
        if count == 0 {
            return true;
        }
        if !self.data.can_write() {
            return false;
        }
        match pos.checked_add(count) {
            Some(end) if end <= self.data.count() => {}
            _ => return false,
        }
        let mut p = pos;
        let end = pos + count;
        while p < end {
            if !self.data.set(p, value) {
                return false;
            }
            p += 1;
        }
        true
    }
}

impl BufferAccess for Vec<u8> {
    fn count(&self) -> u64 {
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
