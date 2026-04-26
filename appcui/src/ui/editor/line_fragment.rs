use std::ops::Range;

/// A contiguous line fragment with cached metadata.
pub(crate) struct LineFragment {
    pub(crate) data: Vec<u8>, // less than a THRESHOLD (e.g. 4096 bytes)
    pub(crate) chars: u16,
    pub(crate) is_ascii: bool,
    pub(crate) has_tabs: bool,
    pub(crate) has_multiline_spread: bool, // e.g. a multiline comment
}

impl LineFragment {
    /// Inserts `ch` at byte `offset`. `offset` must lie on a UTF-8 boundary.
    /// Returns the byte offset just past the inserted character.
    fn insert_char(&mut self, offset: u32, ch: char) -> u32 {
        let off = offset as usize;
        let written = if (ch as u32) < 0x80 {
            self.data.insert(off, ch as u8);
            if ch == '\t' {
                self.has_tabs = true;
            }            
            1
        } else {
            let mut buffer = [0u8; 4];
            let encoded = ch.encode_utf8(&mut buffer);
            let bytes = encoded.as_bytes();
            self.data.splice(off..off, bytes.iter().copied());
            self.is_ascii = false;
            bytes.len()
        };

        self.chars = self.chars.saturating_add(1);
        offset + written as u32
    }

    /// Deletes the single UTF-8 character starting at `offset`.
    /// `offset` must lie on a UTF-8 boundary.
    fn delete_char(&mut self, offset: u32) {
        let start = offset as usize;
        if start >= self.data.len() {
            return;
        }
        let chw = utf8_char_width(self.data[start]);
        self.data.drain(start..start + chw);
        self.chars = self.chars.saturating_sub(1);
    }

    /// Inserts a UTF-8 byte slice at `offset`.
    /// `buffer` must be valid UTF-8; `offset` must lie on a UTF-8 boundary.
    /// Returns the byte offset just past the inserted bytes.
    fn insert_buffer(&mut self, offset: u32, buffer: &[u8]) -> u32 {
        let off = offset as usize;
        debug_assert!(std::str::from_utf8(buffer).is_ok());

        if buffer.is_empty() {
            return offset;
        }

        let inserted_chars = count_chars(buffer);
        let buffer_is_ascii = buffer.is_ascii();
        let buffer_has_tabs = buffer.contains(&b'\t');

        self.data.splice(off..off, buffer.iter().copied());

        self.chars = self.chars.saturating_add(inserted_chars);
        if !buffer_is_ascii {
            self.is_ascii = false;
        }
        if buffer_has_tabs {
            self.has_tabs = true;
        }

        offset + buffer.len() as u32
    }

    /// Deletes the byte range `range`. Both endpoints must lie on UTF-8 boundaries.
    fn delete_range(&mut self, range: Range<u32>) {
        let start = range.start as usize;
        let end = range.end as usize;
        if start >= end || end > self.data.len() {
            return;
        }
        self.data.drain(start..end);
    }
}

/// Number of bytes in a UTF-8 codepoint given its lead byte.
#[inline]
fn utf8_char_width(b: u8) -> usize {
    match b {
        0x00..=0x7F => 1,
        0xC2..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF4 => 4,
        _ => 1, // invalid lead byte; treat conservatively
    }
}

/// Counts UTF-8 codepoints in a valid UTF-8 byte slice.
#[inline]
fn count_chars(bytes: &[u8]) -> u16 {
    // A byte is a UTF-8 continuation byte iff its top two bits are 0b10.
    let n = bytes.iter().filter(|&&b| (b & 0xC0) != 0x80).count();
    n as u16
}