use super::{CheckBox, Command, Menu, MenuBar, SingleChoice};

/// A trait that defines the event handlers for menu-related interactions.
///
/// This trait provides methods for responding to various menu events such as
/// menu opening, command selection, checkbox state changes, single choice selections,
/// and menu bar updates. Controls that need to handle menu events should implement
/// this trait.
pub trait GenericMenuEvents {
    /// Called when a menu is about to be opened.
    ///
    /// This method allows customization of the menu before it's displayed,
    /// such as enabling/disabling items or dynamically adding new items.
    ///
    /// # Parameters
    /// * `menu` - A mutable reference to the menu being opened.
    fn on_menu_open(&self, _menu: &mut Menu) {}

    /// Called when a command menu item is activated.
    ///
    /// # Parameters
    /// * `menu` - A handle to the menu containing the command.
    /// * `item` - A handle to the command that was activated.
    /// * `command` - The command identifier associated with the menu item.
    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<Command>, _command: u32) {}

    /// Called when a checkbox menu item's state changes.
    ///
    /// # Parameters
    /// * `menu` - A handle to the menu containing the checkbox.
    /// * `item` - A handle to the checkbox that was clicked.
    /// * `command` - The command identifier associated with the menu item.
    /// * `checked` - The new checked state of the checkbox.
    fn on_check(&mut self, _menu: Handle<Menu>, _item: Handle<CheckBox>, _command: u32, _checked: bool) {}

    /// Called when a single choice menu item is selected.
    ///
    /// # Parameters
    /// * `menu` - A handle to the menu containing the single choice item.
    /// * `item` - A handle to the single choice item that was selected.
    /// * `command` - The command identifier associated with the menu item.
    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<SingleChoice>, _command: u32) {}

    /// Called when the menu bar needs to be updated.
    ///
    /// This method is used to add registered menus to the menu bar.
    /// It is called whenever the focus changes or when a control
    /// explicitly requests a menu bar update.
    ///
    /// # Parameters
    /// * `menubar` - A mutable reference to the menu bar to update.
    fn on_update_menubar(&self, _menubar: &mut MenuBar) {}
}

use crate::system::Handle;

#[derive(Copy, Clone)]
pub(crate) struct MenuCommandEvent {
    pub(crate) command_id: u32,
    pub(crate) menu: Handle<Menu>,
    pub(crate) item: Handle<Command>,
    pub(crate) control_receiver_handle: Handle<()>,
}

#[derive(Copy, Clone)]
pub(crate) struct MenuCheckBoxStateChangedEvent {
    pub(crate) command_id: u32,
    pub(crate) menu: Handle<Menu>,
    pub(crate) item: Handle<CheckBox>,
    pub(crate) checked: bool,
    pub(crate) control_receiver_handle: Handle<()>,
}

#[derive(Copy, Clone)]
pub(crate) struct MenuRadioBoxSelectedEvent {
    pub(crate) command_id: u32,
    pub(crate) menu: Handle<Menu>,
    pub(crate) item: Handle<SingleChoice>,
    pub(crate) control_receiver_handle: Handle<()>,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum MenuEvent {
    Command(MenuCommandEvent),
    CheckBoxStateChanged(MenuCheckBoxStateChangedEvent),
    SingleChoiceSelected(MenuRadioBoxSelectedEvent),
}

pub(crate) enum MousePressedMenuResult {
    None,
    Repaint,
    CheckParent,
    Activate,
}
pub(crate) enum MouseMoveMenuResult {
    ProcessedAndRepaint,
    RepaintAndPass,
    ProcessWithoutRepaint,
    Ignored,
}
