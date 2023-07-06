use crate::{
    graphics::{Character, Surface},
    system::{Handle, Theme},
};

use super::{AddToToolbar, PaintData, ItemBase, ToolBarItem, Gravity, SymbolAttrState};

pub(crate) struct CloseButton {
    pub(super) base: ItemBase,
    pub(super) handle: Handle,
}

impl AddToToolbar for CloseButton {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle {
        toolbar.items.add(ToolBarItem::CloseButton(self))
    }
}

impl CloseButton {
    pub fn new() -> Self {
        CloseButton {
            base: ItemBase::with_width(Gravity::TopRight, 3, "Press to close this window"),
            handle: Handle::None,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        surface.write_string(
            self.base.get_x(),
            self.base.get_y(),
            "[ ]",
            st.get_attr(theme, data.sep_attr),
            false,
        );
        surface.write_char(
            self.base.get_x() + 1,
            self.base.get_y(),
            Character::with_attributes('x', st.get_attr(theme, theme.symbol.close)),
        );
    }
}
