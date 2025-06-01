use crate::{
    graphics::{Character, SpecialChar, Surface},
    system::{Handle, Theme},
};

use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub(crate) struct ResizeCorner {
    pub(super) base: ItemBase,
}

add_to_toolbar_impl!(ResizeCorner);

impl ResizeCorner {
    pub fn new() -> Self {
        Self {
            base: ItemBase::with_width(2, "Drag to resize this window", true),
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        if data.focused {
            let st = SymbolAttrState::new(data);
            let x = self.base.get_left();
            let y = self.base.get_y();
            let a = st.get_attr(theme, theme.symbol.resize);
            surface.write_char(x, y, Character::with_attributes(SpecialChar::BoxHorizontalSingleLine, a));
            surface.write_char(x + 1, y, Character::with_attributes(SpecialChar::BoxBottomRightCornerSingleLine, a));
        }
    }
}
