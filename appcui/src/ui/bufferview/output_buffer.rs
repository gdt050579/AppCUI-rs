pub(super) struct OutputBuffer {
    buffer: [u8; 31],
    len: u8,
}
impl OutputBuffer {
    pub(super) fn new() -> Self {
        Self { buffer: [0; 31], len: 0 }
    }
}
