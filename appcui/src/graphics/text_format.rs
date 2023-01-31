use super::{CharAttribute, Character};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextAlignament {
    Left,
    Center,
    Right,
}
impl Default for TextAlignament {
    fn default() -> Self {
        TextAlignament::Left
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextWrap {
    None,
    Character,
}
impl Default for TextWrap {
    fn default() -> Self {
        TextWrap::None
    }
}

#[derive(Default)]
pub struct TextFormat {
    pub x: i32,
    pub y: i32,
    pub width: Option<u16>,
    pub char_attr: CharAttribute,
    pub hotkey_attr: Option<CharAttribute>,
    pub hotkey_pos: Option<usize>,
    pub chars_count: Option<u16>,
    pub align: TextAlignament,
    pub left_margin_char: Option<Character>,
    pub right_margin_char: Option<Character>,
    pub text_wrap: TextWrap,
    pub multi_lines: bool,
}

impl TextFormat {
    pub fn new_single_line(x: i32, y: i32, char_attr: CharAttribute, align: TextAlignament) -> Self {
        TextFormat {
            x,
            y,
            char_attr,
            align,
            multi_lines: false,
            ..Default::default()
        }
    }
}
