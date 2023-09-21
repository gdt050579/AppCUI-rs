use super::Error;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) enum TokenType {
    Word,
    Separator,
    Eq,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenBrace,
    CloseBrace,
}
#[derive(Copy, Clone, Debug)]
pub(super) struct Token {
    start: usize,
    end: usize,
    token_type: TokenType,
    link: u16,
}
impl Token {
    const NO_LINK: u16 = 0xFFFFu16;
    fn new(start: usize, end: usize, token_type: TokenType) -> Self {
        Self {
            start,
            end,
            token_type,
            link: Token::NO_LINK,
        }
    }
    fn with_link(start: usize, end: usize, token_type: TokenType, link: u16) -> Self {
        Self {
            start,
            end,
            token_type,
            link,
        }
    }
}
pub(super) struct Tokenizer {
    tokens: Vec<Token>,
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
    const LOCAL_STACK_MAX_SIZE: usize = 16;
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
                    return (pos, 3);
                }
                pos += 1;
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
                return (pos, 1);
            }
            // incomplete string
            (0, 0)
        }
    }
    pub(super) fn new(text: &str) -> Result<Self, Error> {
        let buf = text.as_bytes();
        let len = buf.len();
        let mut local_stack = [0usize; Tokenizer::LOCAL_STACK_MAX_SIZE + 2];
        let mut local_stack_top = 0;
        let mut t = Tokenizer {
            tokens: Vec::with_capacity(16),
        };
        let mut pos = 0usize;

        while pos < len {
            let ctype = CharType::from(buf[pos]);
            match ctype {
                CharType::Word => {
                    let next = Tokenizer::skip(buf, pos);
                    t.tokens.push(Token::new(pos, next, TokenType::Word));
                    pos = next;
                }
                CharType::Space => pos = Tokenizer::skip(buf, pos),
                CharType::Separator => {
                    t.tokens.push(Token::new(pos, pos + 1, TokenType::Separator));
                    pos += 1;
                }
                CharType::Eq => {
                    t.tokens.push(Token::new(pos, pos + 1, TokenType::Eq));
                    pos += 1;
                }
                CharType::String => {
                    let next = Tokenizer::skip_string(buf, pos);
                    if next.1 == 0 {
                        // incomplete string scenario
                        return Err(Error::new(text, "Incomplete string (you should add string terminator)", pos, text.len()));
                    }
                    t.tokens.push(Token::new(pos + next.1 as usize, next.0, TokenType::Word));
                    pos = next.0 + next.1 as usize;
                }
                CharType::OpenSquareBracket | CharType::OpenBrace => {
                    if local_stack_top >= Tokenizer::LOCAL_STACK_MAX_SIZE {
                        return Err(Error::new(text, "Too many levels ( [..] or {..} ). Max depth allowd is 16", pos, pos + 1));
                    }
                    local_stack_top += 1;
                    local_stack[local_stack_top] = t.tokens.len();
                    t.tokens.push(Token::new(
                        pos,
                        pos + 1,
                        if ctype == CharType::OpenSquareBracket {
                            TokenType::OpenSquareBracket
                        } else {
                            TokenType::OpenBrace
                        },
                    ));
                    pos += 1;
                }
                CharType::CloseSquareBracket => {
                    if local_stack_top == 0 {
                        return Err(Error::new(
                            text,
                            "Un-macthed close square bracket ']'. Did you meant to add a '[' before it ?",
                            pos,
                            pos + 1,
                        ));
                    }
                    let link = local_stack[local_stack_top];
                    local_stack_top -= 1;
                    if t.tokens[link].token_type != TokenType::OpenSquareBracket {
                        return Err(Error::new(
                            text,
                            "Incorect macth: '{' with ']'. Did you meant to add a '}' instead of a ']' ?",
                            t.tokens[link].start,
                            pos + 1,
                        ));
                    }
                    t.tokens[link].link = t.tokens.len() as u16;
                    t.tokens.push(Token::with_link(pos, pos + 1, TokenType::CloseSquareBracket, link as u16));
                    pos += 1;
                }
                CharType::CloseBrace => {
                    if local_stack_top == 0 {
                        return Err(Error::new(
                            text,
                            "Un-macthed close brace '}'. Did you meant to add a '{' before it ?",
                            pos,
                            pos + 1,
                        ));
                    }
                    let link = local_stack[local_stack_top];
                    local_stack_top -= 1;
                    if t.tokens[link].token_type != TokenType::OpenBrace {
                        return Err(Error::new(
                            text,
                            "Incorect macth: '[' with '}'. Did you meant to add a ']' instead of a '}' ?",
                            t.tokens[link].start,
                            pos + 1,
                        ));
                    }
                    t.tokens[link].link = t.tokens.len() as u16;
                    t.tokens.push(Token::with_link(pos, pos + 1, TokenType::CloseBrace, link as u16));
                    pos += 1;
                }
            }
            // check the number of tokens
            if t.tokens.len() >= 0xFFFE {
                return Err(Error::new(
                    text,
                    "Too make tokens (param list is too large). Max allowed is 0xFFFF !",
                    pos,
                    text.len(),
                ));
            }
        }
        // extra errors
        if local_stack_top != 0 {
            let token = t.tokens[local_stack[local_stack_top]];
            if token.token_type == TokenType::OpenBrace {
                return Err(Error::new(
                    text,
                    "Un-closed brace: '{'. Have you forgot to add an '}' ?",
                    token.start,
                    token.start + 1,
                ));
            } else {
                return Err(Error::new(
                    text,
                    "Un-closed open bracket: '['. Have you forgot to add an ']' ?",
                    token.start,
                    token.start + 1,
                ));
            }
        }
        Ok(t)
    }
    #[inline(always)]
    fn len(&self) -> usize {
        self.tokens.len()
    }
    #[inline(always)]
    fn get(&self, index: usize) -> &Token {
        &self.tokens[index]
    }
}
