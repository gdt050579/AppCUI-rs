use crate::parameter_parser::size::Size;

use super::{TokenType, Tokenizer};

macro_rules! check_token {
    ($script:expr, $token: expr, $value: expr, $type: expr) => {
        assert_eq!($token.get_text($script), $value);
        assert_eq!($token.get_type(), $type);
    };
}
#[test]
fn check_tokenizer() {
    let script = "a = 10, b=20, 
                  value=['100', 20; 
                      {key:value,
                       key2:[1,2,3],
                    \"str\" = aaa}
                ],   value='''long string'''   ";
    let t = Tokenizer::new(script).unwrap();
    assert_eq!(t.count(), 39);
    check_token!(script, t.get(0), "a", TokenType::Word);
    check_token!(script, t.get(1), "=", TokenType::Eq);
    check_token!(script, t.get(2), "10", TokenType::Word);
    check_token!(script, t.get(3), ",", TokenType::Separator);
    check_token!(script, t.get(4), "b", TokenType::Word);
    check_token!(script, t.get(5), "=", TokenType::Eq);
    check_token!(script, t.get(6), "20", TokenType::Word);
    check_token!(script, t.get(7), ",", TokenType::Separator);
    check_token!(script, t.get(8), "value", TokenType::Word);
    check_token!(script, t.get(9), "=", TokenType::Eq);
    check_token!(script, t.get(10), "[", TokenType::OpenSquareBracket);
    assert_eq!(t.get(10).get_link(), 34);
    check_token!(script, t.get(11), "100", TokenType::Word);
    check_token!(script, t.get(12), ",", TokenType::Separator);
    check_token!(script, t.get(13), "20", TokenType::Word);
    check_token!(script, t.get(14), ";", TokenType::Separator);
    check_token!(script, t.get(15), "{", TokenType::OpenBrace);
    assert_eq!(t.get(15).get_link(), 33);
    check_token!(script, t.get(16), "key", TokenType::Word);
    check_token!(script, t.get(17), ":", TokenType::Eq);
    check_token!(script, t.get(18), "value", TokenType::Word);
    check_token!(script, t.get(19), ",", TokenType::Separator);
    check_token!(script, t.get(20), "key2", TokenType::Word);
    check_token!(script, t.get(21), ":", TokenType::Eq);
    check_token!(script, t.get(22), "[", TokenType::OpenSquareBracket);
    assert_eq!(t.get(22).get_link(), 28);
    check_token!(script, t.get(23), "1", TokenType::Word);
    check_token!(script, t.get(24), ",", TokenType::Separator);
    check_token!(script, t.get(25), "2", TokenType::Word);
    check_token!(script, t.get(26), ",", TokenType::Separator);
    check_token!(script, t.get(27), "3", TokenType::Word);
    check_token!(script, t.get(28), "]", TokenType::CloseSquareBracket);
    assert_eq!(t.get(28).get_link(), 22);
    check_token!(script, t.get(29), ",", TokenType::Separator);
    check_token!(script, t.get(30), "str", TokenType::Word);
    check_token!(script, t.get(31), "=", TokenType::Eq);
    check_token!(script, t.get(32), "aaa", TokenType::Word);
    check_token!(script, t.get(33), "}", TokenType::CloseBrace);
    assert_eq!(t.get(33).get_link(), 15);
    check_token!(script, t.get(34), "]", TokenType::CloseSquareBracket);
    assert_eq!(t.get(34).get_link(), 10);
    check_token!(script, t.get(35), ",", TokenType::Separator);
    check_token!(script, t.get(36), "value", TokenType::Word);
    check_token!(script, t.get(37), "=", TokenType::Eq);
    check_token!(script, t.get(38), "long string", TokenType::Word);
}

#[test]
fn check_size() {
    assert_eq!(Size::from_str("10x50"), Some(Size { width: 10, height: 50 }));
    assert_eq!(Size::from_str("   123 x   4567   "), Some(Size { width: 123, height: 4567 }));
    assert_eq!(Size::from_str("   1     X   4567   "), Some(Size { width: 1, height: 4567 }));
    assert_eq!(Size::from_str("   1234 ,  2   "), Some(Size { width: 1234, height: 2 }));
    assert_eq!(Size::from_str("   0 ,  0   "), None);
    assert_eq!(Size::from_str("   123    "), None);
    assert_eq!(Size::from_str("5x2a"), None);
    assert_eq!(Size::from_str("5 x "), None);
    assert_eq!(Size::from_str(" x 2"), None);
    assert_eq!(Size::from_str(" 5 x 2     a"), None);
}
