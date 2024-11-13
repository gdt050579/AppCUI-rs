use super::CharAttribute;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum TextAlignament {
    #[default]
    Left,
    Center,
    Right,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum TextWrap {
    #[default]
    None,
    Character,
    Word,
}

/// A structure that contains information about how a text should be displayed on the screen.
pub struct TextFormat {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: Option<u16>,
    pub(crate) char_attr: CharAttribute,
    pub(crate) hotkey_attr: Option<CharAttribute>,
    pub(crate) hotkey_pos: Option<usize>,
    pub(crate) chars_count: Option<u16>,
    pub(crate) align: TextAlignament,
    pub(crate) text_wrap: TextWrap,
    pub(crate) multi_line: bool,
}

impl TextFormat {
    /// Creates a new text format structure with a coordinate, character attribute, alignment and multi-line flag.
    /// The rest of the fields are set to default values.
    pub fn new(x: i32, y: i32, char_attr: CharAttribute, align: TextAlignament, multi_line: bool) -> Self {
        TextFormat {
            x,
            y,
            char_attr,
            align,
            multi_line,
            ..Default::default()
        }
    }

    /// Creates a new text format structure with a coordinate, character attribute, alignment, multi-line flag and hotkey information (attribute & position).
    /// The rest of the fields are set to default values.
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

    /// Creates a new text format structure with a coordinate, character attribute, and text alignment.
    /// The text is displayed on a single line.
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

    /// Creates a new text format structure with a coordinate, character attribute, and text alignment.
    /// The text is displayed on multiple lines
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

    /// Creates a new text format structure with a coordinate, character attribute, text alignment, and text wrap type.
    pub fn multi_line_with_text_wrap(x: i32, y: i32, width: u16, char_attr: CharAttribute, align: TextAlignament, text_wrap: TextWrap) -> Self {
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

pub struct TextFormatBuilder {
    format: TextFormat,
}

impl TextFormatBuilder {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            format: Default::default(),
        }
    }
    #[inline(always)]
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.format.x = x;
        self.format.y = y;
        self
    }
    #[inline(always)]
    pub fn attribute(mut self, attr: CharAttribute) -> Self {
        self.format.char_attr = attr;
        self
    }
    #[inline(always)]
    pub fn hotkey(mut self, attr: CharAttribute, pos: usize) -> Self {
        self.format.hotkey_attr = Some(attr);
        self.format.hotkey_pos = Some(pos);
        self
    }
    #[inline(always)]
    pub fn align(mut self, align: TextAlignament) -> Self {
        self.format.align = align;
        self
    }
    #[inline(always)]
    pub fn build(self) -> TextFormat {
        self.format
    }
}