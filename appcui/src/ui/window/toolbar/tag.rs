use crate::{
    graphics::Surface,
    system::{Handle, Theme},
};

use super::{AddToToolbar, Group, ItemBase, PaintData, ToolBarItem};

pub struct Tag {
    pub(super) base: ItemBase,
    text: String,
}

impl AddToToolbar<Tag> for Tag {
    fn add(mut self, toolbar: &mut super::toolbar::ToolBar, group: Group) -> Handle<Tag> {
        self.base.update_group(group);
        toolbar.items.add(ToolBarItem::Tag(self)).cast()
    }
}

impl Tag {
    pub fn new() -> Self {
        Tag {
            base: ItemBase::new(false),
            text: String::new(),
        }
    }
    pub fn set_text(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
        self.base.set_width(text.chars().count() as u16);
        self.base.set_visible(text.len() > 0);
        self.base.request_recompute_layout();
    }
    #[inline(always)]
    pub fn get_text(&self) -> &str {
        &self.text
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let attr = match data.focused {
            true => theme.text.enphasized_2,
            false => theme.text.inactive,
        };
        surface.write_string(self.base.get_left(), self.base.get_y(), self.text.as_str(), attr, false);
    }
}
