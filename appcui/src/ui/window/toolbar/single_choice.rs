use std::ptr::NonNull;

use crate::{
    graphics::{Surface, TextAlignament, TextFormat},
    system::{Handle, Theme},
    utils::Caption
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
            caption: Caption::new("", false),
            selected: false,  
            tooldbar: None          
        };
        obj.set_caption(text);
        obj
    }
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, true);
        self.base.set_width(self.caption.get_chars_count() as u16);
        self.base.request_recompute_layout();
    }
    #[inline(always)]
    pub fn get_content(&self) -> &str {
        &self.caption.get_text()
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
    add_toolbaritem_basic_methods!();
}
