use super::CharAttribute;

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
    Word,
}
impl Default for TextWrap {
    fn default() -> Self {
        TextWrap::None
    }
}

pub struct TextFormat {
    pub x: i32,
    pub y: i32,
    pub width: Option<u16>,
    pub char_attr: CharAttribute,
    pub hotkey_attr: Option<CharAttribute>,
    pub hotkey_pos: Option<usize>,
    pub chars_count: Option<u16>,
    pub align: TextAlignament,
    pub text_wrap: TextWrap,
    pub multi_line: bool,
}

impl TextFormat {
    pub fn new(
        x: i32,
        y: i32,
        char_attr: CharAttribute,
        align: TextAlignament,
        multi_line: bool,
    ) -> Self {
        TextFormat {
            x,
            y,
            char_attr,
            align,
            multi_line,
            ..Default::default()
        }
    }
    pub fn single_line_with_hotkey(
        x: i32,
        y: i32,
        char_attr: CharAttribute,
        hotkey_attr: CharAttribute,
        hotkey_pos: usize,
        align: TextAlignament,
    ) -> Self {
        TextFormat {
            x,
            y,
            char_attr,
            align,
            multi_line: false,
            hotkey_attr: Some(hotkey_attr),
            hotkey_pos: Some(hotkey_pos),
            ..Default::default()
        }
    }
    pub fn single_line(x: i32, y: i32, char_attr: CharAttribute, align: TextAlignament) -> Self {
        TextFormat {
            x,
            y,
            char_attr,
            align,
            multi_line: false,
            ..Default::default()
        }
    }
    pub fn multi_line(x: i32, y: i32, char_attr: CharAttribute, align: TextAlignament) -> Self {
        TextFormat {
            x,
            y,
            char_attr,
            align,
            multi_line: true,
            ..Default::default()
        }
    }
    pub fn multi_line_with_text_wrap(
        x: i32,
        y: i32,
        width: u16,
        char_attr: CharAttribute,
        align: TextAlignament,
        text_wrap: TextWrap,
    ) -> Self {
        TextFormat {
            x,
            y,
            char_attr,
            align,
            text_wrap,
            multi_line: true,
            width: Some(width),
            ..Default::default()
        }
    }
}

impl Default for TextFormat {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: None,
            char_attr: Default::default(),
            hotkey_attr: None,
            hotkey_pos: None,
            chars_count: None,
            align: TextAlignament::Left,
            text_wrap: TextWrap::None,
            multi_line: false,
        }
    }
}
