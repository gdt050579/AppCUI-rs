use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum SpanType {
    Normal = 0x00,
    Bold = 0x01,
    Italic = 0x02,
    Link = 0x04,
    Email = 0x08,
}

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: u32,
    pub end: u32,
    pub x_pos: u32,
    pub y_pos: u32,
    pub span_type: SpanType,
}

#[derive(Clone, Copy, Debug)]
struct Marker {
    start: u32,
    len: u32,
}

pub struct Parser {
    spans: Vec<Span>,
    buffer: Vec<Span>,
    markers: Vec<Marker>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            buffer: Vec::new(),
            markers: Vec::new(),
        }
    }

    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    pub(crate) fn char_len_from_first_byte(first_byte: u8) -> usize {
        if first_byte < 0x80 {
            1
        } else if first_byte >> 5 == 0b110 {
            2
        } else if first_byte >> 4 == 0b1110 {
            3
        } else if first_byte >> 3 == 0b11110 {
            4
        } else {
            1
        }
    }

    pub(crate) fn char_width_from_byte_len(byte_len: usize) -> i32 {
        if byte_len == 4 {
            2
        } else {
            1
        }
    }

    pub(crate) fn char_at(bytes: &[u8], i: usize, byte_len: usize) -> char {
        std::str::from_utf8(&bytes[i..i + byte_len])
            .ok()
            .and_then(|s| s.chars().next())
            .unwrap_or('\u{FFFD}')
    }

    fn style(bold: bool, italic: bool) -> SpanType {
        let mut span_type = SpanType::Normal;
        if bold {
            span_type |= SpanType::Bold;
        }
        if italic {
            span_type |= SpanType::Italic;
        }
        span_type
    }

    fn push_span(out: &mut Vec<Span>, start: usize, end: usize, x_pos: u32, y_pos: u32, span_type: SpanType) {
        if start < end {
            out.push(Span {
                start: start as u32,
                end: end as u32,
                x_pos,
                y_pos,
                span_type,
            });
        }
    }

    fn match_markers(&mut self, bytes: &[u8]) {
        self.markers.clear();

        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] == b'*' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                self.markers.push(Marker { start: i as u32, len: 2 });
                i += 2;
            } else if bytes[i] == b'_' {
                self.markers.push(Marker { start: i as u32, len: 1 });
                i += 1;
            } else {
                i += Self::char_len_from_first_byte(bytes[i]);
            }
        }

        let mut bold_count = 0usize;
        let mut italic_count = 0usize;
        let mut last_bold = usize::MAX;
        let mut last_italic = usize::MAX;

        for (index, marker) in self.markers.iter().enumerate() {
            if marker.len == 2 {
                bold_count += 1;
                last_bold = index;
            } else {
                italic_count += 1;
                last_italic = index;
            }
        }

        let mut first = if bold_count % 2 == 1 { last_bold } else { usize::MAX };
        let mut second = if italic_count % 2 == 1 { last_italic } else { usize::MAX };

        if first < second {
            std::mem::swap(&mut first, &mut second);
        }
        if first != usize::MAX {
            self.markers.remove(first);
        }
        if second != usize::MAX {
            self.markers.remove(second);
        }
    }

    pub fn parse_spans(&mut self, text: &String) {
        let bytes = text.as_bytes();

        self.match_markers(bytes);
        self.spans.clear();

        let mut bold = false;
        let mut italic = false;
        let mut span_start = 0usize;
        let mut i = 0usize;
        let mut next_marker = 0usize;

        while i < bytes.len() {
            let marker = if next_marker < self.markers.len() {
                self.markers[next_marker]
            } else {
                Marker { start: u32::MAX, len: 0 }
            };

            if marker.start as usize == i {
                Self::push_span(&mut self.spans, span_start, i, 0, 0, Self::style(bold, italic));

                if marker.len == 2 {
                    bold = !bold;
                } else {
                    italic = !italic;
                }

                next_marker += 1;
                i += marker.len as usize;
                span_start = i;
            } else {
                i += Self::char_len_from_first_byte(bytes[i]);
            }
        }

        Self::push_span(&mut self.spans, span_start, bytes.len(), 0, 0, Self::style(bold, italic));
    }

    fn word_starts_with(bytes: &[u8], start: usize, end: usize, prefix: &[u8]) -> bool {
        end - start >= prefix.len() && bytes[start..start + prefix.len()].eq_ignore_ascii_case(prefix)
    }

    fn word_type(bytes: &[u8], start: usize, end: usize) -> SpanType {
        if Self::word_starts_with(bytes, start, end, b"http://")
            || Self::word_starts_with(bytes, start, end, b"https://")
            || Self::word_starts_with(bytes, start, end, b"www.")
        {
            return SpanType::Link;
        }

        if let Some(at) = bytes[start..end].iter().position(|&b| b == b'@') {
            let at = start + at;
            if at > start && bytes[at + 1..end].contains(&b'.') {
                return SpanType::Email;
            }
        }

        SpanType::Normal
    }

    pub fn parse_links(&mut self, text: &String) {
        self.buffer.clear();

        let bytes = text.as_bytes();

        for span in &self.spans {
            let base_type = span.span_type;
            let span_start = span.start as usize;
            let span_end = span.end as usize;

            let mut seg_start = span_start;
            let mut i = span_start;

            while i < span_end {
                if bytes[i].is_ascii_whitespace() {
                    i += 1;
                    continue;
                }

                let word_start = i;
                let mut word_end = i;
                while word_end < span_end && !bytes[word_end].is_ascii_whitespace() {
                    word_end = (word_end + Self::char_len_from_first_byte(bytes[word_end])).min(span_end);
                }

                let word_type = Self::word_type(bytes, word_start, word_end);
                if !word_type.is_empty() {
                    Self::push_span(&mut self.buffer, seg_start, word_start, 0, 0, base_type);
                    Self::push_span(&mut self.buffer, word_start, word_end, 0, 0, base_type | word_type);
                    seg_start = word_end;
                }

                i = word_end;
            }

            Self::push_span(&mut self.buffer, seg_start, span_end, 0, 0, base_type);
        }

        std::mem::swap(&mut self.spans, &mut self.buffer);
    }

    fn is_word_start(bytes: &[u8], span_start: usize, i: usize) -> bool {
        i == span_start || bytes[i - 1].is_ascii_whitespace()
    }

    fn word_width(bytes: &[u8], i: usize, span_end: usize) -> u32 {
        let mut width = 0u32;
        let mut j = i;
        while j < span_end && !bytes[j].is_ascii_whitespace() {
            let len = Self::char_len_from_first_byte(bytes[j]);
            width += Self::char_width_from_byte_len(len) as u32;
            j += len;
        }
        width
    }

    pub fn parse_lines(&mut self, text: &String, line_width: u32) {
        self.buffer.clear();

        let bytes = text.as_bytes();
        let line_width = line_width.max(1);

        let mut x: u32 = 0;
        let mut y: u32 = 0;

        for span in &self.spans {
            let span_start = span.start as usize;
            let span_end = span.end as usize;

            let mut seg_start = span_start;
            let mut seg_x = x;
            let mut i = span_start;

            while i < span_end {
                let len = Self::char_len_from_first_byte(bytes[i]);
                let w = Self::char_width_from_byte_len(len) as u32;
                let ch = Self::char_at(bytes, i, len);

                if ch == '\n' {
                    Self::push_span(&mut self.buffer, seg_start, i, seg_x, y, span.span_type);
                    y += 1;
                    x = 0;
                    i += len;
                    seg_start = i;
                    seg_x = 0;
                    continue;
                }

                if !ch.is_whitespace() && Self::is_word_start(bytes, span_start, i) && x > 0 && x + Self::word_width(bytes, i, span_end) > line_width
                {
                    Self::push_span(&mut self.buffer, seg_start, i, seg_x, y, span.span_type);
                    y += 1;
                    x = 0;
                    seg_start = i;
                    seg_x = 0;
                }

                if x > 0 && x + w > line_width {
                    Self::push_span(&mut self.buffer, seg_start, i, seg_x, y, span.span_type);
                    y += 1;
                    x = 0;
                    seg_start = i;
                    seg_x = 0;
                }

                x += w;
                i += len;
            }

            Self::push_span(&mut self.buffer, seg_start, span_end, seg_x, y, span.span_type);
        }

        std::mem::swap(&mut self.spans, &mut self.buffer);
    }

    pub fn parse(&mut self, text: &String, line_width: u32) -> &[Span] {
        self.parse_spans(text);
        self.parse_links(text);
        self.parse_lines(text, line_width);
        &self.spans
    }

    fn width_between(bytes: &[u8], from: usize, to: usize) -> u32 {
        let mut width = 0u32;
        let mut i = from;
        while i < to {
            let len = Self::char_len_from_first_byte(bytes[i]);
            width += Self::char_width_from_byte_len(len) as u32;
            i += len;
        }
        width
    }

    fn count_newlines(bytes: &[u8], from: usize, to: usize) -> u32 {
        let mut count = 0u32;
        let mut i = from;
        while i < to {
            if bytes[i] == b'\n' {
                count += 1;
            }
            i += 1;
        }
        count
    }

    pub fn next_offset(text: &String, offset: u32) -> u32 {
        let bytes = text.as_bytes();
        let i = (offset as usize).min(bytes.len());
        if i >= bytes.len() {
            return bytes.len() as u32;
        }
        (i + Self::char_len_from_first_byte(bytes[i])).min(bytes.len()) as u32
    }

    pub fn prev_offset(text: &String, offset: u32) -> u32 {
        let bytes = text.as_bytes();
        let mut i = (offset as usize).min(bytes.len());
        if i == 0 {
            return 0;
        }
        i -= 1;
        while i > 0 && bytes[i] & 0xC0 == 0x80 {
            i -= 1;
        }
        i as u32
    }

    fn skip_markers_forward(&self, offset: u32) -> u32 {
        let mut offset = offset;
        for marker in &self.markers {
            if marker.start <= offset && offset < marker.start + marker.len {
                offset = marker.start + marker.len;
            }
        }
        offset
    }

    fn skip_markers_backward(&self, offset: u32) -> u32 {
        let mut offset = offset;
        for marker in self.markers.iter().rev() {
            if marker.start < offset && offset <= marker.start + marker.len {
                offset = marker.start;
            }
        }
        offset
    }

    pub fn next_visible_offset(&self, text: &String, offset: u32) -> u32 {
        let end = text.len() as u32;
        let offset = self.skip_markers_forward(offset.min(end));
        if offset >= end {
            return end;
        }
        self.skip_markers_forward(Self::next_offset(text, offset))
    }

    pub fn prev_visible_offset(&self, text: &String, offset: u32) -> u32 {
        let offset = self.skip_markers_backward(offset.min(text.len() as u32));
        if offset == 0 {
            return 0;
        }
        self.skip_markers_backward(Self::prev_offset(text, offset))
    }

    pub fn position(&self, text: &String, offset: u32) -> (u32, u32) {
        let bytes = text.as_bytes();
        let offset = (offset as usize).min(bytes.len());

        let mut found: Option<&Span> = None;
        for span in &self.spans {
            if span.start as usize > offset {
                break;
            }
            found = Some(span);
        }

        let Some(span) = found else {
            return (0, Self::count_newlines(bytes, 0, offset));
        };

        let span_start = span.start as usize;
        let span_end = span.end as usize;

        if offset <= span_end {
            return (span.x_pos + Self::width_between(bytes, span_start, offset), span.y_pos);
        }

        let newlines = Self::count_newlines(bytes, span_end, offset);
        if newlines == 0 {
            (span.x_pos + Self::width_between(bytes, span_start, span_end), span.y_pos)
        } else {
            (0, span.y_pos + newlines)
        }
    }

    pub fn offset_at(&self, text: &String, x: u32, y: u32) -> u32 {
        let bytes = text.as_bytes();
        let mut row_last: Option<&Span> = None;

        for span in &self.spans {
            if span.y_pos != y {
                continue;
            }

            if x < span.x_pos {
                return span.start;
            }

            let span_end = span.end as usize;
            let mut col = span.x_pos;
            let mut i = span.start as usize;

            while i < span_end {
                let len = Self::char_len_from_first_byte(bytes[i]);
                let w = Self::char_width_from_byte_len(len) as u32;
                if x < col + w {
                    return i as u32;
                }
                col += w;
                i += len;
            }

            row_last = Some(span);
        }

        if let Some(span) = row_last {
            return span.end;
        }

        let mut base = 0usize;
        let mut base_row = 0u32;
        for span in &self.spans {
            if span.y_pos >= y {
                break;
            }
            base = span.end as usize;
            base_row = span.y_pos;
        }

        let mut i = base;
        let mut remaining = y - base_row;
        while i < bytes.len() && remaining > 0 {
            if bytes[i] == b'\n' {
                remaining -= 1;
            }
            i += 1;
        }
        i as u32
    }
}