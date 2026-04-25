use std::ops::Range;

/// Maximum size of a chunk in bytes.
///
/// This constant is exposed for the document layer to reference. The chunk
/// itself does NOT enforce this limit — chunks may temporarily exceed it
/// during multi-step operations. The document is responsible for splitting
/// oversized chunks at appropriate moments.
pub(crate) const MAX_CHUNK_SIZE: usize = 8192;

// Compile-time check: chunk-local offsets must fit in u16.
const _: () = assert!(MAX_CHUNK_SIZE <= u16::MAX as usize);

pub(crate) struct Chunk {
    /// Raw bytes. Valid UTF-8 only when concatenated with adjacent chunks
    /// in the document; not necessarily independently valid UTF-8.
    bytes: Vec<u8>,

    /// Number of '\n' bytes in `bytes`. Bounded by chunk size.
    line_count: u16,

    /// Offsets within `bytes` where each line starts.
    ///
    /// Invariants:
    /// - `line_starts[0] == 0` always.
    /// - `line_starts.len() == line_count + 1`.
    /// - `line_starts` is strictly monotonically increasing.
    /// - All entries are <= `bytes.len()`.
    /// - Each entry after the first is one past a '\n' byte.
    line_starts: Vec<u16>,

    /// True if all bytes in `bytes` are < 0x80 (pure ASCII).
    /// May be conservatively false.
    is_ascii: bool,

    /// True if any byte in `bytes` is `\t` (0x09).
    /// May be conservatively true.
    has_tabs: bool,
}

impl Chunk {
    fn recompute_line_metadata(&mut self) {
        self.line_count = 0;
        self.line_starts.clear();
        self.line_starts.push(0);
        for (i, &b) in self.bytes.iter().enumerate() {
            if b == b'\n' {
                self.line_count += 1;
                self.line_starts.push((i + 1) as u16);
            }
        }
    }

    /// Create an empty chunk.
    ///
    /// `line_starts` contains `[0]` to satisfy invariants.
    pub(crate) fn new() -> Self {
        Self {
            bytes: Vec::new(),
            line_count: 0,
            line_starts: vec![0],
            is_ascii: true,
            has_tabs: false,
        }
    }

    /// Create a chunk from owned bytes. Computes all metadata in a single pass.
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Self {
        let mut line_count = 0u16;
        let mut line_starts = vec![0u16];
        let mut is_ascii = true;
        let mut has_tabs = false;

        for (i, &b) in bytes.iter().enumerate() {
            if b == b'\n' {
                line_count += 1;
                line_starts.push((i + 1) as u16);
            }
            if b >= 0x80 {
                is_ascii = false;
            }
            if b == b'\t' {
                has_tabs = true;
            }
        }

        let chunk = Self {
            bytes,
            line_count,
            line_starts,
            is_ascii,
            has_tabs,
        };
        debug_assert!(chunk.check_invariants());
        chunk
    }

