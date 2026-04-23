use crate::graphics::{CharFlags, Color, Character};

/// Mutable view of [`RichTextField`](super::RichTextField) content passed to the `on_color` callback.
/// Indices are glyph (character) positions; out-of-range operations are ignored (debug builds assert).
pub struct AttributeText<'a> {
    pub(super) chars: &'a mut [Character],
}

impl<'a> AttributeText<'a> {
    #[inline(always)]
    pub fn char_count(&self) -> usize {
        self.chars.len()
    }

    #[inline(always)]
    pub fn char_at(&self, index: usize) -> char {
        if index < self.chars.len() {
            self.chars[index].code
        } else {
            debug_assert!(false, "AttributeText::char_at index out of range");
            '\0'
        }
    }

    pub fn set_color(&mut self, index: usize, foreground: Color, background: Color) {
        if let Some(ch) = self.chars.get_mut(index) {
            ch.foreground = foreground;
            ch.background = background;
        } else {
            debug_assert!(false, "AttributeText::set_color index out of range");
        }
    }

    pub fn set_flags(&mut self, index: usize, flags: CharFlags) {
        if let Some(ch) = self.chars.get_mut(index) {
            ch.flags = flags;
        } else {
            debug_assert!(false, "AttributeText::set_flags index out of range");
        }
    }

    pub fn set_color_for_range(&mut self, start: usize, end: usize, foreground: Color, background: Color) {
        let len = self.chars.len();
        let start = start.min(len);
        let end = end.min(len);
        if start >= end {
            return;
        }
        for ch in &mut self.chars[start..end] {
            ch.foreground = foreground;
            ch.background = background;
        }
    }

    pub fn set_flags_for_range(&mut self, start: usize, end: usize, flags: CharFlags) {
        let len = self.chars.len();
        let start = start.min(len);
        let end = end.min(len);
        if start >= end {
            return;
        }
        for ch in &mut self.chars[start..end] {
            ch.flags = flags;
        }
    }

    pub fn reset_all(&mut self) {
        for ch in self.chars.iter_mut() {
            ch.foreground = Color::Transparent;
            ch.background = Color::Transparent;
            ch.flags = CharFlags::None;
        }
    }
}
