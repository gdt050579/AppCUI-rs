pub(super) struct OutputBuffer {
    buffer: [u8; 31],
    len: u8,
}
impl OutputBuffer {
    pub(super) fn new() -> Self {
        Self { buffer: [0; 31], len: 0 }
    }
    #[inline(always)]
    pub(super) fn set(&mut self, index: usize, value: u8) {
        self.buffer[index] = value;
    }
    #[inline(always)]
    pub(super) fn set_len(&mut self, len: u8) {
        self.len = len;
    }
}
