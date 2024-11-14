use crate::utils::Caption;

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
//  X,Y, char_attr, align are left pub(crate) to allow direct access to those filed within the crate
// only members that don't have an associated flag are public(crate)

/// A structure that contains information about how a text should be displayed on the screen.
pub struct TextFormat {
    flags: TextFormatFlags,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(super) width: u16,
    pub(crate) char_attr: CharAttribute,
    pub(super) hotkey_attr: CharAttribute,
    pub(super) hotkey_pos: u32,
    pub(super) chars_count: u16,
    pub(crate) align: TextAlignament,
    pub(super) text_wrap: TextWrap,
}

impl TextFormat {
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
    pub fn set_attribute(&mut self, attr: CharAttribute) {
        self.char_attr = attr;
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
    pub fn clear_hotkey(&mut self) {
        self.flags.remove(TextFormatFlags::Hotkey);
    }
    #[inline(always)]
    pub fn set_wrap(&mut self, wrap: TextWrap, width: u16) {
        self.text_wrap = wrap;
        self.width = width;
        self.flags.set(TextFormatFlags::Width | TextFormatFlags::MultiLine);
    }
    #[inline(always)]
    pub fn set_truncate_width(&mut self, width: u16) {
        self.width = width;
        self.flags.remove(TextFormatFlags::MultiLine);
        self.flags.set(TextFormatFlags::Width);        
    }
    #[inline(always)]
    pub fn set_chars_count(&mut self, value: u16) {
        self.chars_count = value;
        self.flags.set(TextFormatFlags::CharsCount);
    }

    // inner methods
    #[inline(always)]
    pub(crate) fn width(&self) -> u16 {
        self.width
    }
    #[inline(always)]
    pub(crate) fn set_hotkey_from_caption(&mut self, attr: CharAttribute, caption: &Caption) {
        if caption.has_hotkey() {
            self.set_hotkey(attr, caption.hotkey_pos().unwrap() as u32);
        } else {
            self.clear_hotkey();
        }
    }
}

impl Default for TextFormat {
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

pub struct TextFormatBuilder {
    format: TextFormat,
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
        self
    }
    #[inline(always)]
    pub fn align(mut self, align: TextAlignament) -> Self {
        self.format.align = align;
        self
    }
    #[inline(always)]
    pub fn chars_count(mut self, value: u16) -> Self {
        self.format.set_chars_count(value);
        self
    }
    #[inline(always)]
    pub fn wrap(mut self, wrap: TextWrap, width: u16) -> Self {
        self.format.set_wrap(wrap, width);
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
        self.format.set_truncate_width(width);
        self
    }
    #[inline(always)]
    pub fn build(self) -> TextFormat {
        self.format
    }
}
