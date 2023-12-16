use crate::{
    graphics::Surface,
    system::{Handle, Theme},
};

use super::{AddToToolbar, Group, ItemBase, PaintData, ToolBarItem};

pub struct Label {
    pub(super) base: ItemBase,
    text: String,
}

add_to_toolbar_impl!(Label);

impl Label {
    pub fn new(text: &str) -> Self {
        let mut obj = Label {
            base: ItemBase::new(true),
            text: String::new(),
        };
        obj.set_content(text);
        obj
    }
    #[inline]
    pub fn set_content(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
        self.base.set_width(text.chars().count() as u16);
        self.base.request_recompute_layout();
    }
    #[inline(always)]
    pub fn get_content(&self) -> &str {
        &self.text
    }
    #[inline(always)]
    pub fn get_tooltip(&self)->&str {
        self.base.get_tooltip()
    }
    #[inline(always)]
    pub fn set_tooltip(&mut self, text: &str){
        self.base.set_tooltip(text);
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let attr = match data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(self.base.get_left(), self.base.get_y(), self.text.as_str(), attr, false);
    }
}

