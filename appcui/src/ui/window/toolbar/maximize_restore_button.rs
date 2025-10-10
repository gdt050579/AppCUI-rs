use crate::{
    graphics::{Character, SpecialChar, Surface},
    system::{Handle, Theme},
};

use super::super::Type;
use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub(crate) struct MaximizeRestoreButton {
    pub(super) base: ItemBase,
}

add_to_toolbar_impl!(MaximizeRestoreButton);

impl MaximizeRestoreButton {
    pub fn new(window_type: Type) -> Self {
        let w = match window_type {
            Type::Normal => 3,
            Type::Round => 3,
            Type::Panel => 2,
        };
        Self {
            base: ItemBase::with_width(w, "Press to maximize or restore",window_type, true),
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        let x = self.base.left();
        let y = self.base.y();

        match self.base.window_type() {
            Type::Normal | Type::Round => {
                surface.write_string(x, y, "[ ]", st.attr(theme, data.sep_attr), false);
                let ch = match data.maximized {
                    true => SpecialChar::ArrowUpDown,
                    false => SpecialChar::ArrowUp,
                };
                surface.write_char(x+1, y, Character::with_attributes(ch, st.attr(theme, theme.symbol.maximized)));
            }

            Type::Panel => {
                let s = match data.maximized {
                    true => "🗗 ",
                    false => "🗖 ",
                };
                surface.write_string(x, y, s, st.window_bar_attr(theme, theme.window.bar.maximize_button), false);
            }
        }
    }
}
