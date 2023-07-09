use crate::{
    graphics::{Surface, Character},
    system::{Handle, Theme},
};

use super::{AddToToolbar, Gravity, ItemBase, PaintData, ToolBarItem};

pub struct Tag {
    pub(super) base: ItemBase,
    pub(super) handle: Handle,
    text: String,
}

impl AddToToolbar for Tag {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle {
        toolbar.items.add(ToolBarItem::Tag(self))
    }
}

impl Tag {
    pub fn new(gravity: Gravity) -> Self {
        Tag {
            base: ItemBase::new(gravity, false, false),
            handle: Handle::None,
            text: String::new(),
        }
    }
    pub fn set_text(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
        self.base.set_width((text.chars().count() + 2) as u16);
        self.base.set_visible(text.len()>0);
        self.base.request_recompute_layout();
    }
    #[inline(always)]
    pub fn get_text(&self) -> &str {
        &self.text
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        surface.write_char(
            self.base.get_x(),
            self.base.get_y(),
            Character::with_attributes('[', data.sep_attr),
        );
        let attr = match data.focused {
            true => theme.text.enphasized_2,
            false => theme.text.inactive,
        };
        surface.write_string(
            self.base.get_x() + 1,
            self.base.get_y(),
            self.text.as_str(),
            attr,
            false,
        );
        surface.write_char(
            self.base.get_x() + self.base.get_width() - 1,
            self.base.get_y(),
            Character::with_attributes(']', data.sep_attr),
        );
    }
}
