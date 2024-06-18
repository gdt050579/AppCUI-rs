use crate::graphics::*;
use crate::input::*;
use crate::prelude::ControlBase;
use crate::system::*;
use AppCUIProcMacro::*;

use super::ProcessEventResult;

pub struct SearchBar {
    x: i32,
    y: i32,
    width: u16,
    text: String,
    visible: bool,
    edit_mode: bool,
    match_count: u8, // u8::MAX = no match count
    text_offset: u16,
    cursor_offset: u8,
}
impl SearchBar {
    const MIN_WIDTH: u16 = 5;
    const PREFERED_WIDTH: u16 = 14;
    const DRAW_COUNT_MIN_WIDTH: u16 = 9;
    pub fn new(visible: bool) -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            text: String::new(),
            visible,
            edit_mode: false,
            match_count: u8::MAX,
            text_offset: 0,
            cursor_offset: 0,
        }
    }
    #[inline(always)]
    pub(super) fn recompute_layout(&mut self, pos: i32, available_space: i32, control_size: Size) -> i32 {
        if available_space < Self::MIN_WIDTH as i32 {
            self.visible = false;
            return 0;
        }
        if available_space >= Self::PREFERED_WIDTH as i32 {
            self.width = Self::PREFERED_WIDTH;
        } else {
            self.width = available_space as u16;
        }
        self.update_text_offset();
        self.x = pos;
        self.y = control_size.height as i32;
        self.visible = true;
        (self.width as i32) + 1
    }
    pub fn set_match_count(&mut self, count: usize) {
        self.match_count = count.clamp(0, 100) as u8;
        self.update_text_offset();
    }
    pub fn clear_match_count(&mut self) {
        self.match_count = u8::MAX;
        self.update_text_offset();
    }
    fn update_text_offset(&mut self) {
        let match_count_width = if (self.width >= SearchBar::DRAW_COUNT_MIN_WIDTH) && (self.match_count != u8::MAX) {
            6
        } else {
            2
        };
        let tx_chars = self.width.saturating_sub(match_count_width) as usize;
        if tx_chars == 0 {
            self.text_offset = self.text.len() as u16;
        } else {
            if let Some(ofs) = self.text.char_indices().rev().take(tx_chars).map(|(o, _)| o).min() {
                self.text_offset = ofs as u16;
            } else {
                self.text_offset = 0;
            }
        }
        self.cursor_offset = (&self.text[self.text_offset as usize..]).chars().count() as u8;
    }
    fn paint_count(&self, surface: &mut Surface, attr: CharAttribute) {
        let r = self.x + self.width as i32 - 2;
        match self.match_count {
            0..=9 => {
                surface.write_char(r, self.y, Character::with_attributes(48 + self.match_count, attr));
            }
            10..=99 => {
                surface.write_char(r - 1, self.y, Character::with_attributes(48 + (self.match_count / 10), attr));
                surface.write_char(r, self.y, Character::with_attributes(48 + (self.match_count % 10), attr));
            }
            100 => {
                surface.write_string(r - 2, self.y, "99+", attr, false);
            }
            _ => {}
        }
    }
    pub fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        if !self.visible {
            return;
        }
        let attr = if self.edit_mode {
            theme.searchbar.focused
        } else {
            theme.searchbar.normal
        };
        surface.fill_horizontal_line_with_size(self.x, self.y, self.width as u32, Character::with_attributes(' ', attr));
        surface.write_string(self.x + 1, self.y, &self.text[self.text_offset as usize..], attr, false);
        if (self.width >= SearchBar::DRAW_COUNT_MIN_WIDTH) && (self.match_count != u8::MAX) {
            self.paint_count(surface, theme.searchbar.count);
        }
        if self.edit_mode {
            surface.set_cursor(self.x + 1 + self.cursor_offset as i32, self.y);
        }
    }
    pub fn on_mouse_event(&mut self, event: &MouseEvent) -> ProcessEventResult {
        ProcessEventResult::PassToControl
    }
    pub fn process_key_pressed(&mut self, key: Key, character: char) -> bool {
        match key.value() {
            key!("Escape") => {
                if self.edit_mode {
                    self.edit_mode = false;
                    return true;
                }
            }
            key!("Backspace") => {
                if self.text.len() > 0 {
                    self.text.pop();
                    self.update_text_offset();
                }
                self.edit_mode = true;
                return true;
            }
            _ => {}
        }
        if ((character as u32) > 0) && (self.text.len() < 2048) {
            self.text.push(character);
            self.update_text_offset();
            self.edit_mode = true;
            return true;
        }
        false
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}
