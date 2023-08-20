use crate::{
    graphics::{Character, SpecialChar, Surface},
    system::{Handle, Theme}
};

use super::{AddToToolbar, Gravity, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub(crate) struct ResizeCorner {
    pub(super) base: ItemBase,
    pub(super) handle: Handle<ResizeCorner>,
}

impl AddToToolbar<ResizeCorner> for ResizeCorner {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle<ResizeCorner> {
        toolbar.items.add(ToolBarItem::ResizeCorner(self)).cast()
    }
}

impl ResizeCorner {
    pub fn new() -> Self {
        Self {
            base: ItemBase::with_width(Gravity::BottomRight, 2, "Drag to resize this window", true),
            handle: Handle::None,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        if data.focused {
            let st = SymbolAttrState::new(data);
            let x = self.base.get_x();
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
