use super::{menu_item::MenuItem, MenuItemWrapper};
use crate::{
    graphics::{Character, SpecialChar, Surface},
    input::{Key, KeyCode},
    prelude::TextFormat,
    system::{Handle, MenuTheme},
    ui::{common::traits::CommandID, menu::Menu},
    utils::{Caption, ExtractHotKeyMethod},
};

/// A checkbox menu item that can be checked or unchecked.
///
/// A checkbox menu item has two states: checked or unchecked. Clicking the item
/// toggles its state and sends a command to the control that owns the menu.
/// Checkbox menu items are useful for representing boolean options in menus.
///
/// # Examples
///
/// Creating checkbox menu items and adding them to a menu:
///
/// ```rust
/// use appcui::prelude::*;
///
/// #[Window(events = MenuEvents, commands = ShowStatusBar+ShowToolbar+WordWrap)]
/// struct MyWindow {
///     view_menu: Handle<Menu>,
///     status_bar_checkbox: Handle<menu::CheckBox>,
///     toolbar_checkbox: Handle<menu::CheckBox>,
///     word_wrap_checkbox: Handle<menu::CheckBox>,
/// }
///
/// impl MyWindow {
///     fn new() -> Self {
///         let mut w = MyWindow {
///             base: window!("Example,d:c,w:40,h:10"),
///             view_menu: Handle::None,
///             status_bar_checkbox: Handle::None,
///             toolbar_checkbox: Handle::None,
///             word_wrap_checkbox: Handle::None,
///         };
///         
///         // Create a menu for view options
///         let mut view_menu = Menu::new("&View");
///         
///         // Add checkbox items with initial states
///         w.status_bar_checkbox = view_menu.add(menu::CheckBox::new(
///             "Show &Status Bar", 
///             Key::None, 
///             mywindow::Commands::ShowStatusBar, 
///             true // initially checked
///         ));
///         
///         w.toolbar_checkbox = view_menu.add(menu::CheckBox::new(
///             "Show &Toolbar", 
///             Key::None, 
///             mywindow::Commands::ShowToolbar, 
///             true // initially checked
///         ));
///         
///         w.word_wrap_checkbox = view_menu.add(menu::CheckBox::new(
///             "&Word Wrap", 
///             key!("Ctrl+W"), 
///             mywindow::Commands::WordWrap, 
///             false // initially unchecked
///         ));
///         
///         // Register the menu with the window
///         w.view_menu = w.register_menu(view_menu);
///         
///         w
///     }
///     
///     // Method to programmatically toggle word wrap
///     fn toggle_word_wrap(&mut self) {
///         let h_mnu = self.view_menu;
///         let h_item = self.word_wrap_checkbox;
///         if let Some(word_wrap) = self.menuitem_mut(h_mnu, h_item) {
///             word_wrap.set_checked(!word_wrap.is_checked());
///             // Also handle the actual functionality change
///         }
///     }
///     
///     fn apply_word_wrap(&self, enabled: bool) {
///         // Implementation of word wrap functionality
///     }
/// }
///
/// impl MenuEvents for MyWindow {
///     fn on_check(&mut self, _menu: Handle<Menu>, item: Handle<menu::CheckBox>, 
///                command: mywindow::Commands, checked: bool) {
///         match command {
///             mywindow::Commands::ShowStatusBar => {
///                 // Show or hide status bar based on 'checked' value
///             },
///             mywindow::Commands::ShowToolbar => {
///                 // Show or hide toolbar based on 'checked' value
///             },
///             mywindow::Commands::WordWrap => {
///                 self.apply_word_wrap(checked);
///             },
///         }
///     }
/// }
/// ```
pub struct CheckBox {
    pub(super) enabled: bool,
    pub(super) checked: bool,
    pub(super) command_id: u32,
    pub(super) caption: Caption,
    pub(super) shortcut: Key,
    pub(super) menu_handle: Handle<Menu>,
    pub(super) handle: Handle<CheckBox>,
}
impl CheckBox {
    /// Creates a new checkbox menu item.
    ///
    /// # Parameters
    /// * `text` - The caption text to display. If it contains the `&` character,
    ///   the next character will be used as a hotkey.
    /// * `shortcut` - The keyboard shortcut that activates this checkbox (e.g., F1 or Ctrl+C).
    /// * `command_id` - The command identifier that will be sent when the item is activated.
    /// * `checked` - The initial checked state of the checkbox.
    ///
    /// # Returns
    /// A new `CheckBox` instance.
    pub fn new<T, U>(text: &str, shortcut: T, command_id: U, checked: bool) -> Self
    where
        Key: From<T>,
        u32: From<U>,
        U: CommandID + Copy,
    {
        Self {
            enabled: true,
            command_id: u32::from(command_id),
            caption: Caption::new(text, ExtractHotKeyMethod::Key),
            shortcut: Key::from(shortcut),
            checked,
            handle: Handle::None,
            menu_handle: Handle::None,
        }
    }
    
