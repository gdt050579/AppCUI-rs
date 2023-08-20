use crate::{ui::menu::Menu, utils::HandleManager};

use super::Handle;

pub(crate) struct MenuHandleManager {
    manager: HandleManager<Menu>
}
impl MenuHandleManager {
    pub(crate) fn new() -> MenuHandleManager {
        Self {
            manager: HandleManager::new(8),
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
        Handle::None
        // MenuHandle { handle: self.manager.add(menu) }
        // let h = MenuHandle::new(self.items.len() as u32);
        // menu.set_handle(h);
        // menu.update_children_with_parent_handle();
        // self.items.push(Some(menu));
        // return h;
    }
}
