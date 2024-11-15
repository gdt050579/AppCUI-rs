use std::ptr::NonNull;

use crate::{
    graphics::{Surface, TextAlignament, TextFormatBuilder, WrapType},
    system::{Handle, Theme},
    utils::Caption,
    utils::ExtractHotKeyMethod
};

use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem, ToolBar};

pub struct SingleChoice {
    pub(super) base: ItemBase,
    pub(super) caption: Caption,
    selected: bool,
    pub(super) tooldbar: Option<NonNull<ToolBar>>
}

add_to_toolbar_impl!(SingleChoice);

impl SingleChoice {
    pub fn new(text: &str) -> Self {
        let mut obj = SingleChoice {
            base: ItemBase::new(true),
            caption: Caption::new("", ExtractHotKeyMethod::NoHotKey),
            selected: false,  
            tooldbar: None          
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
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    #[inline(always)]
    pub fn select(&mut self) {
        if let Some(toolbar_ptr) = self.tooldbar.as_mut() {
            let toolbar = unsafe { toolbar_ptr.as_mut() };
            toolbar.update_singlechoice_group_id(self.base.get_handle());
        } else {
            panic!("Attempt to use SingleChoice select without having the object added to a toolbar !");
        }
    }

    pub(crate) fn update_select_status(&mut self, value: bool) {
        self.selected = value;
    }

    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let mut st = SymbolAttrState::new(data);
        if (self.selected) && (data.focused) {
            st = SymbolAttrState::Pressed;
        }
        let mut format = TextFormatBuilder::new()
            .position(self.base.get_left(), self.base.get_y())
            .attribute(st.get_button_attr(theme))
            .align(TextAlignament::Left)
            .wrap(WrapType::SingleLineWrap(self.caption.chars_count() as u16))
            .build();
        if self.caption.has_hotkey() {
            format.set_hotkey(st.get_hotkey_attr(theme), self.caption.hotkey_pos().unwrap() as u32);
        }
        surface.write_text(self.caption.text(), &format);
    }
    add_toolbaritem_basic_methods!();
}
