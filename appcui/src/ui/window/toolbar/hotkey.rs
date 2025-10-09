use crate::{
    graphics::Surface,
    input::{Key, KeyCode},
    prelude::Character,
    system::{Handle, Theme},
};

use super::super::Type;
use super::{AddToToolbar, Group, ItemBase, PaintData, ToolBarItem};
pub(crate) struct HotKey {
    pub(super) base: ItemBase,
    key: Key,
}

add_to_toolbar_impl!(HotKey);

impl HotKey {
    pub fn new(window_type: Type) -> Self {
        let base = match window_type {
            Type::Normal => ItemBase::new(window_type, false),
            Type::Round => ItemBase::new(window_type, false),
            Type::Panel => ItemBase::with_width(2, "", window_type, false),
        };
        Self {
            base,
            key: Key::None,
        }
    }
    pub fn set_key(&mut self, key: Key) {
        self.key = key;
        if key == Key::None {
            self.base.set_visible(false);
            self.base.request_recompute_layout();
        } else {
            self.base.set_width(key.code.name().chars().count() as u16);
            self.base.set_visible(true);
            self.base.request_recompute_layout();
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let x = self.base.left();
        let y = self.base.y();
        match self.base.window_type() {
            Type::Normal | Type::Round => {
                let attr = match data.focused {
                    true => theme.text.normal,
                    false => theme.text.inactive,
                };
                surface.write_string(x, y, self.key.code.name(), attr, false);
            }
            Type::Panel => {
                let attr = match data.focused {
                    true => theme.text.normal,
                    false => theme.text.inactive,
                };
                let idx = self.key.code as u8;
                const IDX_0: u8 = KeyCode::N0 as u8;
                const IDX_1: u8 = KeyCode::N1 as u8;
                const IDX_9: u8 = KeyCode::N9 as u8;
                const IDX_A: u8 = KeyCode::A as u8;
                const IDX_Z: u8 = KeyCode::Z as u8;
                let unicode = match idx {
                    IDX_1..=IDX_9 => 0x2460 + (idx - IDX_1) as u32,
                    IDX_0 => 0x24EA,
                    IDX_A..=IDX_Z => 0x24B6 + (idx - IDX_A) as u32,
                    _ => '?' as u32,
                };
                let ch = Character::with_attributes(unsafe { char::from_u32_unchecked(unicode) }, attr);
                surface.write_char(x, y, ch);
            }
        }
    }
}
