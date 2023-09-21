use super::Error;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub (super) enum TokenType {
    Word,
    Separator,
    Eq,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenBrace,
    CloseBrace,  
}
pub(super) struct Token {
    start: usize,
    end: usize,
    token_type: TokenType,
    link: u16
}
pub(super) struct Tokenizer {
    tokens: Vec<Token>
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum CharType {
    Word = 1,
    Space = 2,
    Separator = 3,
    Eq = 4,
    String = 5,
    OpenSquareBracket = 6,
    CloseSquareBracket = 7,
    OpenBrace = 8,
    CloseBrace = 9,
}
impl From<u8> for CharType {
    fn from(value: u8) -> Self {
        match value {
            b' ' | b'\t' | b'\n' | b'\r' => CharType::Space,
            b'=' | b':' => CharType::Eq,
            b'"' | b'\'' => CharType::String,
            b';' | b',' => CharType::Separator,
            b'[' => CharType::OpenSquareBracket,
            b']' => CharType::CloseSquareBracket,
            b'{' => CharType::OpenBrace,
            b'}' => CharType::CloseBrace,
            _ => CharType::Word,
        }
    }
}
impl Tokenizer {
    fn skip(buf: &[u8], start: usize) -> usize {
        let len = buf.len();
        if start >= len {
            return start;
        }
        let mut pos = start;
        let ctype = CharType::from(buf[pos]);
        while (pos < len) && (CharType::from(buf[pos]) == ctype) {
            pos += 1;
        }
        pos
    }
    fn skip_string(buf: &[u8], start: usize) -> (usize, u8) {
        let len = buf.len();
        if start >= len {
            return (start, 0);
        }
        let ch = buf[start];
        if (start + 2 < len) && (buf[start + 1] == ch) && (buf[start + 2] == ch) {
            // either """"...""" or '''...'''
            let mut pos = start + 3;
            while pos + 2 < len {
                if (buf[pos] == ch) && (buf[pos + 1] == ch) && (buf[pos + 2] == ch) {
                    return (pos + 3, 3);
                }
                pos+=1;
            }
            // error --> incomplete string
            (0, 0)
        } else {
            // simple string "..." or '...'
            let mut pos = start + 1;
            while (pos < len) && (buf[pos] != ch) {
                pos += 1;
            }
            if buf[pos] == ch {
                return (pos + 1, 1);
            }
            // incomplete string
            (0, 0)
        }
    }
    fn new(text: &str)->Result<Self,Error> {
        let buf = text.as_bytes();
        let len = buf.len();
        let t = Tokenizer {
            tokens: Vec::with_capacity(16)
        };
        let mut pos = 0usize;
        
        while pos<len {
            let ctype = CharType::from(buf[pos]);
            match ctype {
                CharType::Word => todo!(),
                CharType::Space => pos = Tokenizer::skip(buf, pos),
                CharType::Separator => todo!(),
                CharType::Eq => todo!(),
                CharType::String => todo!(),
                CharType::OpenSquareBracket => todo!(),
                CharType::CloseSquareBracket => todo!(),
                CharType::OpenBrace => todo!(),
                CharType::CloseBrace => todo!(),
            }
        }

        Ok(t)
    }
}
