use EnumBitFlags::EnumBitFlags;

const CODE_MARGIN: u32 = 1;

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


    fn is_wide_char(ch: char) -> bool {
        matches!(ch as u32, 0x1100..=0x115F | 0x231A..=0x231B | 0x2329 | 0x232A | 0x23E9..=0x23EC | 0x23F0 | 0x23F3 | 0x25FD..=0x25FE | 0x2614..=0x2615 | 0x2630..=0x2637 | 0x2648..=0x2653 | 0x267F | 0x268A..=0x268F | 0x2693 | 0x26A1 | 0x26AA..=0x26AB | 0x26BD..=0x26BE | 0x26C4..=0x26C5 | 0x26CE | 0x26D4 | 0x26EA | 0x26F2..=0x26F3 | 0x26F5 | 0x26FA | 0x26FD | 0x2705 | 0x270A..=0x270B | 0x2728 | 0x274C | 0x274E | 0x2753..=0x2755 | 0x2757 | 0x2795..=0x2797 | 0x27B0 | 0x27BF | 0x2B1B..=0x2B1C | 0x2B50 | 0x2B55 | 0x2E80..=0x2E99 | 0x2E9B..=0x2EF3 | 0x2F00..=0x2FD5 | 0x2FF0..=0x2FFF | 0x3000 | 0x3001..=0x3003 | 0x3004 | 0x3005 | 0x3006 | 0x3007 | 0x3008 | 0x3009 | 0x300A | 0x300B | 0x300C | 0x300D | 0x300E | 0x300F | 0x3010 | 0x3011 | 0x3012..=0x3013 | 0x3014 | 0x3015 | 0x3016 | 0x3017 | 0x3018 | 0x3019 | 0x301A | 0x301B | 0x301C | 0x301D | 0x301E..=0x301F | 0x3020 | 0x3021..=0x3029 | 0x302A..=0x302D | 0x302E..=0x302F | 0x3030 | 0x3031..=0x3035 | 0x3036..=0x3037 | 0x3038..=0x303A | 0x303B | 0x303C | 0x303D | 0x303E | 0x3041..=0x3096 | 0x3099..=0x309A | 0x309B..=0x309C | 0x309D..=0x309E | 0x309F | 0x30A0 | 0x30A1..=0x30FA | 0x30FB | 0x30FC..=0x30FE | 0x30FF | 0x3105..=0x312F | 0x3131..=0x318E | 0x3190..=0x3191 | 0x3192..=0x3195 | 0x3196..=0x319F | 0x31A0..=0x31BF | 0x31C0..=0x31E5 | 0x31EF | 0x31F0..=0x31FF | 0x3200..=0x321E | 0x3220..=0x3229 | 0x322A..=0x3247 | 0x3250 | 0x3251..=0x325F | 0x3260..=0x327F | 0x3280..=0x3289 | 0x328A..=0x32B0 | 0x32B1..=0x32BF | 0x32C0..=0x32FF | 0x3300..=0x33FF | 0x3400..=0x4DBF | 0x4DC0..=0x4DFF | 0x4E00..=0x9FFF | 0xA000..=0xA014 | 0xA015 | 0xA016..=0xA48C | 0xA490..=0xA4C6 | 0xA960..=0xA97C | 0xAC00..=0xD7A3 | 0xF900..=0xFA6D | 0xFA6E..=0xFA6F | 0xFA70..=0xFAD9 | 0xFADA..=0xFAFF | 0xFE10..=0xFE16 | 0xFE17 | 0xFE18 | 0xFE19 | 0xFE30 | 0xFE31..=0xFE32 | 0xFE33..=0xFE34 | 0xFE35 | 0xFE36 | 0xFE37 | 0xFE38 | 0xFE39 | 0xFE3A | 0xFE3B | 0xFE3C | 0xFE3D | 0xFE3E | 0xFE3F | 0xFE40 | 0xFE41 | 0xFE42 | 0xFE43 | 0xFE44 | 0xFE45..=0xFE46 | 0xFE47 | 0xFE48 | 0xFE49..=0xFE4C | 0xFE4D..=0xFE4F | 0xFE50..=0xFE52 | 0xFE54..=0xFE57 | 0xFE58 | 0xFE59 | 0xFE5A | 0xFE5B | 0xFE5C | 0xFE5D | 0xFE5E | 0xFE5F..=0xFE61 | 0xFE62 | 0xFE63 | 0xFE64..=0xFE66 | 0xFE68 | 0xFE69 | 0xFE6A..=0xFE6B | 0xFF01..=0xFF03 | 0xFF04 | 0xFF05..=0xFF07 | 0xFF08 | 0xFF09 | 0xFF0A | 0xFF0B | 0xFF0C | 0xFF0D | 0xFF0E..=0xFF0F | 0xFF10..=0xFF19 | 0xFF1A..=0xFF1B | 0xFF1C..=0xFF1E | 0xFF1F..=0xFF20 | 0xFF21..=0xFF3A | 0xFF3B | 0xFF3C | 0xFF3D | 0xFF3E | 0xFF3F | 0xFF40 | 0xFF41..=0xFF5A | 0xFF5B | 0xFF5C | 0xFF5D | 0xFF5E | 0xFF5F | 0xFF60 | 0xFFE0..=0xFFE1 | 0xFFE2 | 0xFFE3 | 0xFFE4 | 0xFFE5..=0xFFE6 | 0x16FE0..=0x16FE1 | 0x16FE2 | 0x16FE3 | 0x16FE4 | 0x16FF0..=0x16FF1 | 0x16FF2..=0x16FF3 | 0x16FF4..=0x16FF6 | 0x17000..=0x187FF | 0x18800..=0x18AFF | 0x18B00..=0x18CD5 | 0x18CFF | 0x18D00..=0x18D1E | 0x18D80..=0x18DF2 | 0x1AFF0..=0x1AFF3 | 0x1AFF5..=0x1AFFB | 0x1AFFD..=0x1AFFE | 0x1B000..=0x1B0FF | 0x1B100..=0x1B122 | 0x1B132 | 0x1B150..=0x1B152 | 0x1B155 | 0x1B164..=0x1B167 | 0x1B170..=0x1B2FB | 0x1D300..=0x1D356 | 0x1D360..=0x1D376 | 0x1F004 | 0x1F0CF | 0x1F18E | 0x1F191..=0x1F19A | 0x1F200..=0x1F202 | 0x1F210..=0x1F23B | 0x1F240..=0x1F248 | 0x1F250..=0x1F251 | 0x1F260..=0x1F265 | 0x1F300..=0x1F320 | 0x1F32D..=0x1F335 | 0x1F337..=0x1F37C | 0x1F37E..=0x1F393 | 0x1F3A0..=0x1F3CA | 0x1F3CF..=0x1F3D3 | 0x1F3E0..=0x1F3F0 | 0x1F3F4 | 0x1F3F8..=0x1F3FA | 0x1F3FB..=0x1F3FF | 0x1F400..=0x1F43E | 0x1F440 | 0x1F442..=0x1F4FC | 0x1F4FF..=0x1F53D | 0x1F54B..=0x1F54E | 0x1F550..=0x1F567 | 0x1F57A | 0x1F595..=0x1F596 | 0x1F5A4 | 0x1F5FB..=0x1F5FF | 0x1F600..=0x1F64F | 0x1F680..=0x1F6C5 | 0x1F6CC | 0x1F6D0..=0x1F6D2 | 0x1F6D5..=0x1F6D8 | 0x1F6DC..=0x1F6DF | 0x1F6EB..=0x1F6EC | 0x1F6F4..=0x1F6FC | 0x1F7E0..=0x1F7EB | 0x1F7F0 | 0x1F90C..=0x1F93A | 0x1F93C..=0x1F945 | 0x1F947..=0x1F9FF | 0x1FA70..=0x1FA7C | 0x1FA80..=0x1FA8A | 0x1FA8E..=0x1FAC6 | 0x1FAC8 | 0x1FACD..=0x1FADC | 0x1FADF..=0x1FAEA | 0x1FAEF..=0x1FAF8 | 0x20000..=0x2A6DF | 0x2A6E0..=0x2A6FF | 0x2A700..=0x2B81D | 0x2B81E..=0x2B81F | 0x2B820..=0x2CEAD | 0x2CEAE..=0x2CEAF | 0x2CEB0..=0x2EBE0 | 0x2EBE1..=0x2EBEF | 0x2EBF0..=0x2EE5D | 0x2EE5E..=0x2F7FF | 0x2F800..=0x2FA1D | 0x2FA1E..=0x2FA1F | 0x2FA20..=0x2FFFD | 0x30000..=0x3134A | 0x3134B..=0x3134F | 0x31350..=0x33479 | 0x3347A..=0x3FFFD)
    }

    pub(crate) fn get_char_width(ch: char) -> i32 {
        if Self::is_wide_char(ch) { 2 } else { 1 }
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

    pub fn parse_spans(&mut self, text: &str) {
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

    pub fn parse_links(&mut self, text: &str) {
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

    pub fn parse_lists(&mut self, text: &str) {
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
            width += Self::get_char_width(Self::get_char(bytes, j, len)) as u32;
            j += len;
        }
        width
    }

    pub fn parse_lines(&mut self, text: &str, line_width: u32) {
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

            let block = !self.show_markers && span.span_type.contains(SpanType::CodeBlock);
            let margin = if block { CODE_MARGIN } else { 0 };
            let limit = if block {
                line_width.saturating_sub(CODE_MARGIN).max(1)
            } else {
                line_width
            };
            if block {
                indent = margin.min(limit - 1);
            }

            let mut seg_start = span_start;
            let mut seg_x = x;
            let mut i = span_start;

            while i < span_end {
                let len = Self::get_char_len(bytes[i]);
                let ch = Self::get_char(bytes, i, len);
                let w = Self::get_char_width(ch) as u32;
                let style = Self::quoted_type(span.span_type, quote);

                if ch == '\n' {
                    Self::push_span(&mut self.buffer, seg_start, i, seg_x, y, style);
                    y += 1;
                    x = margin;
                    i += len;
                    seg_start = i;
                    seg_x = margin;
                    indent = if block {
                        margin.min(limit - 1)
                    } else {
                        Self::list_indent(bytes, i).min(line_width - 1)
                    };
                    quote = !self.show_markers && Self::quoted_line(bytes, i);
                    continue;
                }

                if !ch.is_whitespace()
                    && Self::is_word_start(bytes, span_start, i)
                    && x > indent
                    && x + Self::word_width(bytes, i, span_end) > limit
                {
                    Self::push_span(&mut self.buffer, seg_start, i, seg_x, y, style);
                    y += 1;
                    x = indent;
                    seg_start = i;
                    seg_x = indent;
                }

                if x > indent && x + w > limit {
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

    pub fn parse(&mut self, text: &str, line_width: u32) -> &[Span] {
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
            width += Self::get_char_width(Self::get_char(bytes, i, len)) as u32;
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

    pub fn next_offset(text: &str, offset: u32) -> u32 {
        let bytes = text.as_bytes();
        let i = (offset as usize).min(bytes.len());
        if i >= bytes.len() {
            return bytes.len() as u32;
        }
        (i + Self::get_char_len(bytes[i])).min(bytes.len()) as u32
    }

    pub fn prev_offset(text: &str, offset: u32) -> u32 {
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

    pub fn next_visible_offset(&self, text: &str, offset: u32) -> u32 {
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

    pub fn prev_visible_offset(&self, text: &str, offset: u32) -> u32 {
        if self.show_markers {
            return Self::prev_offset(text, offset);
        }
        let offset = self.skip_markers_backward(offset.min(text.len() as u32));
        if offset == 0 {
            return 0;
        }
        self.skip_markers_backward(Self::prev_offset(text, offset))
    }

    pub fn get_position_from_offset(&self, text: &str, offset: u32) -> (u32, u32) {
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

    pub fn rows(&self, text: &str) -> u32 {
        self.get_position_from_offset(text, text.len() as u32).1 + 1
    }

    pub fn get_offset_from_position(&self, text: &str, x: u32, y: u32) -> u32 {
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
                let w = Self::get_char_width(Self::get_char(bytes, i, len)) as u32;
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