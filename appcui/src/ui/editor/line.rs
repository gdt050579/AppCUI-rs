use std::ops::Range;
use super::LineFragment;

/// Suggested maximum fragment size used by line storage.
pub(crate) const LINE_FRAGMENT_THRESHOLD: usize = 4096;



pub(crate) enum LineData {
    Simple(LineFragment),
    List(Vec<LineFragment>),
}

pub(crate) struct Line {
    pub(crate) data: LineData,
    pub(crate) chars: u32,
    pub(crate) is_ascii: bool,
    pub(crate) has_tabs: bool,
    pub(crate) has_multipline_spread: bool, // e.g. a multiline comment
}

impl LineFragment {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        assert!(
            data.len() <= LINE_FRAGMENT_THRESHOLD,
            "LineFragment::new: data too large ({} > {})",
            data.len(),
            LINE_FRAGMENT_THRESHOLD
        );

        let chars = std::str::from_utf8(&data)
            .expect("LineFragment::new: invalid UTF-8")
            .chars()
            .count();

        assert!(
            chars <= u16::MAX as usize,
            "LineFragment::new: char count does not fit in u16 ({chars})"
        );

        let is_ascii = data.iter().all(|&b| b < 0x80);
        let has_tabs = data.contains(&b'\t');
        Self {
            data,
            chars: chars as u16,
            is_ascii,
            has_tabs,
            has_multipline_spread: false,
        }
    }


}

impl Line {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        let fragment = LineFragment::new(data);
        let mut line = Self {
            chars: fragment.chars as u32,
            is_ascii: fragment.is_ascii,
            has_tabs: fragment.has_tabs,
            has_multipline_spread: fragment.has_multipline_spread,
            data: fragment,
        };
        line.refresh_metadata();
        line
    }

    fn offset_to_fragment(&mut self, offset: u32) -> Option<&mut LineFragment> {
        let ofs = offset as usize;
        match self.data {
            LineData::Simple(ref mut fragment) => {
                if ofs < fragment.data.len() {
                    Some(fragment)
                } else {
                    None
                }
            }
            LineData::List(ref mut fragments) => {
                let mut start = 0;
                for fragment in fragments.iter_mut() {
                    let end = start + fragment.data.len();
                    if ofs < end && ofs >= start {
                        return Some(fragment);
                    }
                    start = end;
                }
                None
            },
        }
    }

    pub(crate) fn delete_char(&mut self, byte_index: u32) {
        self.assert_byte_index(byte_index);
        let start = byte_index as usize;
        assert!(
            start < self.data.data.len(),
            "Line::delete_char: byte index {} out of bounds (len {})",
            byte_index,
            self.data.data.len()
        );

        let text =
            std::str::from_utf8(&self.data.data).expect("Line::delete_char: line data is not valid UTF-8");
        let ch = text[start..]
            .chars()
            .next()
            .expect("Line::delete_char: no character at byte index");
        let end = start + ch.len_utf8();
        self.data.data.drain(start..end);
        self.refresh_metadata();
    }

    pub(crate) fn insert_buffer(&mut self, byte_index: u32, buffer: &[u8]) {
        if buffer.is_empty() {
            return;
        }
        std::str::from_utf8(buffer).expect("Line::insert_buffer: buffer is not valid UTF-8");
        self.assert_byte_index(byte_index);
        let byte_index = byte_index as usize;
        self.data
            .data
            .splice(byte_index..byte_index, buffer.iter().copied());
        self.refresh_metadata();
    }

    pub(crate) fn delete_range(&mut self, range: Range<u32>) {
        assert!(range.start <= range.end, "Line::delete_range: invalid range");
        self.assert_byte_index(range.start);
        self.assert_byte_index(range.end);
        if range.start == range.end {
            return;
        }
        let start = range.start as usize;
        let end = range.end as usize;
        self.data.data.drain(start..end);
        self.refresh_metadata();
    }

    fn assert_byte_index(&self, byte_index: u32) {
        assert!(
            (byte_index as usize) <= self.data.data.len(),
            "Line::assert_byte_index: byte index {} out of bounds (len {})",
            byte_index,
            self.data.data.len()
        );
        let text = std::str::from_utf8(&self.data.data)
            .expect("Line::assert_byte_index: line data is not valid UTF-8");
        assert!(
            text.is_char_boundary(byte_index as usize),
            "Line::assert_byte_index: byte index {} is not on UTF-8 boundary",
            byte_index
        );
    }

    fn refresh_metadata(&mut self) {
        assert!(
            self.data.data.len() <= LINE_FRAGMENT_THRESHOLD,
            "Line::refresh_metadata: line fragment exceeded threshold ({} > {})",
            self.data.data.len(),
            LINE_FRAGMENT_THRESHOLD
        );

        let text = std::str::from_utf8(&self.data.data)
            .expect("Line::refresh_metadata: line data is not valid UTF-8");
        let chars = text.chars().count();
        assert!(
            chars <= u16::MAX as usize,
            "Line::refresh_metadata: char count does not fit in u16 ({chars})"
        );

        self.data.chars = chars as u16;
        self.data.is_ascii = self.data.data.iter().all(|&b| b < 0x80);
        self.data.has_tabs = self.data.data.contains(&b'\t');
        self.data.has_multipline_spread = self
            .data
            .data
            .windows(2)
            .any(|w| w == b"/*" || w == b"*/");

        self.chars = self.data.chars as u32;
        self.is_ascii = self.data.is_ascii;
        self.has_tabs = self.data.has_tabs;
        self.has_multipline_spread = self.data.has_multipline_spread;
    }
}
