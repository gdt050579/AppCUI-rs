pub(crate) struct OutputBuffer {
    buffer: [u8; 31],
    len: u8,
}
impl OutputBuffer {
    pub(crate) fn new() -> Self {
        Self { buffer: [0; 31], len: 0 }
    }
    #[inline(always)]
    pub(crate) fn set(&mut self, index: usize, value: u8) {
        self.buffer[index] = value;
    }
    #[inline(always)]
    pub(crate) fn set_len(&mut self, len: u8) {
        self.len = len;
    }
    #[inline(always)]
    pub(crate) fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.len as usize]
    }
}
