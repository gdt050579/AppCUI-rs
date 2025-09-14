use super::{ItemBase, MenuBarItem, MenuBarItemWrapper, MenuBarPosition};
use crate::system::{Handle, RuntimeManager};
use crate::ui::menu::Menu;

pub struct MenuEntry {
    handle: Handle<Menu>,
    receiver_control_handle: Handle<()>,
    pub(super) base: ItemBase,
}

impl MenuEntry {
    pub fn new(menu: Menu, order: u8, pos: MenuBarPosition) -> Self {
        let w = (menu.caption().chars_count().max(1) + 2).min(u8::MAX as usize) as u8;
        let h = RuntimeManager::get().add_menu(menu);
        Self {
            handle: h,
            receiver_control_handle: Handle::None,
            base: ItemBase::new(w, order, pos, true),
        }
    }
    pub fn with_handle(handle: Handle<Menu>, order: u8, pos: MenuBarPosition) -> Self {
        todo!()
    }
    pub(super) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        self.receiver_control_handle = handle;
    }
}

impl MenuBarItem for MenuEntry {
    fn into_menuibartem(self) -> MenuBarItemWrapper {
        MenuBarItemWrapper::MenuEntry(self)
    }
}
