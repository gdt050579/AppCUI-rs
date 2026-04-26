use super::{LineFragment, LineChunkSplitter};
use std::ops::Range;


pub(crate) enum LineData {
    Simple(LineFragment),
    List(Vec<LineFragment>),
}

pub(crate) struct Line {
    pub(crate) data: LineData,
    pub(crate) chars: u32,
    pub(crate) is_ascii: bool,
    pub(crate) has_tabs: bool,
}

impl Line {
    pub(super) fn new(text: &str) -> Self {
        let ld = if text.len() <= LineChunkSplitter::MAX_CHUNK_SIZE {
            LineData::Simple(LineFragment::new(text))
        } else {
            let mut fragments = Vec::with_capacity(LineChunkSplitter::approximate_fragment_count(text));
            let mut splitter = LineChunkSplitter::new(text);
            while let Some(chunk) = splitter.next() {
                fragments.push(LineFragment::new(chunk));
            }
            LineData::List(fragments.into_iter().collect())
        };
        Self {
            data: ld,
            chars: text.chars().count() as u32,
            is_ascii: text.is_ascii(),
            has_tabs: text.contains('\t'),
        }
    }

    fn offset_to_fragment(&mut self, offset: u32) -> Option<(&mut LineFragment, u32)> {
        let ofs = offset as usize;
        match self.data {
            LineData::Simple(ref mut fragment) => {
                if ofs < fragment.data.len() {
                    Some((fragment, 0))
                } else {
                    None
                }
            }
            LineData::List(ref mut fragments) => {
                let mut start = 0;
                for fragment in fragments.iter_mut() {
                    let end = start + fragment.data.len();
                    if ofs < end && ofs >= start {
                        return Some((fragment, start as u32));
                    }
                    start = end;
                }
                None
            }
        }
    }

    pub(crate) fn delete_char(&mut self, byte_index: u32) {
        // if let Some((fragment, start)) = self.offset_to_fragment(byte_index) {
        //     fragment.delete_char(byte_index - start);
        //     self.refresh_metadata();
        // }
    }

    pub(crate) fn insert_buffer(&mut self, byte_index: u32, buffer: &[u8]) {
        // if buffer.is_empty() {
        //     return;
        // }
        // if let Some((fragment, start)) = self.offset_to_fragment(byte_index) {
        //     fragment.insert_buffer(byte_index - start, buffer);
        //     self.refresh_metadata();
        // }
    }

    pub(crate) fn delete_range(&mut self, range: Range<u32>) {
        // if let Some((fragment, start)) = self.offset_to_fragment(range.start) {
        //     let st = range.start - start;
        //     let end = range.end - start;
        //     fragment.delete_range(st..end);
        //     self.refresh_metadata();
        // }
    }

    fn refresh_metadata(&mut self) {
        // assert!(
        //     self.data.data.len() <= LINE_FRAGMENT_THRESHOLD,
        //     "Line::refresh_metadata: line fragment exceeded threshold ({} > {})",
        //     self.data.data.len(),
        //     LINE_FRAGMENT_THRESHOLD
        // );

        // let text = std::str::from_utf8(&self.data.data).expect("Line::refresh_metadata: line data is not valid UTF-8");
        // let chars = text.chars().count();
        // assert!(
        //     chars <= u16::MAX as usize,
        //     "Line::refresh_metadata: char count does not fit in u16 ({chars})"
        // );

        // self.data.chars = chars as u16;
        // self.data.is_ascii = self.data.data.iter().all(|&b| b < 0x80);
        // self.data.has_tabs = self.data.data.contains(&b'\t');
        // self.data.has_multipline_spread = self.data.data.windows(2).any(|w| w == b"/*" || w == b"*/");

        // self.chars = self.data.chars as u32;
        // self.is_ascii = self.data.is_ascii;
        // self.has_tabs = self.data.has_tabs;
        // self.has_multipline_spread = self.data.has_multipline_spread;
    }
}
