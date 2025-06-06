#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub(crate) enum CharClass {
    Word,
    Operator,
    Bracket,
    String,
    Space,
    Other,
}
impl From<char> for CharClass {
    fn from(value: char) -> Self {
        match value {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => CharClass::Word,
            ' ' | '\t' | '\n' | '\r' => CharClass::Space,
            '(' | ')' | '[' | ']' | '{' | '}' => CharClass::Bracket,
            '+' | '-' | '*' | '.' | '/' | '\\' | '!' | '=' | '&' | '|' | ':' | '@' | '%' | '^' | '~' | '<' | '>' | '?' | ',' | ';' | '#' | '$' => {
                CharClass::Operator
            }
            '"' | '\'' | '`' => CharClass::String,
            '\u{0080}'..=char::MAX => CharClass::Word,
            _ => CharClass::Other,
        }
    }
}
