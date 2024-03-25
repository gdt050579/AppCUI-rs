use crate::{
    graphics::{Surface, TextAlignament, TextFormat},
    system::{Handle, Theme},
    utils::Caption, 
    utils::ExtractHotKeyMethod
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
            caption: Caption::new("", ExtractHotKeyMethod::NoHotKey),
        };
        obj.set_caption(text);
        obj
    }
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::AltPlusKey);
        self.base.set_width(self.caption.chars_count() as u16);
        self.base.request_recompute_layout();
    }
    #[inline(always)]
    pub fn get_content(&self) -> &str {
        self.caption.text()
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        let mut format = TextFormat::single_line(
            self.base.get_left(),
            self.base.get_y(),
            st.get_button_attr(theme),
            TextAlignament::Left,
        );
        format.width = Some(self.caption.chars_count() as u16);
        format.hotkey_pos = self.caption.hotkey_pos();
        if self.caption.has_hotkey() {
            format.hotkey_attr = Some(st.get_hotkey_attr(theme));
        }
        surface.write_text(self.caption.text(), &format);
    }
    add_toolbaritem_basic_methods!();
}
