use crate::{
    graphics::Surface,
    system::{Handle, Theme}
};

use super::{AddToToolbar, PaintData, ItemBase, ToolBarItem, Group};

pub struct Label {
    pub(super) base: ItemBase,
    text: String
}

impl AddToToolbar<Label> for Label {
    fn add(mut self, toolbar: &mut super::toolbar::ToolBar,  group: Group) -> Handle<Label> {
        self.base.update_group(group);
        self.base.set_window_handle(toolbar.get_window_handle());
        toolbar.items.add(ToolBarItem::Label(self)).cast()
    }
}

impl Label {
    pub fn new(text: &str)->Self {
        let mut obj = Label {
            base: ItemBase::new(true),
            text: String::new(),
        };
        obj.set_text(text);
        obj
    }
    pub fn set_text(&mut self,text: &str) {
        self.text.clear();
        self.text.push_str(text);
        self.base.set_width(text.chars().count() as u16);
        self.base.request_recompute_layout();
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let attr = match data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(self.base.get_left(), self.base.get_y(), self.text.as_str(), attr, false);   
    }

}
