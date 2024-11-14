use super::CharAttribute;
use EnumBitFlags::EnumBitFlags;

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
    Character,
    Word,
}

#[EnumBitFlags(bits = 8)]
enum TextFormatFlags {
    Hotkey = 0x01,
    MultiLine = 0x02,
    CharsCount = 0x04,
    Width = 0x08,
}

/// A structure that contains information about how a text should be displayed on the screen.
pub struct TextFormatNew {
    flags: TextFormatFlags,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(super) width: u16,
    pub(super) char_attr: CharAttribute,
    pub(super) hotkey_attr: CharAttribute,
    pub(super) hotkey_pos: u32,
    pub(super) chars_count: u16,
    pub(crate) align: TextAlignament,
    pub(super) text_wrap: TextWrap,
}

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
        Self {
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
        Self {
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
        Self {
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
        Self {
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
        Self {
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

impl TextFormatNew {
    pub(super) fn from_old(format: &TextFormat) -> Self {
        Self {
            flags: if format.hotkey_attr.is_some() && format.hotkey_pos.is_some() {
                TextFormatFlags::Hotkey
            } else {
                TextFormatFlags::None
            } | if format.chars_count.is_some() {
                TextFormatFlags::CharsCount
            } else {
                TextFormatFlags::None
            } | if format.width.is_some() {
                TextFormatFlags::Width
            } else {
                TextFormatFlags::None
            } | if format.multi_line {
                TextFormatFlags::MultiLine
            } else {
                TextFormatFlags::None
            },
            x: format.x,
            y: format.y,
            width: format.width.unwrap_or(0),
            char_attr: format.char_attr,
            hotkey_attr: format.hotkey_attr.unwrap_or_default(),
            hotkey_pos: format.hotkey_pos.unwrap_or(0) as u32,
            chars_count: format.chars_count.unwrap_or(0),
            align: format.align,
            text_wrap: format.text_wrap,
        }
    }
    #[inline(always)]
    pub(super) fn has_hotkey(&self) -> bool {
        self.flags.contains(TextFormatFlags::Hotkey)
    }
    #[inline(always)]
    pub(super) fn has_chars_count(&self) -> bool {
        self.flags.contains(TextFormatFlags::CharsCount)
    }
    #[inline(always)]
    pub(super) fn has_width(&self) -> bool {
        self.flags.contains(TextFormatFlags::Width)
    }
    #[inline(always)]
    pub(super) fn is_multi_line(&self) -> bool {
        self.flags.contains(TextFormatFlags::MultiLine)
    }
    #[inline(always)]
    pub fn set_align(&mut self, align: TextAlignament) {
        self.align = align;
    }
    #[inline(always)]
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    #[inline(always)]
    pub fn set_hotkey(&mut self, attr: CharAttribute, pos: u32) {
        self.hotkey_attr = attr;
        self.hotkey_pos = pos;
        self.flags.set(TextFormatFlags::Hotkey);
    }
    #[inline(always)]
    pub fn set_wrap(&mut self, wrap: TextWrap, width: u16) {
        self.text_wrap = wrap;
        self.width = width;
        self.flags.set(TextFormatFlags::Width | TextFormatFlags::MultiLine);
    }
}

impl Default for TextFormatNew {
    fn default() -> Self {
        Self {
            flags: TextFormatFlags::None,
            x: 0,
            y: 0,
            width: 0,
            char_attr: Default::default(),
            hotkey_attr: Default::default(),
            hotkey_pos: 0,
            chars_count: 0,
            align: TextAlignament::Left,
            text_wrap: TextWrap::Character,
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
            text_wrap: TextWrap::Character,
            multi_line: false,
        }
    }
}

pub struct TextFormatBuilder {
    format: TextFormatNew,
}

impl TextFormatBuilder {
    #[inline(always)]
    pub fn new() -> Self {
        Self { format: Default::default() }
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
    pub fn hotkey(mut self, attr: CharAttribute, pos: u32) -> Self {
        self.format.set_hotkey(attr, pos);
        // self.format.hotkey_attr = attr;
        // self.format.hotkey_pos = pos;
        // self.format.flags.set(TextFormatFlags::Hotkey);
        self
    }
    #[inline(always)]
    pub fn align(mut self, align: TextAlignament) -> Self {
        self.format.align = align;
        self
    }
    #[inline(always)]
    pub fn chars_count(mut self, value: u16) -> Self {
        self.format.chars_count = value;
        self.format.flags.set(TextFormatFlags::CharsCount);
        self
    }
    #[inline(always)]
    pub fn wrap(mut self, wrap: TextWrap, width: u16) -> Self {
        self.format.set_wrap(wrap, width);
        // self.format.text_wrap = wrap;
        // self.format.width = width;
        // self.format.flags.set(TextFormatFlags::Width | TextFormatFlags::MultiLine);
        self
    }
    #[inline(always)]
    pub fn multi_line(mut self) -> Self {
        self.format.width = 0;
        self.format.flags.set(TextFormatFlags::MultiLine);
        self.format.flags.remove(TextFormatFlags::Width);
        self
    }
    #[inline(always)]
    pub fn truncate(mut self, width: u16) -> Self {
        self.format.width = width;
        self.format.flags.remove(TextFormatFlags::MultiLine);
        self.format.flags.set(TextFormatFlags::Width);
        self
    }
    #[inline(always)]
    pub fn build(self) -> TextFormatNew {
        self.format
    }
}