    /// Sets a new caption for the checkbox.
    ///
    /// # Parameters
    /// * `text` - The new caption text. If it contains the `&` character,
    ///   the next character will be used as a hotkey.
    #[inline(always)]
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::Key);
    }
    
    /// Returns the current caption text of the checkbox.
    ///
    /// # Returns
    /// The caption text as a string slice.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }
    
    /// Checks if the checkbox is currently checked.
    ///
    /// # Returns
    /// `true` if the checkbox is checked, `false` otherwise.
    #[inline(always)]
    pub fn is_checked(&self) -> bool {
        self.checked
    }
    
    /// Sets the checked state of the checkbox.
    ///
    /// # Parameters
    /// * `value` - `true` to check the checkbox, `false` to uncheck it.
    #[inline(always)]
    pub fn set_checked(&mut self, value: bool) {
        self.checked = value;
    }
    
    /// Checks if the checkbox is enabled.
    ///
    /// # Returns
    /// `true` if the checkbox is enabled, `false` otherwise.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Enables or disables the checkbox.
    ///
    /// # Parameters
    /// * `value` - `true` to enable the checkbox, `false` to disable it.
    #[inline(always)]
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }
    
    /// Returns the keyboard shortcut associated with the checkbox.
    ///
    /// # Returns
    /// The `Key` representing the shortcut.
    #[inline(always)]
    pub fn shortcut(&self) -> Key {
        self.shortcut
    }
    
    /// Sets a new keyboard shortcut for the checkbox.
    ///
    /// # Parameters
    /// * `shortcut` - The new shortcut to set.
    #[inline(always)]
    pub fn set_shortcut<T>(&mut self, shortcut: T)
    where
        Key: From<T>,
    {
        self.shortcut = Key::from(shortcut)
    }

    pub(super) fn paint(&self, surface: &mut Surface, format: &mut TextFormat, width: u16, current_item: bool, color: &MenuTheme) {
        super::utils::update_format_with_caption(&self.caption, format, self.enabled, current_item, color);
        if current_item && self.enabled {
            // highlight current item
            surface.fill_horizontal_line_with_size(1, format.y, width as u32, Character::with_attributes(' ', color.text.hovered));
        }
        format.x = 4;
        surface.write_text(self.caption.text(), format);
        if self.checked {
            let attr = super::utils::get_symbol_attr(self.enabled, current_item, color);
            surface.write_char(2, format.y, Character::with_attributes(SpecialChar::CheckMark, attr));
        }
        if self.shortcut.code != KeyCode::None {
            super::utils::paint_shortcut(self.shortcut, surface, format.y, width, self.enabled, current_item, color);
        }
    }
}
impl MenuItem for CheckBox {
    fn into_menuitem(self) -> MenuItemWrapper {
        MenuItemWrapper::CheckBox(self)
    }

    fn update_handles(&mut self, parent: Handle<crate::prelude::Menu>, me: Handle<()>) {
        self.menu_handle = parent;
        self.handle = me.cast();
    }
}
