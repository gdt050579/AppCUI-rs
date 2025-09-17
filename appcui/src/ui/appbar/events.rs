use crate::ui::AppBar;

/// A trait that defines the event handlers for AppBar interation
pub trait AppBarEvents {
    fn on_button_click(&mut self, _button: Handle<super::Button>) {}

    /// Called when a checkbox menu item's state changes.
    ///
    /// # Parameters
    /// * `menu` - A handle to the menu containing the checkbox.
    /// * `item` - A handle to the checkbox that was clicked.
    /// * `command` - The command identifier associated with the menu item.
    /// * `checked` - The new checked state of the checkbox.
    // fn on_check(&mut self, _menu: Handle<Menu>, _item: Handle<CheckBox>, _command: u32, _checked: bool) {}

    /// Called when a single choice menu item is selected.
    ///
    /// # Parameters
    /// * `menu` - A handle to the menu containing the single choice item.
    /// * `item` - A handle to the single choice item that was selected.
    /// * `command` - The command identifier associated with the menu item.
    // fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<SingleChoice>, _command: u32) {}

    /// Called when the app bar needs to be updated.
    ///
    /// This method is used to add appbar items (e.g. usually a MenuButton)
    /// It is called whenever the focus changes or when a control
    /// explicitly requests a app bar update.
    ///
    /// # Parameters
    /// * `appbar` - A mutable reference to the AppCUI AppBar object.
    fn on_update(&self, _appbar: &mut AppBar) {}
}

use crate::system::Handle;

#[derive(Copy, Clone)]
pub(crate) struct ButtonClickEvent {
    pub(crate) button_handle: Handle<super::Button>,
    pub(crate) control_receiver_handle: Handle<()>,
}

#[derive(Copy, Clone)]
pub(crate) enum AppBarEvent {
    ButtonClick(ButtonClickEvent),
}
