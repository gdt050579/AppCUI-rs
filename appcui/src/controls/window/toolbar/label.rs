
use crate::system::Handle;

use super::{Position, AddToToolbar, ToolBarItem};

pub struct Label {
    pub (super) position: Position,
    pub (super) handle: Handle,
}

impl AddToToolbar for Label {
    fn add(self, toolbar: &mut super::toolbar::ToolBar) -> Handle {
        toolbar.items.add(ToolBarItem::Label(self))
    }
}