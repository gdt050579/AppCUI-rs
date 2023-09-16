use crate::{
    graphics::{Surface, TextAlignament, TextFormat},
    system::{Handle, Theme},
    utils::Caption, 
};

use super::{AddToToolbar, ItemBase, PaintData, SymbolAttrState, ToolBarItem, Group};

pub struct Button {
    pub(super) base: ItemBase,
    pub(super) caption: Caption,
}
add_to_toolbar_impl!(Button);

impl Button {
    pub fn new(text: &str) -> Self {
        let mut obj = Button {
            base: ItemBase::new(true),
            caption: Caption::new("", false),
        };
        obj.set_text(text);
        obj
    }
    pub fn set_text(&mut self, text: &str) {
        self.caption.set_text(text, true);
        self.base.set_width(self.caption.get_chars_count() as u16);
        self.base.request_recompute_layout();
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        let mut format = TextFormat::single_line(
            self.base.get_left(),
            self.base.get_y(),
            st.get_button_attr(theme),
            TextAlignament::Left,
        );
        format.width = Some(self.caption.get_chars_count() as u16);
        format.hotkey_pos = self.caption.get_hotkey_pos();
        if self.caption.has_hotkey() {
            format.hotkey_attr = Some(st.get_hotkey_attr(theme));
        }
        surface.write_text(self.caption.get_text(), &format);
    }
}
