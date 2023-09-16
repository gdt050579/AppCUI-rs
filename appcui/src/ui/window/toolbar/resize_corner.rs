use crate::{
    graphics::{Character, SpecialChar, Surface},
    system::{Handle, Theme}
};

use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub(crate) struct ResizeCorner {
    pub(super) base: ItemBase,
}

impl AddToToolbar<ResizeCorner> for ResizeCorner {
    fn add(mut self, toolbar: &mut super::toolbar::ToolBar,  group: Group) -> Handle<ResizeCorner> {
        self.base.update_group(group);
        self.base.set_window_handle(toolbar.get_window_handle());
        toolbar.items.add(ToolBarItem::ResizeCorner(self)).cast()
    }
}

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
            surface.write_char(
                x,
                y,
                Character::with_attributes(SpecialChar::BoxHorizontalSingleLine, a),
            );
            surface.write_char(
                x + 1,
                y,
                Character::with_attributes(SpecialChar::BoxBottomRightCornerSingleLine, a),
            );
        }
    }
}
