use crate::{
    graphics::{Character, SpecialChar, Surface},
    system::{Handle, Theme},
};

use super::super::Type;
use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub(crate) struct ResizeCorner {
    pub(super) base: ItemBase,
    wtype: Type,
}

add_to_toolbar_impl!(ResizeCorner);

impl ResizeCorner {
    pub fn new(window_type: Type) -> Self {
        let w = match window_type {
            Type::Normal => 2,
            Type::Round => 2,
        };
        Self {
            base: ItemBase::with_width(w, "Drag to resize this window", true),
            wtype: window_type,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        if data.focused {
            let st = SymbolAttrState::new(data);
            let x = self.base.get_left();
            let y = self.base.get_y();
            let a = st.get_attr(theme, theme.symbol.resize);
            match self.wtype {
                Type::Normal => {
                    surface.write_char(x, y, Character::with_attributes(SpecialChar::BoxTopLeftCornerSingleLine, a));
                    surface.write_char(x + 1, y, Character::with_attributes(SpecialChar::BoxHorizontalSingleLine, a));
                }
                Type::Round => {
                    surface.write_char(x, y, Character::with_attributes('\u{256F}', a));
                    surface.write_char(x + 1, y, Character::with_attributes(SpecialChar::BoxHorizontalSingleLine, a));
                }
            }
        }
    }
}
