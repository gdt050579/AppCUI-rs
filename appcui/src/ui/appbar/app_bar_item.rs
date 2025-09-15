use super::ItemBase;
use super::MenuButton;
use crate::graphics::Surface;
use crate::input::*;
use crate::system::MenuHandleManager;
use crate::{
    system::{Handle, HandleSupport, Theme},
    ui::appbar::ItemStatus,
};

// un menu bar itm are:
// flag-urile: enabld / visible / to left / capture input
// obiectul in sine (poate fi nimic la separator, un handle la un menu in MenuEntry, tc)
pub(crate) trait MenuBarItem {
    fn into_menuibartem(self) -> MenuBarItemWrapper;
}
pub(crate) enum MenuBarItemWrapper {
    Separator(bool),
    MenuEntry(MenuButton),
    Label(bool),
    Button(bool),
    CheckBox(bool),
}

impl MenuBarItemWrapper {
    pub(super) fn base(&self) -> &ItemBase {
        match self {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(obj) => &obj.base,
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn base_mut(&mut self) -> &mut ItemBase {
        match self {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(obj) => &mut obj.base,
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(super) fn is_enabled(&self) -> bool {
        self.base().is_enabled()
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Key {
        match self {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(menu_entry) => menu_entry.hotkey(),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(super) fn process_shortcut(&self, key: Key, menus: &mut MenuHandleManager) -> bool {
        match self {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(menu_entry) => menu_entry.process_shortcut(key, menus),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn activate(&mut self) {
        match self {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(obj) => obj.on_activate(),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ItemStatus) {
        match self {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(obj) => obj.paint(surface, theme, status),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        match self {
            MenuBarItemWrapper::Separator(_) => todo!(),
            MenuBarItemWrapper::MenuEntry(obj) => obj.set_receiver_control_handle(handle),
            MenuBarItemWrapper::Label(_) => todo!(),
            MenuBarItemWrapper::Button(_) => todo!(),
            MenuBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
}

impl HandleSupport<MenuBarItemWrapper> for MenuBarItemWrapper {
    fn handle(&self) -> Handle<MenuBarItemWrapper> {
        self.base().handle()
    }

    fn set_handle(&mut self, handle: Handle<MenuBarItemWrapper>) {
        self.base_mut().update_handle(handle);
    }
}
