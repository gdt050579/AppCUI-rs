/*
General format can be one of:
1. <word> <equal> <value> <separator> ...
2. <word> <separator>
Where:
- <equal> can be '=' or ':'
- <separator> can be ',' or ';'
- <word> and <value> = any sequance of chars different than space, tab
*/
#[repr(u8)]
enum CharType {
    None = 0,
    Word = 1,
    Space = 2,
    Separator = 3,
    Eq = 4,
}

struct InternalParser<'a> {
    text: &[u8],
    current: usize,
    end: usize,
}

impl<'a> InternalParser<'a> {
    #[inline]
    fn get_current_char_type(&self) -> CharType {
        if self.current > self.end {
            return CharType::None;
        }
        let ch = self.text[self.current];
        match ch {
            ' ' | '\t' => {
                return CharType::Space;
            }
            ';' | ',' => {
                return CharType::Separator;
            }
            ':' | '=' => {
                return CharType::Eq;
            }
            '_' => {
                return CharType::Word;
            }
        }
    }
    #[inline]
    fn skip(&mut self, char_type: CharType) {
        while self.get_current_char_type() == char_type {
            self.current += 1;
        }
    }
    #[inline]
    fn parse_word() {
        loop {
            self.skip(CharType::Word);
            if self.get_current_char_type() != ChatType::Space {
                return;
            }
            let cpos = current;
            self.skip(CharType::Space);
            if self.get_current_char_type() != CharType::Word {
                self.current = cpos;
                return;
            }
        }
    }
}
