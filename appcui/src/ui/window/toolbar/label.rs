use crate::{
    graphics::Surface,
    system::{Handle, Theme},
};

use super::{AddToToolbar, PaintData, ItemBase, ToolBarItem, Gravity};

pub struct Label {
    pub(super) base: ItemBase,
    pub(super) handle: Handle,
    text: String
}

impl AddToToolbar for Label {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle {
        toolbar.items.add(ToolBarItem::Label(self))
    }
}

impl Label {
    pub fn new(gravity: Gravity, text: &str)->Self {
        let mut obj = Label {
            base: ItemBase::new(gravity, true, true),
            handle: Handle::None,
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
        surface.write_string(self.base.get_x(), self.base.get_y(), self.text.as_str(), attr, false);   
    }

}
