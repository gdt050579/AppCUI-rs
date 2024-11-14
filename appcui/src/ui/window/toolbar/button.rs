use crate::{
    graphics::{Surface, TextAlignament, TextFormatBuilder},
    system::{Handle, Theme},
    utils::{Caption, ExtractHotKeyMethod},
};

use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

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
        let mut format = TextFormatBuilder::new()
            .position(self.base.get_left(), self.base.get_y())
            .attribute(st.get_button_attr(theme))
            .align(TextAlignament::Left)
            .truncate(self.caption.chars_count() as u16)
            .build();
        if self.caption.has_hotkey() {
            format.set_hotkey(st.get_hotkey_attr(theme), self.caption.hotkey_pos().unwrap() as u32);
        }
        surface.write_text_new(self.caption.text(), &format);
    }
    add_toolbaritem_basic_methods!();
}
