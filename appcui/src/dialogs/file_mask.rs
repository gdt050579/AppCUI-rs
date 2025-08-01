static LOWER_CASE_TABLE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
    39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 97, 98, 99, 100, 101, 102, 103, 104, 105,
    106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133,
    134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162,
    163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
    221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharType {
    Space,
    Assign,
    OpenSquareBracket,
    CloseSquareBracket,
    Comma,
    Word,
    Invalid,
}
impl CharType {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b' ' | b'\n' | b'\r' | b'\t' => CharType::Space,
            b'=' | b':' => CharType::Assign,
            b'[' => CharType::OpenSquareBracket,
            b']' => CharType::CloseSquareBracket,
            b',' => CharType::Comma,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' | b'.' => CharType::Word,
            _ => CharType::Invalid,
        }
    }
    fn name(&self) -> &str {
        match self {
            CharType::Space => "space",
            CharType::Assign => "assignment operator ('=' or ':')",
            CharType::OpenSquareBracket => "open square bracket ('[')",
            CharType::CloseSquareBracket => "close square bracket (']')",
            CharType::Comma => "comma (',') separator",
            CharType::Word => "word character (A-Z, a-z, 0-9, _, -, .)",
            CharType::Invalid => "invalid character",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Extension {
    hash: u32,
    start: u16,
    end: u16,
}
#[derive(Debug)]
pub(super) struct FileMask {
    name: String,
    extension_string: String,
    extensions_hash: Vec<Extension>,
}
impl FileMask {
    fn compute_hash(buf: &[u8]) -> u32 {
        if buf.is_empty() {
            return 0;
        }
        let mut hash = 0x811c9dc5u32;
        let mut idx = 0usize;
        while idx < buf.len() {
            hash ^= LOWER_CASE_TABLE[buf[idx] as usize] as u32;
            hash = hash.wrapping_mul(0x01000193u32);
            idx += 1;
        }
        hash
    }
    fn extension_pos(buf: &[u8]) -> Option<usize> {
        if buf.is_empty() {
            return None;
        }
        // search where the extension starts
        let mut idx = buf.len() - 1;
        while idx > 0 {
            if buf[idx] == b'.' {
                return Some(idx + 1);
            }
            if buf[idx] == b'\\' || buf[idx] == b'/' {
                // no extension
                break;
            }
            idx -= 1;
        }
        None
    }
    fn create_error(msg: &str, buffer: &[u8], offset: usize) -> String {
        let mut s = String::from(msg);
        s.push('\n');
        let start = offset.saturating_sub(10);
        let end = (start + 30).min(buffer.len());
        for val in buffer.iter().take(end).skip(start) {
            s.push((*val) as char);
        }
        s.push('\n');
        for i in start..end {
            if i == offset {
                s.push('^');
            } else {
                s.push(' ');
            }
        }
        s
    }
    fn skip_spaces(buffer: &[u8], start: usize) -> usize {
        if start >= buffer.len() {
            return buffer.len();
        }
        let mut index = start;
        let len = buffer.len();
        while (index < len) && (CharType::from_byte(buffer[index]) == CharType::Space) {
            index += 1;
        }
        index
    }
    fn skip_type(buffer: &[u8], start: usize) -> usize {
        if start >= buffer.len() {
            return buffer.len();
        }
        let t = CharType::from_byte(buffer[start]);
        let mut index = start;
        let len = buffer.len();
        while (index < len) && (CharType::from_byte(buffer[index]) == t) {
            index += 1;
        }
        index
    }
    fn skip_word(buffer: &[u8], start: usize) -> usize {
        if start >= buffer.len() {
            return buffer.len();
        }
        let mut index = start;
        let len = buffer.len();
        loop {
            while (index < len) && (CharType::from_byte(buffer[index]) == CharType::Word) {
                index += 1;
            }
            let should_repeat = if index < len && buffer[index] == b' ' {
                let mut next_word_index = index;
                while (next_word_index < len) && ((buffer[next_word_index] == b' ') || (buffer[next_word_index] == b'\t')) {
                    next_word_index += 1;
                }
                if (next_word_index < len) && (CharType::from_byte(buffer[next_word_index]) == CharType::Word) {
                    index = next_word_index;
                    true
                } else {
                    false
                }
            } else {
                false
            };
            if !should_repeat {
                break;
            }
        }
        index
    }
    fn expect_type(buffer: &[u8], start: usize, t: CharType) -> Result<(), String> {
        if start >= buffer.len() {
            return Err(FileMask::create_error(
                &format!("Unexpecting end of file mask definition. Expecting a {}", t.name()),
                buffer,
                start,
            ));
        }
        let current = CharType::from_byte(buffer[start]);
        if current != t {
            return Err(FileMask::create_error(
                &format!("Expected {} but got {}", t.name(), current.name()),
                buffer,
                start,
            ));
        }
        Ok(())
    }
    fn expect_type_or_type(buffer: &[u8], start: usize, t1: CharType, t2: CharType) -> Result<CharType, String> {
        if start >= buffer.len() {
            return Err(FileMask::create_error(
                &format!(
                    "Unexpecting end of file mask definition. Expecting either a {} or a {}",
                    t1.name(),
                    t2.name()
                ),
                buffer,
                start,
            ));
        }
        let current = CharType::from_byte(buffer[start]);
        if current != t1 && current != t2 {
            return Err(FileMask::create_error(
                &format!("Expected {} or {} but got {}", t1.name(), t2.name(), current.name()),
                buffer,
                start,
            ));
        }
        Ok(current)
    }
    pub(super) fn parse(text: &str) -> Result<Vec<Self>, String> {
        // parse text
        let mut v = Vec::new();
        let bytes = text.as_bytes();
        let len = bytes.len();
        let mut start = FileMask::skip_spaces(bytes, 0);
        while start < len {
            start = FileMask::skip_spaces(bytes, start);
            FileMask::expect_type(bytes, start, CharType::Word)?;
            let end_word = FileMask::skip_word(bytes, start);
            let mut mask = FileMask {
                name: text[start..end_word].to_string(),
                extension_string: String::new(),
                extensions_hash: Vec::new(),
            };
            start = FileMask::skip_spaces(bytes, end_word);
            FileMask::expect_type(bytes, start, CharType::Assign)?;
            start = FileMask::skip_spaces(bytes, start + 1);
            FileMask::expect_type(bytes, start, CharType::OpenSquareBracket)?;
            start = FileMask::skip_spaces(bytes, start + 1);
            let start_extension_string = start;
            let mut end_extension_string = start;
            let ct = FileMask::expect_type_or_type(bytes, start, CharType::Word, CharType::CloseSquareBracket)?;
            if ct == CharType::Word {
                loop {
                    // we have a word (extension)
                    let end_word = FileMask::skip_type(bytes, start);
                    let ext_buf = &text.as_bytes()[start..end_word];
                    let hash = if let Some(ext_pos) = FileMask::extension_pos(ext_buf) {
                        FileMask::compute_hash(&ext_buf[ext_pos..])
                    } else {
                        FileMask::compute_hash(ext_buf)
                    };
                    mask.extensions_hash.push(Extension {
                        hash,
                        start: start.saturating_sub(start_extension_string) as u16,
                        end: end_word.saturating_sub(start_extension_string) as u16,
                    });
                    // do something with extension
                    end_extension_string = end_word;
                    start = FileMask::skip_spaces(bytes, end_word);
                    let ct = FileMask::expect_type_or_type(bytes, start, CharType::Comma, CharType::CloseSquareBracket)?;
                    if ct == CharType::CloseSquareBracket {
                        break;
                    }
                    start = FileMask::skip_spaces(bytes, start + 1);
                }
            }
            mask.extension_string = text[start_extension_string..end_extension_string].to_string();
            start = FileMask::skip_spaces(bytes, start + 1);
            if start < len {
                FileMask::expect_type(bytes, start, CharType::Comma)?;
                start = FileMask::skip_spaces(bytes, start + 1);
            }
            // sort the extension hash to prepare for binary search
            mask.extensions_hash.sort_by_key(|&ext| ext.hash);
            v.push(mask);
        }
        Ok(v)
    }
    #[inline(always)]
    pub(super) fn extensions_count(&self) -> usize {
        self.extensions_hash.len()
    }
    #[inline(always)]
    pub(super) fn name(&self) -> &str {
        &self.name
    }
    #[inline(always)]
    pub(super) fn extension(&self, index: usize) -> &str {
        &self.extension_string[self.extensions_hash[index].start as usize..self.extensions_hash[index].end as usize]
    }
    pub(super) fn matches(&self, file_name: &str) -> bool {
        if self.extensions_hash.is_empty() {
            return true;
        }
        let bytes = file_name.as_bytes();
        let hash = if let Some(pos) = FileMask::extension_pos(bytes) {
            FileMask::compute_hash(&bytes[pos..])
        } else {
            0
        };
        self.extensions_hash.binary_search_by_key(&hash, |&ext| ext.hash).is_ok()
    }
}
