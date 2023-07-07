use crate::{
    graphics::{Character, Surface, SpecialChar},
    system::{Handle, Theme},
};

use super::{AddToToolbar, PaintData, ItemBase, ToolBarItem, Gravity, SymbolAttrState};

pub(crate) struct ResizeCorner {
    pub(super) base: ItemBase,
    pub(super) handle: Handle,
}

impl AddToToolbar for ResizeCorner {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle {
        toolbar.items.add(ToolBarItem::ResizeCorner(self))
    }
}

impl ResizeCorner {
    pub fn new() -> Self {
        Self {
            base: ItemBase::with_width(Gravity::BottomRight, 1, "Drag to resize this window"),
            handle: Handle::None,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        if data.focused {
            let st = SymbolAttrState::new(data);
            surface.write_char(
                self.base.get_x(),
                self.base.get_y(),
                Character::with_attributes(
                    SpecialChar::BoxBottomRightCornerSingleLine,
                    st.get_attr(theme, theme.symbol.resize),
                ),
            );
        }
    }
}
