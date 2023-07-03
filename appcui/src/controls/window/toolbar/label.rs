use crate::{
    graphics::Surface,
    system::{Handle, Theme},
};

use super::{AddToToolbar, PaintData, Position, ToolBarItem};

pub struct Label {
    pub(super) position: Position,
    pub(super) handle: Handle,
    text: String
}

impl AddToToolbar for Label {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle {
        toolbar.items.add(ToolBarItem::Label(self))
    }
}

impl Label {
    pub fn new(text: &str)->Self {
        let mut obj = Label {
            position: todo!(),
            handle: Handle::None,
            text: String::new(),
        };
        obj.set_text(text);
        obj
    }
    pub fn set_text(&mut self,text: &str) {
        self.text.clear();
        self.text.push_str(text);
        self.position.set_width(text.chars().count() as u16);
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let attr = match data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(self.position.get_x() + 1, self.position.get_y(), self.text.as_str(), attr, false);   
    }

}
