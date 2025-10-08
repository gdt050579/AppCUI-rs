use crate::{
    graphics::{Character, Surface},
    system::{Handle, Theme},
};

use super::super::Type;
use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub(crate) struct CloseButton {
    pub(super) base: ItemBase,
    wtype: Type,
}

add_to_toolbar_impl!(CloseButton);

impl CloseButton {
    pub fn new(window_type: Type) -> Self {
        let w = match window_type {
            Type::Normal => 3,
            Type::Round => 3,
            Type::Panel => 2,
        };
        CloseButton {
            base: ItemBase::with_width(w, "Press to close this window", true),
            wtype: window_type,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        let x = self.base.get_left();
        let y = self.base.get_y();
        match self.wtype {
            Type::Normal => {
                surface.write_string(x, y, "[ ]", st.get_attr(theme, data.sep_attr), false);
                surface.write_char(x + 1, y, Character::with_attributes('x', st.get_attr(theme, theme.symbol.close)));
            }
            Type::Round => {
                surface.write_string(x, y, "[ ]", st.get_attr(theme, data.sep_attr), false);
                surface.write_char(x + 1, y, Character::with_attributes('x', st.get_attr(theme, theme.symbol.close)));
            }
            Type::Panel => {
                surface.write_string(x, y, "✖ ", st.get_attr(theme, theme.symbol.close), false);
            }
        }
    }
}
// ✕  ✖ ❌ ⨉ x  ⦅  ⦆ ⦗ ⦘  ⨷  ⮿  ⸨         【 】  （ ）  𐙝