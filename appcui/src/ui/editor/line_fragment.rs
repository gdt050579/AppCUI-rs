use std::ops::Range;

/// A contiguous line fragment with cached metadata.
pub(super) struct LineFragment {
    pub(super) data: String,
    pub(super) is_ascii: bool,
    pub(super) has_tabs: bool,
    pub(super) has_multiline_spread: bool, // e.g. a multiline comment
}

impl LineFragment {
    pub(super) fn new(text: &str) -> Self {
        Self {
            data: text.to_string(),
            is_ascii: true,
            has_tabs: false,
            has_multiline_spread: false,
        }
    }
    pub(super) fn insert_char(&mut self, byte_offset: u32, ch: char) -> u32 {
        self.data.insert(byte_offset as usize, ch);
        byte_offset + ch.len_utf8() as u32
    }
    pub(super) fn delete_char(&mut self, byte_offset: u32) {
        self.data.remove(byte_offset as usize);
    }
    pub(super) fn insert_string(&mut self, offset: u32, text: &str) -> u32 {
        self.data.insert_str(offset as usize, text);
        offset + text.len() as u32
    }
    pub(super) fn delete_range(&mut self, range: Range<u32>) {
        self.data.drain(range.start as usize..range.end as usize);
    }
}
