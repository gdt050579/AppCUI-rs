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
}