    /// Create a chunk from borrowed bytes (copies).
    pub(crate) fn from_slice(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes.to_vec())
    }

    /// Length of chunk in bytes.
    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    /// True if chunk contains no bytes.
    pub(crate) fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Number of newlines in chunk.
    pub(crate) fn line_count(&self) -> u16 {
        self.line_count
    }

    /// True if chunk is known to be pure ASCII.
    ///
    /// May be conservatively false even when content is ASCII.
    pub(crate) fn is_ascii(&self) -> bool {
        self.is_ascii
    }

    /// True if chunk is known to contain tab characters.
    ///
    /// May be conservatively true even when no tabs remain.
    pub(crate) fn has_tabs(&self) -> bool {
        self.has_tabs
    }

    /// Read-only access to the chunk's bytes.
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get byte at offset.
    ///
    /// Panics if `offset >= len`.
    pub(crate) fn byte_at(&self, offset: u16) -> u8 {
        self.bytes[offset as usize]
    }

    /// Get a byte slice for the given range.
    ///
    /// Panics if range is invalid or out of bounds.
    pub(crate) fn slice(&self, range: Range<u16>) -> &[u8] {
        &self.bytes[range.start as usize..range.end as usize]
    }

    /// Returns the byte range of the given line within this chunk, excluding the
    /// trailing '\n' (if any). For the last line, the range extends to bytes.len().
    ///
    /// `line_idx` must be `< line_count + 1`. Panics otherwise.
    pub(crate) fn line_range(&self, line_idx: u16) -> Range<u16> {
        let idx = line_idx as usize;
        let start = self.line_starts[idx];
        let end = if idx + 1 < self.line_starts.len() {
            self.line_starts[idx + 1] - 1
        } else {
            self.bytes.len() as u16
        };
        start..end
    }

    /// Returns the line index containing the given byte offset.
    ///
    /// `offset` must be `<= bytes.len()`. Panics otherwise.
    ///
    /// At a newline byte, returns the line that the newline ends.
    /// At `offset == bytes.len()`, returns the last line index.
    pub(crate) fn line_at_offset(&self, offset: u16) -> u16 {
        assert!(offset as usize <= self.bytes.len());
        self.line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1) as u16
    }

    /// Insert bytes at the given offset.
    ///
    /// Updates all metadata. Does NOT enforce `MAX_CHUNK_SIZE`.
    ///
    /// Panics if `offset > self.len()`.
    pub(crate) fn insert(&mut self, offset: u16, new_bytes: &[u8]) {
        assert!(offset as usize <= self.bytes.len());

        if new_bytes.is_empty() {
            return;
        }

        let off = offset as usize;
        self.bytes.splice(off..off, new_bytes.iter().copied());

        for &b in new_bytes {
            if b >= 0x80 {
                self.is_ascii = false;
            }
            if b == b'\t' {
                self.has_tabs = true;
            }
        }

        let insert_len = new_bytes.len() as u16;
        for ls in &mut self.line_starts {
            if *ls > offset {
                *ls += insert_len;
            }
        }

        let mut insert_pos = self.line_starts.partition_point(|&ls| ls <= offset);
        for (i, &b) in new_bytes.iter().enumerate() {
            if b == b'\n' {
                self.line_count += 1;
                let new_line_start = offset + (i as u16) + 1;
                self.line_starts.insert(insert_pos, new_line_start);
                insert_pos += 1;
            }
        }

        debug_assert!(self.check_invariants());
    }

    /// Delete a byte range.
    ///
    /// Updates all metadata. Does NOT enforce a minimum chunk size.
    ///
    /// Panics if range is invalid or out of bounds.
    pub(crate) fn delete(&mut self, range: Range<u16>) {
        assert!(range.start <= range.end);
        assert!(range.end as usize <= self.bytes.len());

        if range.start == range.end {
            return;
        }

        let start = range.start as usize;
        let end = range.end as usize;
        self.bytes.drain(start..end);
        self.recompute_line_metadata();

        debug_assert!(!self.line_starts.is_empty());
        debug_assert!(self.line_starts[0] == 0);
        debug_assert!(self.check_invariants());
    }

    /// Append another chunk's contents to this one. Consumes the other.
    ///
    /// Maintains all invariants. Does NOT enforce size limits.
    pub(crate) fn append(&mut self, other: Chunk) {
        let original_len = self.bytes.len() as u16;

        self.bytes.extend_from_slice(&other.bytes);
        self.line_count += other.line_count;

        for &ls in other.line_starts.iter().skip(1) {
            self.line_starts.push(ls + original_len);
        }

        self.is_ascii = self.is_ascii && other.is_ascii;
        self.has_tabs = self.has_tabs || other.has_tabs;

        debug_assert!(self.check_invariants());
    }

    /// Split the chunk at the given offset, returning the right portion as
    /// a new chunk. Self retains the left portion.
    ///
    /// The split offset must be a valid byte position (`0..=self.len()`) and
    /// on a UTF-8 character boundary. The chunk does NOT verify UTF-8 boundary.
    ///
    /// Panics if `offset > self.len()`.
    pub(crate) fn split_off(&mut self, offset: u16) -> Chunk {
        assert!(offset as usize <= self.bytes.len());

        let off = offset as usize;
        let right_bytes = self.bytes.split_off(off);

        self.recompute_line_metadata();
        let right = Chunk::from_bytes(right_bytes);

        debug_assert!(self.check_invariants());
        debug_assert!(right.check_invariants());

        right
    }

    /// Re-scan bytes to refresh `is_ascii` and `has_tabs` to their accurate values.
    pub(crate) fn refresh_flags(&mut self) {
        self.is_ascii = self.bytes.iter().all(|&b| b < 0x80);
        self.has_tabs = self.bytes.contains(&b'\t');
    }

    /// Verify all chunk invariants. Used by debug assertions in mutation methods.
    #[cfg(debug_assertions)]
    pub(crate) fn check_invariants(&self) -> bool {
        if self.line_starts.is_empty() || self.line_starts[0] != 0 {
            return false;
        }

        for window in self.line_starts.windows(2) {
            if window[0] >= window[1] {
                return false;
            }
        }

        if let Some(&last) = self.line_starts.last() {
            if last as usize > self.bytes.len() {
                return false;
            }
        }

        if self.line_count as usize + 1 != self.line_starts.len() {
            return false;
        }

        let actual = self.bytes.iter().filter(|&&b| b == b'\n').count();
        if actual != self.line_count as usize {
            return false;
        }

        for &ls in &self.line_starts[1..] {
            let pos = ls as usize;
            if pos == 0 || self.bytes[pos - 1] != b'\n' {
                return false;
            }
        }

        if self.is_ascii && self.bytes.iter().any(|&b| b >= 0x80) {
            return false;
        }

        if !self.has_tabs && self.bytes.contains(&b'\t') {
            return false;
        }

        true
    }

    #[cfg(not(debug_assertions))]
    pub(crate) fn check_invariants(&self) -> bool {
        true
    }
}
