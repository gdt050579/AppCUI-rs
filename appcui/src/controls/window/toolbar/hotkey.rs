use crate::{
    graphics::{Character, Surface},
    input::Key,
    system::{Handle, Theme},
};

use super::{AddToToolbar, PaintData, Position, ToolBarItem};

pub(crate) struct HotKey {
    pub(super) position: Position,
    pub(super) handle: Handle,
    key: Key,
}

impl AddToToolbar for HotKey {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle {
        toolbar.items.add(ToolBarItem::HotKey(self))
    }
}

impl HotKey {
    pub fn new(key: Key) -> Self {
        HotKey {
            position: todo!(),
            handle: Handle::None,
            key,
        };
    }
    pub fn set_key(&mut self, key: Key) {
        self.key = key;
        self.position.set_width(key.code.get_name().chars().count() as u16);
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        surface.write_char(
            self.position.get_x(),
            self.position.get_y(),
            Character::with_attributes('[', data.sep_attr),
        );
        let attr = match data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(
            self.position.get_x() + 1,
            self.position.get_y(),
            self.key.code.get_name(),
            attr,
            false,
        );
        surface.write_char(
            self.position.get_x() + self.position.get_width() - 1,
            self.position.get_y(),
            Character::with_attributes(']', data.sep_attr),
        );
    }
}
