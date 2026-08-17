use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16)]
pub(crate) enum SpanType {
    Normal = 0x000,
    Bold = 0x001,
    Italic = 0x002,
    Link = 0x004,
    Email = 0x008,
    Code = 0x010,
    CodeBlock = 0x020,
    Bullet = 0x040,
    QuoteMark = 0x080,
    Quote = 0x100,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Span {
    pub start: u32,
    pub end: u32,
    pub x_pos: u32,
    pub y_pos: u32,
    pub span_type: SpanType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MarkerKind {
    Bold,
    Italic,
    Code,
    CodeBlock,
}

#[derive(Clone, Copy, Debug)]
struct Marker {
    start: u32,
    len: u32,
    kind: MarkerKind,
}

pub struct Parser {
    spans: Vec<Span>,
    buffer: Vec<Span>,
    markers: Vec<Marker>,
    styles: Vec<Marker>,
    show_markers: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            buffer: Vec::new(),
            markers: Vec::new(),
            styles: Vec::new(),
            show_markers: false,
        }
    }
    
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    pub fn show_markers(&self) -> bool {
        self.show_markers
    }

    pub fn set_show_markers(&mut self, value: bool) {
        self.show_markers = value;
    }

    pub(crate) fn get_char_len(first_byte: u8) -> usize {
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

    pub(crate) fn get_char_width(len: usize) -> i32 {
        if len == 4 { 2 } else { 1 }
    }

    pub(crate) fn get_char(bytes: &[u8], i: usize, byte_len: usize) -> char {
        std::str::from_utf8(&bytes[i..i + byte_len])
            .ok()
            .and_then(|s| s.chars().next())
            .unwrap_or('\u{FFFD}')
    }

    fn marker_to_flag(kind: MarkerKind) -> SpanType {
        match kind {
            MarkerKind::Bold => SpanType::Bold,
            MarkerKind::Italic => SpanType::Italic,
            MarkerKind::Code => SpanType::Code,
            MarkerKind::CodeBlock => SpanType::CodeBlock,
        }
    }

    fn toggle_span(var: &mut SpanType, mask: SpanType) {
        if var.contains(mask) {
            var.remove(mask);
        } else {
            var.set(mask);
        }
    }

    fn push_span(
        span_vec: &mut Vec<Span>,
        start: usize,
        end: usize,
        x_pos: u32,
        y_pos: u32,
        span_type: SpanType,
    ) {
        if start < end {
            span_vec.push(Span {
                start: start as u32,
                end: end as u32,
                x_pos,
                y_pos,
                span_type,
            });
        }
    }

    fn find_close_backtick(bytes: &[u8], from: usize, len: usize) -> Option<usize> {
        let mut i = from;
        while i + len <= bytes.len() {
            if bytes[i..i + len].iter().all(|b| *b == b'`') {
                return Some(i);
            }
            i += Self::get_char_len(bytes[i]);
        }
        None
    }

    fn find_code_markers(&mut self, bytes: &[u8]) {
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] != b'`' {
                i += Self::get_char_len(bytes[i]);
                continue;
            }

            let (kind, len) = if bytes[i..].starts_with(b"```") {
                (MarkerKind::CodeBlock, 3usize)
            } else {
                (MarkerKind::Code, 1usize)
            };

            let Some(close) = Self::find_close_backtick(&bytes, i + len, len) else {
                i += len;
                continue;
            };

            if close == i + len {
                i += len;
                continue;
            }
            
            self.markers.push(Marker {
                start: i as u32,
                len: len as u32,
                kind,
            });
            
            self.markers.push(Marker {
                start: close as u32,
                len: len as u32,
                kind,
            });
            i = close + len;
        }
    }

    fn find_style_markers(&mut self, bytes: &[u8]) {
        let mut i = 0usize;
        let mut markers_index = 0usize;
        let mut last_bold_index = 0usize;
        let mut last_italic_index = 0usize;
        let mut num_bold = 0usize;
        let mut num_italic = 0usize;
        let mut index = 0usize;
        while i < bytes.len() {
            if markers_index < self.markers.len() && self.markers[markers_index].start as usize == i
            {
                let close = self.markers[markers_index + 1];
                i = (close.start + close.len) as usize;
                markers_index += 2;
                continue;
            }
            if bytes[i] == b'*' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                self.styles.push(Marker {
                    start: i as u32,
                    len: 2,
                    kind: MarkerKind::Bold,
                });
                i += 2;
                last_bold_index = index;
                num_bold += 1;
                index += 1;
            } else if bytes[i] == b'_' {
                self.styles.push(Marker {
                    start: i as u32,
                    len: 1,
                    kind: MarkerKind::Italic,
                });
                i += 1;
                last_italic_index = index;
                num_italic += 1;
                index += 1;
            } else {
                i += Self::get_char_len(bytes[i]);
            }
        }

        let mut first = if num_bold % 2 == 1 {
            last_bold_index
        } else {
            usize::MAX
        };

        let mut second = if num_italic % 2 == 1 {
            last_italic_index
        } else {
            usize::MAX
        };

        if first < second {
            std::mem::swap(&mut first, &mut second);
        }

        if first != usize::MAX {
            self.styles.remove(first);
        }

        if second != usize::MAX {
            self.styles.remove(second);
        }
    }

    fn match_markers(&mut self, bytes: &[u8]) {
        self.markers.clear();
        self.styles.clear();

        self.find_code_markers(bytes);
        self.find_style_markers(bytes);

        self.markers.extend_from_slice(&self.styles);
        self.markers.sort_unstable_by_key(|marker| marker.start);
    }

    pub fn parse_spans(&mut self, text: &String) {
        let bytes = text.as_bytes();

        self.match_markers(bytes);
        self.spans.clear();

        let mut style = SpanType::Normal;
        let mut span_start = 0usize;
        let mut i = 0usize;
        let mut next_marker = 0usize;

        while i < bytes.len() {
            let marker = if next_marker < self.markers.len() {
                self.markers[next_marker]
            } else {
                Marker {
                    start: u32::MAX,
                    len: 0,
                    kind: MarkerKind::Bold,
                }
            };

            if marker.start as usize == i {
                Self::push_span(&mut self.spans, span_start, i, 0, 0, style);

                let end = i + marker.len as usize;
                let flag = Self::marker_to_flag(marker.kind);
                let is_open = !style.contains(flag);

                if is_open {
                    if self.show_markers {
                        Self::push_span(&mut self.spans, i, end, 0, 0, style);
                    }
                    Self::toggle_span(&mut style, flag);
                } else {
                    Self::toggle_span(&mut style, flag);
                    if self.show_markers {
                        Self::push_span(&mut self.spans, i, end, 0, 0, style);
                    }
                }

                next_marker += 1;
                i = end;
                span_start = i;
            } else {
                i += Self::get_char_len(bytes[i]);
            }
        }

        Self::push_span(&mut self.spans, span_start, bytes.len(), 0, 0, style);
    }

    fn word_starts_with(bytes: &[u8], start: usize, end: usize, prefix: &[u8]) -> bool {
        end - start >= prefix.len()
            && bytes[start..start + prefix.len()].eq_ignore_ascii_case(prefix)
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

            if base_type.contains_one(SpanType::Code | SpanType::CodeBlock) {
                self.buffer.push(*span);
                continue;
            }

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
                    word_end = (word_end + Self::get_char_len(bytes[word_end])).min(span_end);
                }

                let word_type = Self::word_type(bytes, word_start, word_end);
                if !word_type.is_empty() {
                    Self::push_span(&mut self.buffer, seg_start, word_start, 0, 0, base_type);
                    Self::push_span(
                        &mut self.buffer,
                        word_start,
                        word_end,
                        0,
                        0,
                        base_type | word_type,
                    );
                    seg_start = word_end;
                }

                i = word_end;
            }
            Self::push_span(&mut self.buffer, seg_start, span_end, 0, 0, base_type);
        }
        std::mem::swap(&mut self.spans, &mut self.buffer);
    }

    fn line_start(bytes: &[u8], i: usize) -> bool {
        let mut j = i;
        while j > 0 && bytes[j - 1] == b' ' {
            j -= 1;
        }
        if j == 0 || bytes[j - 1] == b'\n' {
            return true;
        }
        if bytes[j - 1] == b'>' {
            let k = j - 1;
            return k == 0 || bytes[k - 1] == b'\n';
        }
        false
    }

    fn is_bullet_at(bytes: &[u8], i: usize) -> bool {
        if bytes[i] != b'*' && bytes[i] != b'-' {
            return false;
        }

        if i + 1 >= bytes.len() || bytes[i + 1] != b' ' {
            return false;
        }
        Self::line_start(bytes, i)
    }

    fn is_quote_at(bytes: &[u8], i: usize) -> bool {
        if bytes[i] != b'>' {
            return false;
        }

        if i + 1 >= bytes.len() || bytes[i + 1] != b' ' {
            return false;
        }
        i == 0 || bytes[i - 1] == b'\n'
    }

    pub fn quoted_line(bytes: &[u8], from: usize) -> bool {
        from < bytes.len() && Self::is_quote_at(bytes, from)
    }

    pub fn list_indent(bytes: &[u8], from: usize) -> u32 {
        if Self::quoted_line(bytes, from) {
            return 2 + Self::list_indent(bytes, from + 2);
        }

        let mut i = from;
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }
        let spaces = (i - from) as u32;

        if i < bytes.len() && (bytes[i] == b'*' || bytes[i] == b'-') {
            if i + 1 < bytes.len() && bytes[i + 1] == b' ' {
                return spaces + 2;
            }
            return 0;
        }

        let digits = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i > digits && i + 1 < bytes.len() && bytes[i] == b'.' && bytes[i + 1] == b' ' {
            return spaces + (i - digits) as u32 + 2;
        }
        0
    }

    pub fn parse_lists(&mut self, text: &String) {
        if self.show_markers {
            return;
        }

        self.buffer.clear();
        let bytes = text.as_bytes();

        for span in &self.spans {
            let span_start = span.start as usize;
            let span_end = span.end as usize;

            let mut seg_start = span_start;
            let mut i = span_start;

            while i < span_end {
                let span_type = if Self::is_bullet_at(bytes, i) {
                    SpanType::Bullet
                } else if Self::is_quote_at(bytes, i) {
                    SpanType::QuoteMark
                } else {
                    SpanType::Normal
                };

                if !span_type.is_empty() {
                    Self::push_span(&mut self.buffer, seg_start, i, 0, 0, span.span_type);
                    Self::push_span(&mut self.buffer, i, i + 1, 0, 0, span.span_type | span_type);
                    seg_start = i + 1;
                    i += 1;
                    continue;
                }
                i += Self::get_char_len(bytes[i]);
            }
            Self::push_span(&mut self.buffer, seg_start, span_end, 0, 0, span.span_type);
        }

        std::mem::swap(&mut self.spans, &mut self.buffer);
    }

    fn is_word_start(bytes: &[u8], span_start: usize, i: usize) -> bool {
        i == span_start || bytes[i - 1].is_ascii_whitespace()
    }

    fn quoted_type(span_type: SpanType, quote: bool) -> SpanType {
        if quote {
            span_type | SpanType::Quote
        } else {
            span_type
        }
    }

    fn word_width(bytes: &[u8], i: usize, span_end: usize) -> u32 {
        let mut width = 0u32;
        let mut j = i;
        while j < span_end && !bytes[j].is_ascii_whitespace() {
            let len = Self::get_char_len(bytes[j]);
            width += Self::get_char_width(len) as u32;
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
        let mut indent = Self::list_indent(bytes, 0).min(line_width - 1);
        let mut quote = !self.show_markers && Self::quoted_line(bytes, 0);

        for span in &self.spans {
            let span_start = span.start as usize;
            let span_end = span.end as usize;

            let mut seg_start = span_start;
            let mut seg_x = x;
            let mut i = span_start;

            while i < span_end {
                let len = Self::get_char_len(bytes[i]);
                let w = Self::get_char_width(len) as u32;
                let ch = Self::get_char(bytes, i, len);
                let style = Self::quoted_type(span.span_type, quote);

                if ch == '\n' {
                    Self::push_span(&mut self.buffer, seg_start, i, seg_x, y, style);
                    y += 1;
                    x = 0;
                    i += len;
                    seg_start = i;
                    seg_x = 0;
                    indent = Self::list_indent(bytes, i).min(line_width - 1);
                    quote = !self.show_markers && Self::quoted_line(bytes, i);
                    continue;
                }

                if !ch.is_whitespace()
                    && Self::is_word_start(bytes, span_start, i)
                    && x > indent
                    && x + Self::word_width(bytes, i, span_end) > line_width
                {
                    Self::push_span(&mut self.buffer, seg_start, i, seg_x, y, style);
                    y += 1;
                    x = indent;
                    seg_start = i;
                    seg_x = indent;
                }

                if x > indent && x + w > line_width {
                    if ch.is_whitespace() {
                        Self::push_span(&mut self.buffer, seg_start, i + len, seg_x, y, style);
                        y += 1;
                        x = indent;
                        i += len;
                        seg_start = i;
                        seg_x = indent;
                        continue;
                    }

                    Self::push_span(&mut self.buffer, seg_start, i, seg_x, y, style);
                    y += 1;
                    x = indent;
                    seg_start = i;
                    seg_x = indent;
                }
                x += w;
                i += len;
            }
            Self::push_span(
                &mut self.buffer,
                seg_start,
                span_end,
                seg_x,
                y,
                Self::quoted_type(span.span_type, quote),
            );
        }

        std::mem::swap(&mut self.spans, &mut self.buffer);
    }

    pub fn parse(&mut self, text: &String, line_width: u32) -> &[Span] {
        self.parse_spans(text);
        self.parse_links(text);
        self.parse_lists(text);
        self.parse_lines(text, line_width);
        &self.spans
    }

    fn width_between(bytes: &[u8], from: usize, to: usize) -> u32 {
        let mut width = 0u32;
        let mut i = from;
        while i < to {
            let len = Self::get_char_len(bytes[i]);
            width += Self::get_char_width(len) as u32;
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
        (i + Self::get_char_len(bytes[i])).min(bytes.len()) as u32
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
        if self.show_markers {
            return Self::next_offset(text, offset);
        }
        let end = text.len() as u32;
        let offset = self.skip_markers_forward(offset.min(end));
        if offset >= end {
            return end;
        }
        self.skip_markers_forward(Self::next_offset(text, offset))
    }

    pub fn prev_visible_offset(&self, text: &String, offset: u32) -> u32 {
        if self.show_markers {
            return Self::prev_offset(text, offset);
        }
        let offset = self.skip_markers_backward(offset.min(text.len() as u32));
        if offset == 0 {
            return 0;
        }
        self.skip_markers_backward(Self::prev_offset(text, offset))
    }

    pub fn get_position_from_offset(&self, text: &String, offset: u32) -> (u32, u32) {
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
            return (
                span.x_pos + Self::width_between(bytes, span_start, offset),
                span.y_pos,
            );
        }

        let newlines = Self::count_newlines(bytes, span_end, offset);
        if newlines == 0 {
            (
                span.x_pos + Self::width_between(bytes, span_start, span_end),
                span.y_pos,
            )
        } else {
            (0, span.y_pos + newlines)
        }
    }

    pub fn rows(&self, text: &String) -> u32 {
        self.get_position_from_offset(text, text.len() as u32).1 + 1
    }

    pub fn get_offset_from_position(&self, text: &String, x: u32, y: u32) -> u32 {
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
                let len = Self::get_char_len(bytes[i]);
                let w = Self::get_char_width(len) as u32;
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