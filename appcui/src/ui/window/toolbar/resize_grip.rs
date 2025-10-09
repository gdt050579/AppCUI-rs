use crate::{
    graphics::{Character, SpecialChar, Surface},
    system::{Handle, Theme},
};

use super::super::Type;
use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub(crate) struct ResizeGrip {
    pub(super) base: ItemBase,
}

add_to_toolbar_impl!(ResizeGrip);

impl ResizeGrip {
    pub fn new(window_type: Type) -> Self {
        let w = match window_type {
            Type::Normal => 2,
            Type::Round => 2,
            Type::Panel => 1,
        };
        Self {
            base: ItemBase::with_width(w, "Drag to resize this window", window_type, true),
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        if data.focused {
            let st = SymbolAttrState::new(data);
            let x = self.base.left();
            let y = self.base.y();
            let a = st.get_attr(theme, theme.symbol.resize);
            match self.base.window_type() {
                Type::Normal => {
                    surface.write_char(x + 1, y, Character::with_attributes(SpecialChar::BoxBottomRightCornerSingleLine, a));
                    surface.write_char(x, y, Character::with_attributes(SpecialChar::BoxHorizontalSingleLine, a));
                }
                Type::Round => {
                    surface.write_char(x + 1, y, Character::with_attributes('\u{256F}', a));
                    surface.write_char(x, y, Character::with_attributes(SpecialChar::BoxHorizontalSingleLine, a));
                }
                Type::Panel => {
                    surface.write_char(x, y, Character::with_attributes('⇲', a));
                }
            }
        }
    }
}
