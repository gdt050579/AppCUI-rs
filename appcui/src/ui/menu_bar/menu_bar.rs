use super::{MenuBarItem, MenuBarItemWrapper};
use crate::system::Handle;
use crate::utils::HandleManager;

pub struct MenuBar {
    manager: HandleManager<MenuBarItemWrapper>,
    visibl_indexes: Vec<u32>,
}
impl MenuBar {
    pub(crate) fn new() -> Self {
        Self {
            manager: HandleManager::with_capacity(16),
            visibl_indexes: Vec::with_capacity(64),
        }
    }
    pub fn add<T>(&mut self, item: T) -> Handle<T>
    where
        T: MenuBarItem,
    {
        self.manager.add(item.into_menuibartem()).cast()
    }
    pub fn get<T>(&self, menubaritem_hamdle: Handle<T>) -> Option<&T>
    where
        T: MenuBarItem,
    {
        let ref_item = self.manager.get(menubaritem_hamdle.cast())?;
        match ref_item {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(_) => todo!(),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub fn get_mut<T>(&mut self, menubaritem_hamdle: Handle<T>) -> Option<&mut T>
    where
        T: MenuBarItem,
    {
        let ref_item = self.manager.get_mut(menubaritem_hamdle.cast())?;
        match ref_item {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(_) => todo!(),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub(crate) fn update_positions(&mut self) {
        todo!()
    }
    pub fn show<T>(&mut self, handle: Handle<T>)
    where
        T: MenuBarItem,
    {
    }
}
