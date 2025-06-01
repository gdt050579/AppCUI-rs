use crate::{
    graphics::{Character, SpecialChar, Surface},
    system::{Handle, Theme},
};

use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub(crate) struct MaximizeRestoreButton {
    pub(super) base: ItemBase,
}

add_to_toolbar_impl!(MaximizeRestoreButton);

impl MaximizeRestoreButton {
    pub fn new() -> Self {
        Self {
            base: ItemBase::with_width(3, "Press to maximize or restore", true),
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        surface.write_string(self.base.get_left(), self.base.get_y(), "[ ]", st.get_attr(theme, data.sep_attr), false);
        let ch = match data.maximized {
            true => SpecialChar::ArrowUpDown,
            false => SpecialChar::ArrowUp,
        };
        surface.write_char(
            self.base.get_left() + 1,
            self.base.get_y(),
            Character::with_attributes(ch, st.get_attr(theme, theme.symbol.maximized)),
        );
    }
}
