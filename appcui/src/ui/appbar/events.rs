use crate::ui::AppBar;

/// A trait that defines the event handlers for AppBar interation
pub trait AppBarEvents {
    /// Called when a button is clicked.
    /// 
    /// # Parameters
    /// * `button` - A handle to the button that was clicked.
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// #[Desktop(events = AppBarEvents+DesktopEvents)]
    /// struct MyDesktop {
    ///     my_button: Handle<appbar::Button>,
    /// }
    /// 
    /// impl MyDesktop {
    ///     fn new() -> Self {
    ///         Self {
    ///             base: Desktop::new(),
    ///             my_button: Handle::None,
    ///         }
    ///     }
    /// }
    /// 
    /// impl DesktopEvents for MyDesktop {
    ///     fn on_start(&mut self) {
    ///         self.my_button = self.appbar().add(
    ///                 appbar::Button::new("Button", 
    ///                                 0, 
    ///                                 appbar::Side::Left));
    ///     }
    /// }
    /// 
    /// impl AppBarEvents for MyDesktop {
    ///     fn on_button_click(&mut self, button: Handle<appbar::Button>) {
    ///         // Do something when the button is clicked
    ///     }
    /// }
    /// ```
    fn on_button_click(&mut self, _button: Handle<super::Button>) {}

    /// Called when a toggle button's state changes.
    /// 
    /// # Parameters
    /// * `togglebutton` - A handle to the toggle button that had its state changed.
    /// * `selected` - The new state of the toggle button.
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// #[Desktop(events = AppBarEvents+DesktopEvents)]
    /// struct MyDesktop {
    ///     my_togglebutton: Handle<appbar::ToggleButton>,
    /// }
    /// 
    /// impl MyDesktop {
    ///     fn new() -> Self {
    ///         Self {
    ///             base: Desktop::new(),
    ///             my_togglebutton: Handle::None,
    ///         }
    ///     }
    /// }
    /// 
    /// impl DesktopEvents for MyDesktop {
    ///     fn on_start(&mut self) {
    ///         self.my_togglebutton = self.appbar().add(
    ///                 appbar::ToggleButton::new("ToggleButton", 
    ///                                         false, 
    ///                                         0, 
    ///                                         appbar::Side::Left));
    ///     }
    /// }
    /// 
    /// impl AppBarEvents for MyDesktop {
    ///     fn on_togglebutton_state_changed(&mut self, togglebutton: Handle<appbar::ToggleButton>, selected: bool) {
    ///         // Do something when the toggle button's state changes
    ///     }
    /// }
    /// ```
    fn on_togglebutton_state_changed(&mut self, _togglebutton: Handle<super::ToggleButton>, _selected: bool) {}

    /// Called when a switch button's state changes.
    /// 
    /// # Parameters
    /// * `switchbutton` - A handle to the switch button that had its state changed.
    /// * `selected` - The new state of the switch button.
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// #[Desktop(events = AppBarEvents+DesktopEvents)]
    /// struct MyDesktop {
    ///     my_switchbutton: Handle<appbar::SwitchButton>,
    /// }
    /// 
    /// impl MyDesktop {
    ///     fn new() -> Self {
    ///         Self {
    ///             base: Desktop::new(),
    ///             my_switchbutton: Handle::None,
    ///         }
    ///     }
    /// }
    /// 
    /// impl DesktopEvents for MyDesktop {
    ///     fn on_start(&mut self) {
    ///         self.my_switchbutton = self.appbar().add(
    ///                 appbar::SwitchButton::new(" State-1 ", " State-2 ", 
    ///                                         false, 
    ///                                         0, 
    ///                                         appbar::Side::Left));
    ///     }
    /// }
    /// 
    /// impl AppBarEvents for MyDesktop {
    ///     fn on_switchbutton_state_changed(&mut self, switchbutton: Handle<appbar::SwitchButton>, selected: bool) {
    ///         // Do something when the switch button's state changes
    ///     }
    /// }
    /// ```
    fn on_switchbutton_state_changed(&mut self, _switchbutton: Handle<super::SwitchButton>, _selected: bool) {}



    /// Called when the app bar needs to be updated. By default, all items are hidden and whenever the focus changes, the AppCUI framework hides all items and starts from the focus control and moves to its parent and calls this method. In this method you should call appbar.show(...) method to show the items you want to show.
    ///
    /// This method is used to add appbar items (e.g. usually a MenuButton)
    /// It is called whenever the focus changes or when a control
    /// explicitly requests a app bar update.
    ///
    /// # Parameters
    /// * `appbar` - A mutable reference to the AppCUI AppBar object.
    /// 
    /// # Example
    /// 
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// #[Desktop(events = AppBarEvents+DesktopEvents)]
    /// struct MyDesktop {
    ///     my_button: Handle<appbar::Button>,
    /// }
    /// impl MyDesktop {
    ///     fn new() -> Self {
    ///         Self {
    ///             base: Desktop::new(),
    ///             my_button: Handle::None,
    ///         }
    ///     }
    /// }
    /// 
    /// impl DesktopEvents for MyDesktop {
    ///     fn on_start(&mut self) {
    ///         // Add the button to the app bar and store the handle
    ///         self.my_button = self.appbar().add(
    ///                 appbar::Button::new("Button", 
    ///                                 0, 
    ///                                 appbar::Side::Left));
    ///     }
    /// }
    /// 
    /// impl AppBarEvents for MyDesktop {
    ///     fn on_update(&self, appbar: &mut AppBar) {
    ///         // Show the button in the app bar
    ///         appbar.show(self.my_button);
    ///     }
    /// }
    /// ```
    fn on_update(&self, _appbar: &mut AppBar) {}
}

use crate::system::Handle;

#[derive(Copy, Clone)]
pub(crate) struct ButtonClickEvent {
    pub(crate) button_handle: Handle<super::Button>,
    pub(crate) control_receiver_handle: Handle<()>,
}

#[derive(Copy, Clone)]
pub(crate) struct ToggleButtonStatusChangedEvent {
    pub(crate) button_handle: Handle<super::ToggleButton>,
    pub(crate) control_receiver_handle: Handle<()>,
    pub(crate) state: bool,
}
#[derive(Copy, Clone)]
pub(crate) struct SwitchButtonStatusChangedEvent {
    pub(crate) button_handle: Handle<super::SwitchButton>,
    pub(crate) control_receiver_handle: Handle<()>,
    pub(crate) state: bool,
}

#[derive(Copy, Clone)]
pub(crate) enum AppBarEvent {
    ButtonClick(ButtonClickEvent),
    ToggleButtonStatusChanged(ToggleButtonStatusChangedEvent),
    SwitchButtonStatusChanged(SwitchButtonStatusChangedEvent),
}
