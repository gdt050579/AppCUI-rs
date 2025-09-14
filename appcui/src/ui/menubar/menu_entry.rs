use super::{ItemBase, MenuBarItem, MenuBarItemWrapper};
use crate::system::Handle;
use crate::ui::menu::Menu;

pub struct MenuEntry {
    handle: Handle<Menu>,
    base: ItemBase,
}

impl MenuEntry {
    pub fn new(menu: Menu) -> Self {
        todo!()
    }
    pub fn with_handle(handle: Handle<Menu>) -> Self {
        todo!()
    }
}

impl MenuBarItem for MenuEntry {
    fn into_menuibartem(self) -> MenuBarItemWrapper {
        todo!()
    }
}
