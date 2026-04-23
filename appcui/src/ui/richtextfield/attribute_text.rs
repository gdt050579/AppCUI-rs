use crate::{graphics::{CharFlags, Character, Color}, prelude::CharAttribute};

/// Mutable view of [`RichTextField`](super::RichTextField) content passed to the `on_color` callback.
/// Indices are glyph (character) positions; out-of-range operations are ignored (debug builds assert).
pub struct AttributeText<'a> {
    pub(super) chars: &'a mut [Character],
}

impl<'a> AttributeText<'a> {
    /// Returns the number of characters (glyphs) in the text.
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.chars.len()
    }

    /// Returns the character at the specified index or None if the index is out of range.
    #[inline(always)]
    pub fn char(&self, index: usize) -> Option<char> {
        self.chars.get(index).map(|c| c.code)
    }

    /// Sets the character at `index`.
    ///
    /// Out-of-range indices are ignored in release builds and asserted in debug builds.
    pub fn set_char(&mut self, index: usize, ch: char) {
        if let Some(c) = self.chars.get_mut(index) {
            c.code = ch;
        } else {
            debug_assert!(false, "AttributeText::set_color index out of range");
        }       
    }

    /// Sets the style attributes for the character at `index`.
    ///
    /// Out-of-range indices are ignored in release builds and asserted in debug builds.
    pub fn set_attr(&mut self, index: usize, attr: CharAttribute) {
        if let Some(ch) = self.chars.get_mut(index) {
            ch.foreground = attr.foreground;
            ch.background = attr.background;
            ch.flags = attr.flags;
        } else {
            debug_assert!(false, "AttributeText::set_color index out of range");
        }
    }

    /// Sets the style attributes for the half-open range `[start, end)`.
    ///
    /// Bounds are clamped to the available text length.
    pub fn set_range_attr(&mut self, start: usize, end: usize, attr: CharAttribute) {
        let len = self.chars.len();
        let start = start.min(len);
        let end = end.min(len);
        if start >= end {
            return;
        }
        for ch in &mut self.chars[start..end] {
            ch.foreground = attr.foreground;
            ch.background = attr.background;
            ch.flags = attr.flags;
        }
    }

    /// Resets all characters to the specified attributes.
    pub fn reset_all(&mut self, attr: CharAttribute) {
        for ch in self.chars.iter_mut() {
            ch.foreground = attr.foreground;
            ch.background = attr.background;
            ch.flags = attr.flags;
        }
    }
}
