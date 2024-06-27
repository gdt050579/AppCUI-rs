use crate::graphics::*;
use crate::prelude::*;
use crate::utils::*;

pub struct Column {
    pub(super) name: Caption,
    pub(super) width: u8,
    pub(super) alignment: TextAlignament,
    pub(super) tooltip: String,
    pub(super) x: i32,
}

impl Column {
    pub fn new(name: &str, width: u8, alignment: TextAlignament) -> Self {
        Self {
            name: Caption::new(name, ExtractHotKeyMethod::CtrlPlusKey),
            width,
            alignment,
            tooltip: String::new(),
            x: 0,
        }
    }
    pub fn set_name(&mut self, name: &str) {
        self.name.set_text(name, ExtractHotKeyMethod::CtrlPlusKey)
    }
    pub fn set_tooltip(&mut self, tooltip: &str) {
        self.tooltip.clear();
        self.tooltip.push_str(tooltip);
    }
    pub fn set_alignment(&mut self, alignment: TextAlignament) {
        self.alignment = alignment;
    }
    pub fn name(&self) -> &str {
        self.name.text()
    }
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }
    pub fn alignment(&self) -> TextAlignament {
        self.alignment
    }
    pub fn width(&self) -> u8 {
        self.width
    }
    pub(super) fn paint(&self, surface: &mut Surface, char_attr: CharAttribute, hotkey_attr: CharAttribute) {
        let x = match self.alignment {
            TextAlignament::Left => self.x + 1,
            TextAlignament::Center => self.x + (self.width as i32) / 2,
            TextAlignament::Right => self.x + (self.width as i32) - 1,
        };
        let format = TextFormat {
            x,
            y: 0,
            width: Some(self.width as u16),
            char_attr,
            hotkey_attr: Some(hotkey_attr),
            hotkey_pos: self.name.hotkey_pos(),
            chars_count: Some(self.name.chars_count() as u16),
            align: self.alignment,
            text_wrap: TextWrap::None,
            multi_line: false,
        };
        surface.write_text(self.name.text(), &format);
    }
}
