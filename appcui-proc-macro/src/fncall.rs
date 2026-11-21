struct Span {
    start: usize,
    end: usize,
}
pub(crate) struct FnCall {
    data: String,
    name: Span,
    params: Vec<Span>,
}
impl FnCall {
    fn skip_spaces(buf: &[u8], start: usize) -> usize {
        let len = buf.len();
        let mut pos = start;
        while (pos < len) && ((buf[pos] == b' ') || (buf[pos] == b'\n') || (buf[pos] == b'\r') || (buf[pos] == b'\t')) {
            pos += 1;
        }
        pos
    }
    fn is_word_character(value: u8) -> bool {
        matches!(value, b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'-' | 128..)
    }
    fn skip_words(buf: &[u8], start: usize) -> usize {
        let len = buf.len();
        let mut pos = start;
        while (pos < len) && Self::is_word_character(buf[pos]) {
            pos += 1;
        }
        pos
    }    
    // format: name (param1, param2, ... )
    pub(crate) fn new(repr: &str) -> Option<Self> {
        let buf = repr.as_bytes();
        let len = buf.len();
        let mut pos = Self::skip_spaces(buf, 0);
        if pos >= len {
            return None;
        }
        let name_start = pos;
        pos = Self::skip_words(buf, pos);
        if pos == name_start {
            return None;
        }
        let name = Span {
            start: name_start,
            end: pos,
        };
        pos = Self::skip_spaces(buf, pos);
        if pos >= len {
            return Some(Self {
                data: repr.to_string(),
                name,
                params: Vec::new(),
            });
        }
        if buf[pos] != b'(' {
            return None;
        }
        pos += 1;
        let mut params = Vec::new();
        loop {
            pos = Self::skip_spaces(buf, pos);
            if pos >= len {
                return None;
            }
            if buf[pos] == b')' {
                pos += 1;
                break;
            }
            if (buf[pos] == b'"') || (buf[pos] == b'\'') {
                let quote = buf[pos];
                pos += 1;
                let start = pos;
                while (pos < len) && (buf[pos] != quote) {
                    pos += 1;
                }
                if pos >= len {
                    return None;
                }
                params.push(Span { start, end: pos });
                pos += 1;
            } else {
                let start = pos;
                while (pos < len) && (buf[pos] != b',') && (buf[pos] != b')') {
                    pos += 1;
                }
                if pos >= len {
                    return None;
                }
                let mut end = pos;
                while end > start {
                    let c = buf[end - 1];
                    if (c == b' ') || (c == b'\n') || (c == b'\r') || (c == b'\t') {
                        end -= 1;
                    } else {
                        break;
                    }
                }
                if end > start {
                    params.push(Span { start, end });
                }
            }
            pos = Self::skip_spaces(buf, pos);
            if pos >= len {
                return None;
            }
            if buf[pos] == b',' {
                pos += 1;
                continue;
            }
            if buf[pos] == b')' {
                pos += 1;
                break;
            }
            return None;
        }
        pos = Self::skip_spaces(buf, pos);
        if pos != len {
            return None;
        }
        Some(Self {
            data: repr.to_string(),
            name,
            params,
        })
    }
    pub(crate) fn name(&self) -> &str {
        &self.data[self.name.start..self.name.end]
    }
    pub(crate) fn params_count(&self) -> usize {
        self.params.len()
    }
    pub(crate) fn param(&self, index: usize) -> Option<&str> {
        if index >= self.params.len() {
            return None;
        }
        Some(&self.data[self.params[index].start..self.params[index].end])
    }
    pub(crate) fn match_name(&self, list: &[(&'static str, &'static str)]) -> Option<&'static str> {
        crate::utils::find_string_in_array(list, self.name())
    }
    pub(crate) fn is_param_number(&self, index: usize) -> bool {
        if index >= self.params.len() {
            return false;
        }
        crate::utils::is_number(&self.data[self.params[index].start..self.params[index].end])
    }
    pub(crate) fn is_param_integer(&self, index: usize) -> bool {
        if index >= self.params.len() {
            return false;
        }
        crate::utils::is_integer(&self.data[self.params[index].start..self.params[index].end])
    }
}