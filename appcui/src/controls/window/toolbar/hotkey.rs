use crate::{
    graphics::{Character, Surface},
    input::Key,
    system::{Handle, Theme},
};

use super::{AddToToolbar, Gravity, ItemBase, PaintData, ToolBarItem};

pub(crate) struct HotKey {
    pub(super) base: ItemBase,
    pub(super) handle: Handle,
    key: Key,
}

impl AddToToolbar for HotKey {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle {
        toolbar.items.add(ToolBarItem::HotKey(self))
    }
}

impl HotKey {
    pub fn new(gravity: Gravity) -> Self {
        HotKey {
            base: ItemBase::new(gravity, false, false),
            handle: Handle::None,
            key: Key::None,
        }
    }
    pub fn set_key(&mut self, key: Key) {
        self.key = key;
        if key == Key::None {
            self.base.set_visible(false);
        } else {
            self.base
                .set_width(key.code.get_name().chars().count() as u16 + 2);
            self.base.set_visible(true);
            self.base.request_recompute_layout();
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        surface.write_char(
            self.base.get_x(),
            self.base.get_y(),
            Character::with_attributes('[', data.sep_attr),
        );
        let attr = match data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(
            self.base.get_x() + 1,
            self.base.get_y(),
            self.key.code.get_name(),
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
