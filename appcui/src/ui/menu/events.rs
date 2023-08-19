use super::{MenuBar, Menu};

pub trait MenuEvents {
    fn on_menu_open(&self, _menu: &mut Menu) {}
    fn on_item_clicked(&mut self, _command: u32) {}
    fn on_update_menubar(&self, _menubar: &mut MenuBar) {}
}

use crate::{system::Handle, ui::common::UIElement};

use super::MenuHandle;

#[derive(Copy,Clone)]
pub struct MenuCommandEvent {
    pub command_id: u32,
    pub menu: MenuHandle,
    pub(crate) control_receiver_handle: Handle<UIElement>,
    // GDT I should also add a menu item handle or index
}

#[derive(Copy,Clone)]
pub struct MenuCheckBoxStateChangedEvent {
    pub command_id: u32,
    pub menu: MenuHandle,
    pub checked: bool,
    pub(crate) control_receiver_handle: Handle<UIElement>,
    // GDT I should also add a menu item handle or index
}

#[derive(Copy,Clone)]
pub struct MenuRadioBoxSelectedEvent {
    pub command_id: u32,
    pub menu: MenuHandle,
    pub(crate) control_receiver_handle: Handle<UIElement>,
    // GDT I should also add a menu item handle or index
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MenuEvent {
    Command(MenuCommandEvent),
    CheckBoxStateChanged(MenuCheckBoxStateChangedEvent),
    RadioBoxSelected(MenuRadioBoxSelectedEvent),
}