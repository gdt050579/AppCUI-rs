use crate::graphics::*;
use crate::prelude::{ControlBase, MouseEvent};
use crate::system::*;
use crate::ui::common::*;

use self::traits::EventProcessStatus;

use super::{Component, ProcessEventResult};

pub struct SearchBar {
    x: i32,
    y: i32,
    width: u16,
    text: String,
    visible: bool,
    pub(super) handle: Handle<UIElement>,
}
impl SearchBar {
    pub fn new(width: u16) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            text: String::new(),
            handle: Handle::None,
            visible: false,
        }
    }
    #[inline(always)]
    pub(super) fn recompute_position(&mut self, pos: i32, available_size: i32, control_size: Size) -> i32 {
        if pos + self.width as i32 + 1 >= available_size {
            self.visible = false;
            return 0;
        }
        self.x = pos;
        self.y = control_size.height as i32;
        self.visible = true;
        self.width as i32
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
}
impl Component for SearchBar {
    #[allow(private_interfaces)]
    fn into_toolbar(self) -> super::component_toolbar_item::ComponentToolbarItem {
        super::component_toolbar_item::ComponentToolbarItem::SearchBar(self)
    }
}
