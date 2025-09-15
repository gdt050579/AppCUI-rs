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
pub(crate) trait AppBarItem {
    fn into_appbaribartem(self) -> AppBarItemWrapper;
}
pub(crate) enum AppBarItemWrapper {
    Separator(bool),
    MenuEntry(MenuButton),
    Label(bool),
    Button(bool),
    CheckBox(bool),
}

impl AppBarItemWrapper {
    pub(super) fn base(&self) -> &ItemBase {
        match self {
            AppBarItemWrapper::Separator(_) => todo!(),
            AppBarItemWrapper::MenuEntry(obj) => &obj.base,
            AppBarItemWrapper::Label(_) => todo!(),
            AppBarItemWrapper::Button(_) => todo!(),
            AppBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn base_mut(&mut self) -> &mut ItemBase {
        match self {
            AppBarItemWrapper::Separator(_) => todo!(),
            AppBarItemWrapper::MenuEntry(obj) => &mut obj.base,
            AppBarItemWrapper::Label(_) => todo!(),
            AppBarItemWrapper::Button(_) => todo!(),
            AppBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(super) fn is_enabled(&self) -> bool {
        self.base().is_enabled()
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Key {
        match self {
            AppBarItemWrapper::Separator(_) => todo!(),
            AppBarItemWrapper::MenuEntry(menu_entry) => menu_entry.hotkey(),
            AppBarItemWrapper::Label(_) => todo!(),
            AppBarItemWrapper::Button(_) => todo!(),
            AppBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(super) fn process_shortcut(&self, key: Key, menus: &mut MenuHandleManager) -> bool {
        match self {
            AppBarItemWrapper::Separator(_) => todo!(),
            AppBarItemWrapper::MenuEntry(menu_entry) => menu_entry.process_shortcut(key, menus),
            AppBarItemWrapper::Label(_) => todo!(),
            AppBarItemWrapper::Button(_) => todo!(),
            AppBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn activate(&mut self) {
        match self {
            AppBarItemWrapper::Separator(_) => todo!(),
            AppBarItemWrapper::MenuEntry(obj) => obj.on_activate(),
            AppBarItemWrapper::Label(_) => todo!(),
            AppBarItemWrapper::Button(_) => todo!(),
            AppBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ItemStatus) {
        match self {
            AppBarItemWrapper::Separator(_) => todo!(),
            AppBarItemWrapper::MenuEntry(obj) => obj.paint(surface, theme, status),
            AppBarItemWrapper::Label(_) => todo!(),
            AppBarItemWrapper::Button(_) => todo!(),
            AppBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        match self {
            AppBarItemWrapper::Separator(_) => todo!(),
            AppBarItemWrapper::MenuEntry(obj) => obj.set_receiver_control_handle(handle),
            AppBarItemWrapper::Label(_) => todo!(),
            AppBarItemWrapper::Button(_) => todo!(),
            AppBarItemWrapper::CheckBox(_) => todo!(),
        }
    }
}

impl HandleSupport<AppBarItemWrapper> for AppBarItemWrapper {
    fn handle(&self) -> Handle<AppBarItemWrapper> {
        self.base().handle()
    }

    fn set_handle(&mut self, handle: Handle<AppBarItemWrapper>) {
        self.base_mut().update_handle(handle);
    }
}
