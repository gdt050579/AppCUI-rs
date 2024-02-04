use super::{MenuBar, Menu, CheckBox, Command, SingleChoice};

pub trait GenericMenuEvents {
    fn on_menu_open(&self, _menu: &mut Menu) {}
    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<Command>, command: u32) {}
    fn on_check(&mut self, menu: Handle<Menu>, item: Handle<CheckBox>, command: u32, checked: bool) {}
    fn on_select(&mut self, menu: Handle<Menu>, item: Handle<SingleChoice>, command: u32) {}
    fn on_update_menubar(&self, _menubar: &mut MenuBar) {}
}

use crate::{system::Handle, ui::common::UIElement};

#[derive(Copy,Clone)]
pub(crate) struct MenuCommandEvent {
    pub(crate) command_id: u32,
    pub(crate) menu: Handle<Menu>,
    pub(crate) item: Handle<Command>,
    pub(crate) control_receiver_handle: Handle<UIElement>,
}

#[derive(Copy,Clone)]
pub(crate) struct MenuCheckBoxStateChangedEvent {
    pub(crate) command_id: u32,
    pub(crate) menu: Handle<Menu>,
    pub(crate) item: Handle<CheckBox>,
    pub(crate) checked: bool,
    pub(crate) control_receiver_handle: Handle<UIElement>,
}

#[derive(Copy,Clone)]
pub(crate) struct MenuRadioBoxSelectedEvent {
    pub(crate) command_id: u32,
    pub(crate) menu: Handle<Menu>,
    pub(crate) item: Handle<SingleChoice>,
    pub(crate) control_receiver_handle: Handle<UIElement>,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum MenuEvent {
    Command(MenuCommandEvent),
    CheckBoxStateChanged(MenuCheckBoxStateChangedEvent),
    SingleChoiceSelected(MenuRadioBoxSelectedEvent),
}