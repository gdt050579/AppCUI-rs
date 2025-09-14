use crate::system::{Handle, HandleSupport};
use super::ItemBase;
use crate::graphics::Surface;
use super::MenuEntry;

// un menu bar itm are:
// flag-urile: enabld / visible / to left / capture input
// obiectul in sine (poate fi nimic la separator, un handle la un menu in MenuEntry, tc)
pub(crate) trait MenuBarItem {
    fn into_menuibartem(self) -> MenuBarItemWrapper;
}
pub(crate) enum MenuBarItemWrapper {
    Separator(bool),
    MenuEntry(MenuEntry),
    Label(bool),
    Button(bool),
    CheckBox(bool),
}

impl MenuBarItemWrapper {
    pub(super) fn base(&self) -> &ItemBase {
        match self {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(_) => todo!(),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn base_mut(&mut self) -> &mut ItemBase {
        match self {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(_) => todo!(),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }        
    }
    pub(super) fn paint(&self, surface: &mut Surface) {

    }
}

impl HandleSupport<MenuBarItemWrapper> for MenuBarItemWrapper {
    fn handle(&self) -> Handle<MenuBarItemWrapper> {
        todo!()
    }

    fn set_handle(&mut self, handle: Handle<MenuBarItemWrapper>) {
        todo!()
    }
}
