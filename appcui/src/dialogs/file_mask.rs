use chrono::offset;

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
pub(super) struct FileMask {}
impl FileMask {
    fn create_error(msg: &str, buffer: &[u8], offset: usize) -> String {
        let mut s = String::from(msg);
        s.push_str("\n");
        let start = offset.saturating_sub(10);
        let end = (start + 30).min(buffer.len());
        for i in start..end {
            s.push(buffer[i] as char);
        }
        s.push_str("\n");
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
    fn new(text: &str) -> Result<Self, String> {
        // parse text
        let bytes = text.as_bytes();
        let len = bytes.len();
        let mut start = FileMask::skip_spaces(bytes, 0);
        while start < len {
            start = FileMask::skip_spaces(bytes, start);
            FileMask::expect_type(bytes, start, CharType::Word)?;
            let end_word = FileMask::skip_type(bytes, start);
            let word = &text[start..end_word];
            start = FileMask::skip_spaces(bytes, end_word);
            FileMask::expect_type(bytes, start, CharType::Assign)?;
            start = FileMask::skip_spaces(bytes, start + 1);
            FileMask::expect_type(bytes, start, CharType::OpenSquareBracket)?;
            start = FileMask::skip_spaces(bytes, start + 1);
            let ct = FileMask::expect_type_or_type(bytes, start, CharType::Word, CharType::CloseSquareBracket)?;
            if ct == CharType::Word {
                loop {
                    // we have a word (extension)
                    let end_word = FileMask::skip_type(bytes, start);
                    let ext = &text[start..end_word];
                    // do something with extension
                    start = FileMask::skip_spaces(bytes, end_word);
                    let ct = FileMask::expect_type_or_type(bytes, start, CharType::Comma, CharType::CloseSquareBracket)?;
                    if ct == CharType::CloseSquareBracket {
                        break;
                    }
                    start += 1;
                }
            }
            start = FileMask::skip_spaces(bytes, start + 1);
            if start<len {
                FileMask::expect_type(bytes, start, CharType::Comma)?;
                start = FileMask::skip_spaces(bytes, start + 1);
            }
        }
        Ok(FileMask {})
    }
}
