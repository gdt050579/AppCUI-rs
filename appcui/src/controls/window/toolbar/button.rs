use crate::{
    graphics::{Surface, TextAlignament, TextFormat},
    system::{Handle, Theme},
    utils::Caption,
};

use super::{AddToToolbar, Gravity, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

pub struct Button {
    pub(super) base: ItemBase,
    pub(super) handle: Handle,
    caption: Caption,
    command_id: u32,
}

impl AddToToolbar for Button {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle {
        toolbar.items.add(ToolBarItem::Button(self))
    }
}

impl Button {
    pub fn new(gravity: Gravity, text: &str, command_id: u32) -> Self {
        let mut obj = Button {
            base: ItemBase::new(gravity, true, true),
            handle: Handle::None,
            caption: Caption::new("", false),
            command_id,
        };
        obj.set_text(text);
        obj
    }
    #[inline(always)]
    pub fn get_command_id(&self)->u32 {
        self.command_id
    }
    pub fn set_text(&mut self, text: &str) {
        self.caption.set_text(text, true);
        self.base.set_width(self.caption.get_chars_count() as u16);
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        let mut format = TextFormat::single_line(
            self.base.get_x(),
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
