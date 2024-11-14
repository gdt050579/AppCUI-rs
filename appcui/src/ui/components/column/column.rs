use crate::graphics::*;
use crate::utils::*;


/// Represents a column in a control that supports such components. A column has a name, a width, an alignment and a tooltip.
pub struct Column {
    pub(crate) name: Caption,
    pub(crate) width: u8,
    pub(crate) alignment: TextAlignament,
    pub(crate) tooltip: String,
    pub(crate) x: i32,
}

impl Column {
    /// Creates a new column with the specified name, width and alignment.
    /// The tooltip is empty, but can further be set using the `set_tooltip` method.
    pub fn new(name: &str, width: u8, alignment: TextAlignament) -> Self {
        Self {
            name: Caption::new(name, ExtractHotKeyMethod::CtrlPlusKey),
            width,
            alignment,
            tooltip: String::new(),
            x: 0,
        }
    }

    /// Sets the name (caption) of the column.
    pub fn set_name(&mut self, name: &str) {
        self.name.set_text(name, ExtractHotKeyMethod::CtrlPlusKey)
    }

    /// Sets the tooltip of the column.
    pub fn set_tooltip(&mut self, tooltip: &str) {
        self.tooltip.clear();
        self.tooltip.push_str(tooltip);
    }

    /// Sets the alignment of the column (left, center or right).
    pub fn set_alignment(&mut self, alignment: TextAlignament) {
        self.alignment = alignment;
    }
    /// Returns the name (caption) of the column.
    #[inline(always)]
    pub fn name(&self) -> &str {
        self.name.text()
    }
    /// Returns the tooltip of the column.
    #[inline(always)]
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }
    /// Returns the alignment of the column (left, center or right).
    #[inline(always)]
    pub fn alignment(&self) -> TextAlignament {
        self.alignment
    }
    /// Returns the width of the column in characters.
    #[inline(always)]
    pub fn width(&self) -> u8 {
        self.width
    }
    pub(crate) fn paint(&self, surface: &mut Surface, char_attr: CharAttribute, hotkey_attr: CharAttribute, fill: bool) {
        let w = self.width.saturating_sub(2) as i32;
        if w <= 0 {
            return;
        }
        let x = match self.alignment {
            TextAlignament::Left => self.x + 1,
            TextAlignament::Center => self.x + 1 + (w / 2),
            TextAlignament::Right => self.x + w,
        };
        if fill {
            surface.fill_horizontal_line_with_size(self.x, 0, self.width as u32, Character::with_attributes(' ', char_attr));
        }
        let mut format = TextFormatBuilder::new()
            .position(x, 0)
            .attribute(char_attr)
            .align(self.alignment)
            .chars_count(self.name.chars_count() as u16)
            .truncate(w as u16)
            .build();
        if self.name.has_hotkey() {
            format.set_hotkey(hotkey_attr, self.name.hotkey_pos().unwrap() as u32);
        }
        surface.write_text_new(self.name.text(), &format);
    }
}
