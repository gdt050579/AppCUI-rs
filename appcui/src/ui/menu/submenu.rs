use crate::{
    graphics::{Character, SpecialChar, Surface, TextFormat},
    system::{Handle, MenuTheme, RuntimeManager},
    utils::Caption,
    utils::ExtractHotKeyMethod
};

use super::Menu;
use super::{menu_item::MenuItem, MenuItemWrapper};

/// A submenu menu item that contains another menu.
///
/// A submenu is a menu item that, when selected, opens another menu containing
/// additional menu items. This allows for creating hierarchical menu structures.
/// Submenus are useful for organizing related menu items into logical groups.
///
/// # Examples
///
/// Creating a submenu and adding it to a parent menu:
///
/// ```rust
/// use appcui::prelude::*;
///
/// #[Window(events = MenuEvents, commands = BoldText+ItalicText+UnderlineText)]
/// struct MyWindow {
///     format_menu: Handle<Menu>,
/// }
///
/// impl MyWindow {
///     fn new() -> Self {
///         let mut w = MyWindow {
///             base: window!("Example,d:c,w:40,h:10"),
///             format_menu: Handle::None,
///         };
///         
///         // Create the main format menu
///         let mut format_menu = Menu::new("&Format");
///         
///         // Create a text style submenu
///         let mut text_style_menu = Menu::new("Text &Style");
///         
///         // Add items to the text style submenu
///         text_style_menu.add(menu::Command::new(
///             "&Bold", 
///             key!("Ctrl+B"), 
///             mywindow::Commands::BoldText
///         ));
///         
///         text_style_menu.add(menu::Command::new(
///             "&Italic", 
///             key!("Ctrl+I"), 
///             mywindow::Commands::ItalicText
///         ));
///         
///         text_style_menu.add(menu::Command::new(
///             "&Underline", 
///             key!("Ctrl+U"), 
///             mywindow::Commands::UnderlineText
///         ));
///         
///         // Create a SubMenu from the text style menu and add it to the format menu
///         format_menu.add(menu::SubMenu::new(text_style_menu));
///         
///         // Register the format menu with the window
///         w.format_menu = w.register_menu(format_menu);
///         
///         w
///     }
/// }
///
/// impl MenuEvents for MyWindow {
///     fn on_update_menubar(&self, menubar: &mut MenuBar) {
///         menubar.add(self.format_menu, 0);
///     }
///     
///     fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: mywindow::Commands) {
///         match command {
///             mywindow::Commands::BoldText => {
///                 // Handle bold text formatting
///             },
///             mywindow::Commands::ItalicText => {
///                 // Handle italic text formatting
///             },
///             mywindow::Commands::UnderlineText => {
///                 // Handle underline text formatting
///             },
///         }
///     }
/// }
/// ```
pub struct SubMenu {
    pub(super) enabled: bool,
    pub(super) caption: Caption,
    pub(super) submenu_handle: Handle<Menu>,
    pub(super) menu_handle: Handle<Menu>,
    pub(super) handle: Handle<SubMenu>,
}
impl SubMenu {
    /// Creates a new submenu from an existing menu.
    ///
    /// The provided menu is registered with the framework and becomes
    /// accessible through this submenu item.
    ///
    /// # Parameters
    /// * `menu` - The menu to be associated with this submenu.
    ///
    /// # Returns
    /// A new `SubMenu` instance.
    pub fn new(menu: Menu) -> Self {
        let mut caption = menu.caption.clone();        
        let handle = RuntimeManager::get().get_menus().add(menu);
        // submenu hotkey should be a letter while a menu hotkey shoult be Alt+Letter
        // as such, we will clear the Alt if it is set up
        caption.clear_hotkey_modifier();
        SubMenu {
            enabled: true,
            caption,
            submenu_handle: handle,
            handle: Handle::None,
            menu_handle: Handle::None,
        }
    }
    
    /// Sets a new caption for the submenu.
    ///
    /// # Parameters
    /// * `text` - The new caption text. If it contains the `&` character,
    ///   the next character will be used as a hotkey.
    #[inline(always)]
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::Key);
    }
    
    /// Returns the current caption text of the submenu.
    ///
    /// # Returns
    /// The caption text as a string slice.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }
    
    /// Checks if the submenu is enabled.
    ///
    /// # Returns
    /// `true` if the submenu is enabled, `false` otherwise.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Enables or disables the submenu.
    ///
    /// # Parameters
    /// * `value` - `true` to enable the submenu, `false` to disable it.
    #[inline(always)]
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub(super) fn paint(&self, surface: &mut Surface, format: &mut TextFormat, width: u16, current_item: bool, color: &MenuTheme) {
        super::utils::update_format_with_caption(&self.caption, format, self.enabled, current_item, color);
        if current_item && self.enabled {
            // highlight current item
            surface.fill_horizontal_line_with_size(1, format.y, width as u32, Character::with_attributes(' ', color.text.hovered));
        }
        format.x = 2;
        surface.write_text(self.caption.text(), format);
        surface.write_char(
            width as i32,
            format.y,
            Character::with_attributes(SpecialChar::TriangleRight, format.char_attr),
        );
    }
}
impl MenuItem for SubMenu {
    fn into_menuitem(self) -> MenuItemWrapper {
        MenuItemWrapper::SubMenu(self)
    }
    fn update_handles(&mut self, parent: Handle<crate::prelude::Menu>, me: Handle<()>) {
        self.menu_handle = parent;
        self.handle = me.cast();
        if let Some(menu) = RuntimeManager::get().get_menu(self.submenu_handle) {
            menu.parent_handle = parent;
        }
    }
}
