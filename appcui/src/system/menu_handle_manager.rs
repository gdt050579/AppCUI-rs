use crate::controls::menu::{Menu, MenuHandle};

pub(crate) struct MenuHandleManager {
    items: Vec<Option<Menu>>,
}
impl MenuHandleManager {
    pub(crate) fn new() -> MenuHandleManager {
        Self {
            items: Vec::with_capacity(64),
        }
    }
    pub(crate) fn get_mut(&mut self, handle: MenuHandle) -> Option<&mut Menu> {
        let idx = handle.get_index();
        if idx < self.items.len() {
            let m = self.items[idx].as_mut();
            if m.is_some() {
                if m.as_ref().unwrap().get_handle() == handle {
                    return m;
                }
            }
        }
        None
    }
    pub(crate) fn get(&mut self, handle: MenuHandle) -> Option<&Menu> {
        let idx = handle.get_index();
        if idx < self.items.len() {
            let m = self.items[idx].as_ref();
            if m.is_some() {
                if m.as_ref().unwrap().get_handle() == handle {
                    return m;
                }
            }
        }
        None
    }
    pub(crate) fn add(&mut self, mut menu: Menu)->MenuHandle {
        let h = MenuHandle::new(self.items.len() as u32);
        menu.set_handle(h);
        menu.update_children_with_parent_handle();
        self.items.push(Some(menu));
        return h;
    }
}