use crate::system::Handle;

use super::MenuHandle;

#[derive(Copy,Clone)]
pub struct MenuCommandEvent {
    pub command_id: u32,
    pub menu: MenuHandle,
    pub(crate) control_receiver_handle: Handle,
    // GDT I should also add a menu item handle or index
}

#[derive(Copy,Clone)]
pub struct MenuCheckBoxStateChangedEvent {
    pub command_id: u32,
    pub menu: MenuHandle,
    pub checked: bool,
    pub(crate) control_receiver_handle: Handle,
    // GDT I should also add a menu item handle or index
}

#[derive(Copy,Clone)]
pub struct MenuRadioBoxSelectedEvent {
    pub command_id: u32,
    pub menu: MenuHandle,
    pub(crate) control_receiver_handle: Handle,
    // GDT I should also add a menu item handle or index
}