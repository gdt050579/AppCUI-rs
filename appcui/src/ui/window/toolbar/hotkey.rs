use crate::{
    graphics::Surface,
    input::Key,
    system::{Handle, Theme},
};

use super::{AddToToolbar, Group, ItemBase, PaintData, ToolBarItem};

pub(crate) struct HotKey {
    pub(super) base: ItemBase,
    key: Key,
}

crate::add_to_toolbar_impl!(HotKey);

impl HotKey {
    pub fn new() -> Self {
        HotKey {
            base: ItemBase::new(false),
            key: Key::None,
        }
    }
    pub fn set_key(&mut self, key: Key) {
        self.key = key;
        if key == Key::None {
            self.base.set_visible(false);
            self.base.request_recompute_layout();
        } else {
            self.base.set_width(key.code.get_name().chars().count() as u16);
            self.base.set_visible(true);
            self.base.request_recompute_layout();
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let attr = match data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(self.base.get_left(), self.base.get_y(), self.key.code.get_name(), attr, false);
    }
}
