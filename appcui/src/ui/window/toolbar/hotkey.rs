use crate::{
    graphics::{Character, Surface},
    input::Key,
    system::{Handle, Theme}
};

use super::{AddToToolbar, ItemBase, PaintData, ToolBarItem, Group};

pub(crate) struct HotKey {
    pub(super) base: ItemBase,
    key: Key,
}

impl AddToToolbar<HotKey> for HotKey {
    fn add(mut self, toolbar: &mut super::toolbar::ToolBar,  group: Group) -> Handle<HotKey> {
        self.base.update_group(group);
        toolbar.items.add(ToolBarItem::HotKey(self)).cast()
    }
}

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
