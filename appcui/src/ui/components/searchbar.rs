use crate::graphics::*;
use crate::prelude::ControlBase;
use crate::system::*;
use crate::ui::common::*;
use crate::input::*;

use super::ProcessEventResult;

pub struct SearchBar {
    x: i32,
    y: i32,
    width: u16,
    text: String,
    visible: bool,
}
impl SearchBar {
    const MIN_WIDTH: u16 = 5;
    const PREFERED_WIDTH: u16 = 14;
    pub fn new(visible: bool) -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0 ,
            text: String::new(),
            visible,
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
        self.x = pos;
        self.y = control_size.height as i32;
        self.visible = true;
        (self.width as i32) + 1
    }
    pub fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        if !self.visible {
            return;
        }
        let attr = theme.searchbar.normal;
        surface.fill_horizontal_line_with_size(self.x, self.y, self.width as u32, Character::with_attributes(' ', attr));
    }
    pub fn on_mouse_event(&mut self, event: &MouseEvent) -> ProcessEventResult {
        ProcessEventResult::PassToControl
    }
    pub fn process_key_pressed(&mut self, key: Key, character: char) -> bool  {
        false
    }
}

