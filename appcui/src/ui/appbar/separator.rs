use super::{ItemBase, Side};
use crate::graphics::*;
use crate::system::Theme;

pub struct Separator {
    pub(super) base: ItemBase,
}

impl Separator {
    pub fn new(order: u8, pos: Side) -> Self {
        Self {
            base: ItemBase::new(1, order, pos, false),
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.write_char(self.base.x(), 0, Character::with_attributes('|', theme.menu.text.inactive));
    }
}
