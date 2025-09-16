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

pub(crate) enum AppBarItem {
    Separator(bool),
    MenuButton(MenuButton),
    Label(bool),
    Button(bool),
    CheckBox(bool),
}

impl From<super::MenuButton> for AppBarItem {
    fn from(value: super::MenuButton) -> Self {
        AppBarItem::MenuButton(value)
    }
}

impl AppBarItem {
    pub(super) fn base(&self) -> &ItemBase {
        match self {
            AppBarItem::Separator(_) => todo!(),
            AppBarItem::MenuButton(obj) => &obj.base,
            AppBarItem::Label(_) => todo!(),
            AppBarItem::Button(_) => todo!(),
            AppBarItem::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn base_mut(&mut self) -> &mut ItemBase {
        match self {
            AppBarItem::Separator(_) => todo!(),
            AppBarItem::MenuButton(obj) => &mut obj.base,
            AppBarItem::Label(_) => todo!(),
            AppBarItem::Button(_) => todo!(),
            AppBarItem::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(super) fn is_enabled(&self) -> bool {
        self.base().is_enabled()
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Key {
        match self {
            AppBarItem::Separator(_) => todo!(),
            AppBarItem::MenuButton(menu_entry) => menu_entry.hotkey(),
            AppBarItem::Label(_) => todo!(),
            AppBarItem::Button(_) => todo!(),
            AppBarItem::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(super) fn process_shortcut(&self, key: Key, menus: &mut MenuHandleManager) -> bool {
        match self {
            AppBarItem::Separator(_) => todo!(),
            AppBarItem::MenuButton(menu_entry) => menu_entry.process_shortcut(key, menus),
            AppBarItem::Label(_) => todo!(),
            AppBarItem::Button(_) => todo!(),
            AppBarItem::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn activate(&mut self) {
        match self {
            AppBarItem::Separator(_) => todo!(),
            AppBarItem::MenuButton(obj) => obj.on_activate(),
            AppBarItem::Label(_) => todo!(),
            AppBarItem::Button(_) => todo!(),
            AppBarItem::CheckBox(_) => todo!(),
        }
    }
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ItemStatus) {
        match self {
            AppBarItem::Separator(_) => todo!(),
            AppBarItem::MenuButton(obj) => obj.paint(surface, theme, status),
            AppBarItem::Label(_) => todo!(),
            AppBarItem::Button(_) => todo!(),
            AppBarItem::CheckBox(_) => todo!(),
        }
    }
    pub(super) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        match self {
            AppBarItem::Separator(_) => todo!(),
            AppBarItem::MenuButton(obj) => obj.set_receiver_control_handle(handle),
            AppBarItem::Label(_) => todo!(),
            AppBarItem::Button(_) => todo!(),
            AppBarItem::CheckBox(_) => todo!(),
        }
    }
}

impl HandleSupport<AppBarItem> for AppBarItem {
    fn handle(&self) -> Handle<AppBarItem> {
        self.base().handle()
    }

    fn set_handle(&mut self, handle: Handle<AppBarItem>) {
        self.base_mut().update_handle(handle);
    }
}
