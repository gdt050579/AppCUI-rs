use crate::{ui::menu::Menu, utils::HandleManager};

use super::Handle;

pub(crate) struct MenuHandleManager {
    manager: HandleManager<Menu>
}
impl MenuHandleManager {
    pub(crate) fn new() -> MenuHandleManager {
        Self {
            manager: HandleManager::with_capacity(8),
        }
    }
    #[inline(always)]
    pub(crate) fn get_mut(&mut self, menu_handle: Handle<Menu>) -> Option<&mut Menu> {
        self.manager.get_mut(menu_handle.cast())
    }
    #[inline(always)]
    pub(crate) fn get(&mut self, menu_handle: Handle<Menu>) -> Option<&Menu> {
        self.manager.get(menu_handle.cast())
    }
    #[inline(always)]
    pub(crate) fn add(&mut self, menu: Menu) -> Handle<Menu> {
        self.manager.add(menu)
    }
}
