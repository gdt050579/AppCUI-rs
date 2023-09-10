use crate::{
    graphics::{Character, Surface},
    system::{Handle, Theme}
};

use super::{AddToToolbar, PaintData, ItemBase, ToolBarItem, SymbolAttrState, Group};

pub(crate) struct CloseButton {
    pub(super) base: ItemBase,
    pub(super) handle: Handle<CloseButton>,
}

impl AddToToolbar<CloseButton> for CloseButton {
    fn add(mut self, toolbar: &mut super::toolbar::ToolBar,  group: Group) -> Handle<CloseButton> {
        self.base.update_group(group);
        toolbar.items.add(ToolBarItem::CloseButton(self)).cast()
    }
}

impl CloseButton {
    pub fn new() -> Self {
        CloseButton {
            base: ItemBase::with_width(3, "Press to close this window", true),
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
