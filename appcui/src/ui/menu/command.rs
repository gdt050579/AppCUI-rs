use crate::{
    graphics::{Character, Surface, TextFormat},
    input::{Key, KeyCode},
    system::Handle,
    system::MenuTheme,
    ui::common::traits::CommandID,
    ui::menu::Menu,
    utils::Caption,
    utils::ExtractHotKeyMethod
};

use super::{menu_item::MenuItem, MenuItemWrapper};

/// A command menu item that triggers an action when selected.
///
/// A command is the most basic type of menu item, similar to a button.
/// When clicked, it sends a command to the control that owns the menu.
/// Command items can have a hotkey (designated by the `&` character in the text)
/// and a shortcut key that can be used to activate them.
///
/// # Examples
///
/// Creating a command menu item and adding it to a menu:
///
/// ```rust
/// use appcui::prelude::*;
///
/// #[Window(events = MenuEvents, commands = Cut+Copy+Paste)]
/// struct MyWindow {
///     edit_menu: Handle<Menu>,
///     cut_command: Handle<menu::Command>,
///     copy_command: Handle<menu::Command>,
///     paste_command: Handle<menu::Command>,
/// }
///
/// impl MyWindow {
///     fn new() -> Self {
///         let mut w = MyWindow {
///             base: window!("Example,d:c,w:40,h:10"),
///             edit_menu: Handle::None,
///             cut_command: Handle::None,
///             copy_command: Handle::None,
///             paste_command: Handle::None,
///         };
///         
///         // Create a menu for editing commands
///         let mut edit_menu = Menu::new("&Edit");
///         
///         // Add command items with keyboard shortcuts
///         w.cut_command = edit_menu.add(menu::Command::new("Cu&t", key!("Ctrl+X"), mywindow::Commands::Cut));
///         w.copy_command = edit_menu.add(menu::Command::new("&Copy", key!("Ctrl+C"), mywindow::Commands::Copy));
///         w.paste_command = edit_menu.add(menu::Command::new("&Paste", key!("Ctrl+V"), mywindow::Commands::Paste));
///         
///         // Register the menu with the window
///         w.edit_menu = w.register_menu(edit_menu);
///         
///         w
///     }
///     
///     // Method to enable or disable the paste command
///     fn update_paste_availability(&mut self, has_clipboard_content: bool) {
///         let h_mnu = self.edit_menu;
///         let h_item = self.paste_command;
///         if let Some(paste_cmd) = self.menuitem_mut(h_mnu, h_item) {
///             paste_cmd.set_enabled(has_clipboard_content);
///         }
///     }
/// }
///
/// impl MenuEvents for MyWindow {
///     fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: mywindow::Commands) {
///         match command {
///             mywindow::Commands::Cut => { /* Handle Cut command */ },
///             mywindow::Commands::Copy => { /* Handle Copy command */ },
///             mywindow::Commands::Paste => { /* Handle Paste command */ },
///         }
///     }
/// }
/// ```
pub struct Command {
    pub(super) enabled: bool,
    pub(super) command_id: u32,
    pub(super) caption: Caption,
    pub(super) shortcut: Key,
    pub(super) menu_handle: Handle<Menu>,
    pub(super) handle: Handle<Command>,
}
impl Command {
    /// Creates a new command menu item.
    ///
    /// # Parameters
    /// * `text` - The caption text to display. If it contains the `&` character,
    ///   the next character will be used as a hotkey.
    /// * `shortcut` - The keyboard shortcut that activates this command (e.g., F1 or Ctrl+C).
    /// * `command_id` - The command identifier that will be sent when the item is activated.
    ///
    /// # Returns
    /// A new `Command` instance.
    pub fn new<T, U>(text: &str, shortcut: T, command_id: U) -> Self
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
            handle: Handle::None,
            menu_handle: Handle::None,
        }
    }
    
    /// Sets a new caption for the command.
    ///
    /// # Parameters
    /// * `text` - The new caption text. If it contains the `&` character,
    ///   the next character will be used as a hotkey.
    #[inline(always)]
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::Key);
    }
    
    /// Returns the current caption text of the command.
    ///
    /// # Returns
    /// The caption text as a string slice.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }
    
    /// Checks if the command is enabled.
    ///
    /// # Returns
    /// `true` if the command is enabled, `false` otherwise.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Enables or disables the command.
    ///
    /// # Parameters
    /// * `value` - `true` to enable the command, `false` to disable it.
    #[inline(always)]
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }
    
    /// Returns the keyboard shortcut associated with the command.
    ///
    /// # Returns
    /// The `Key` representing the shortcut.
    #[inline(always)]
    pub fn shortcut(&self) -> Key {
        self.shortcut
    }
    
    /// Sets a new keyboard shortcut for the command.
    ///
    /// # Parameters
    /// * `shortcut` - The new shortcut to set.
    #[inline(always)]
    pub fn set_shortcut<T>(&mut self, shortcut: T) where Key: From<T>, {
        self.shortcut = Key::from(shortcut)
    }

    pub(super) fn paint(&self, surface: &mut Surface, format: &mut TextFormat, width: u16, current_item: bool, color: &MenuTheme) {
        super::utils::update_format_with_caption(&self.caption, format, self.enabled, current_item, color);
        if current_item && self.enabled {
            // highlight current item
            surface.fill_horizontal_line_with_size(1, format.y, width as u32, Character::with_attributes(' ', color.text.hovered));
        }
        format.x = 2;
        surface.write_text(self.caption.text(), format);
        if self.shortcut.code != KeyCode::None {
            super::utils::paint_shortcut(self.shortcut, surface, format.y, width, self.enabled, current_item, color);
        }
    }
}
impl MenuItem for Command {
    fn into_menuitem(self) -> MenuItemWrapper {
        MenuItemWrapper::Command(self)
    }
    fn update_handles(&mut self, parent: Handle<crate::prelude::Menu>, me: Handle<()>) {
        self.menu_handle = parent;
        self.handle = me.cast();
    }
}
