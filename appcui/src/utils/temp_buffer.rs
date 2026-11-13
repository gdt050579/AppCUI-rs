struct StackBuffer<const N: usize> {
    buffer: [u8; N],
    len: usize,
}
enum InnerTempBuffer<const N: usize> {
    StackBuffer(StackBuffer<N>),
    HeapBuffer(Vec<u8>),
}   
pub(crate) struct TempBuffer<const N: usize> {
    inner: InnerTempBuffer<N>,
}
impl<const N: usize> TempBuffer<N> {
    pub(crate) fn new(slice: &[u8]) -> Self {
        Self {
            inner: if N >= slice.len() {
                InnerTempBuffer::StackBuffer(StackBuffer {
                    buffer: {
                        let mut buffer = [0; N];
                        buffer[..slice.len()].copy_from_slice(slice);
                        buffer
                    },
                    len: slice.len(),
                })
            } else {
                InnerTempBuffer::HeapBuffer(slice.to_vec())
            },
        }
    }
    #[inline(always)]
    pub(crate) fn as_slice(&self) -> &[u8] {
        match &self.inner {
            InnerTempBuffer::StackBuffer(buffer) => &buffer.buffer[..buffer.len],
            InnerTempBuffer::HeapBuffer(buffer) => buffer.as_slice(),
        }
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn is_on_heap(&self) -> bool {
        matches!(&self.inner, InnerTempBuffer::HeapBuffer(_))
    }
    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        match &self.inner {
            InnerTempBuffer::StackBuffer(buffer) => buffer.len == 0,
            InnerTempBuffer::HeapBuffer(buffer) => buffer.is_empty(),
        }
    }
    /// Drops the contents. A heap buffer stays on the heap.
    pub(crate) fn clear(&mut self) {
        match &mut self.inner {
            InnerTempBuffer::StackBuffer(buffer) => buffer.len = 0,
            InnerTempBuffer::HeapBuffer(buffer) => buffer.clear(),
        }
    }
    /// Drops the contents and moves a heap buffer back to the stack.
    pub(crate) fn reset(&mut self) {
        if matches!(self.inner, InnerTempBuffer::HeapBuffer(_)) {
            self.inner = InnerTempBuffer::StackBuffer(StackBuffer { buffer: [0; N], len: 0 });
        } else {
            self.clear();
        }
    }
    fn push_slice(&mut self, slice: &[u8]) {
        if slice.is_empty() {
            return;
        }
        match &mut self.inner {
            InnerTempBuffer::StackBuffer(buffer) => {
                let new_len = buffer.len + slice.len();
                if new_len <= N {
                    buffer.buffer[buffer.len..new_len].copy_from_slice(slice);
                    buffer.len = new_len;
                } else {
                    let mut heap = Vec::with_capacity(new_len);
                    heap.extend_from_slice(&buffer.buffer[..buffer.len]);
                    heap.extend_from_slice(slice);
                    self.inner = InnerTempBuffer::HeapBuffer(heap);
                }
            }
            InnerTempBuffer::HeapBuffer(buffer) => buffer.extend_from_slice(slice),
        }
    }
}

pub(crate) struct TempString<const N: usize> {
    buffer: TempBuffer<N>,
}
impl<const N: usize> TempString<N> {
    pub(crate) fn new(slice: &str) -> Self {
        Self {
            buffer: TempBuffer::new(slice.as_bytes()),
        }
    }
    #[inline(always)]
    pub(crate) fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.buffer.as_slice()) }
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn is_on_heap(&self) -> bool {
        self.buffer.is_on_heap()
    }
    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    #[inline(always)]
    pub(crate) fn push_str(&mut self, text: &str) {
        self.buffer.push_slice(text.as_bytes());
    }
    #[inline(always)]
    pub(crate) fn clear(&mut self) {
        self.buffer.clear();
    }
    #[inline(always)]
    pub(crate) fn reset(&mut self) {
        self.buffer.reset();
    }
    #[inline(always)]
    pub(crate) fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        std::fmt::Write::write_fmt(self, args)
    }
}
impl<const N: usize> std::fmt::Write for TempString<N> {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        self.push_str(text);
        Ok(())
    }
}