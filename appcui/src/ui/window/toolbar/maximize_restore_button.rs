use crate::{
    graphics::{Character, Surface, SpecialChar},
    system::{Handle, Theme}
};

use super::{AddToToolbar, PaintData, ItemBase, ToolBarItem, Gravity, SymbolAttrState};

pub(crate) struct MaximizeRestoreButton {
    pub(super) base: ItemBase,
    pub(super) handle: Handle<MaximizeRestoreButton>,
}

impl AddToToolbar<MaximizeRestoreButton> for MaximizeRestoreButton {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle<MaximizeRestoreButton> {
        toolbar.items.add(ToolBarItem::MaximizeRestoreButton(self)).cast()
    }
}

impl MaximizeRestoreButton {
    pub fn new() -> Self {
        Self {
            base: ItemBase::with_width(Gravity::TopLeft, 3, "Press to maximize or restore", true),
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
        let ch = match data.maximized {
            true => SpecialChar::ArrowUpDown,
            false => SpecialChar::ArrowUp,
        };
        surface.write_char(
            self.base.get_x() + 1,
            self.base.get_y(),
            Character::with_attributes(ch, st.get_attr(theme, theme.symbol.maximized)),
        );
    }
}
