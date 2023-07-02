use crate::{system::Handle, utils::HandleManager};

use super::tool_bar_item::ToolBarItem;

pub struct ToolBar {
    pub (super) items: HandleManager<ToolBarItem>,
}

pub trait AddToToolbar {
    fn add(self, toolbar: &mut ToolBar) -> Handle;
}

impl ToolBar {
    pub(super) fn new() -> Self {
        ToolBar {
            items: HandleManager::new(4),
        }
    }
    pub fn add<T>(&mut self, item: T)
    where
        T: AddToToolbar
    {
        let _h = AddToToolbar::add(item, self);
    }
}
