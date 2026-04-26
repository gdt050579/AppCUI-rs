pub(super) struct LineChunkSplitter<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> LineChunkSplitter<'a> {
    pub(super) const MAX_CHUNK_SIZE: usize = 1024;
    pub(super) const MIN_CHUNK_SIZE: usize = 512;
    pub(super) const TARGET_CHUNK_SIZE: usize = 768;
    pub(super) fn new(text: &'a str) -> Self {
        Self { text, pos: 0 }
    }
}

impl<'a> Iterator for LineChunkSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let bytes = self.text.as_bytes();
        let len = bytes.len();

        if self.pos >= len {
            return None;
        }

        let remaining = len - self.pos;

        // Last piece: if what's left fits in MAX, return it whole.
        if remaining <= Self::MAX_CHUNK_SIZE {
            let slice = &self.text[self.pos..];
            self.pos = len;
            return Some(slice);
        }
        let ideal_end = self.pos + Self::TARGET_CHUNK_SIZE;
        let max_end = (self.pos + Self::MAX_CHUNK_SIZE).min(len.saturating_sub(Self::MIN_CHUNK_SIZE));
        let min_end = self.pos + Self::MIN_CHUNK_SIZE;
        let mut end = ideal_end.clamp(min_end, max_end);

        // Walk back to a char boundary. Bounded by 3 bytes.
        while end > min_end && !self.text.is_char_boundary(end) {
            end -= 1;
        }
        while !self.text.is_char_boundary(end) && end < max_end {
            end += 1;
        }

        let slice = &self.text[self.pos..end];
        self.pos = end;
        Some(slice)
    }
}
