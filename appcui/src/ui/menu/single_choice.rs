use super::{menu_item::MenuItem, MenuItemWrapper};
use crate::{
    graphics::{Character, SpecialChar, Surface, TextFormat},
    input::{Key, KeyCode},
    system::MenuTheme,
    ui::common::traits::CommandID,
    utils::Caption,
    utils::ExtractHotKeyMethod,
    ui::menu::Menu,
    system::Handle, prelude::RuntimeManager
};

/// A single choice menu item that represents one option in a group.
///
/// Single choice menu items are grouped together, with only one item in the group
/// being selected at a time. When a single choice item is selected, any other selected
/// item in the same group is automatically deselected. This is useful for representing
/// mutually exclusive options in menus.
///
/// # Examples
///
/// Creating a group of single choice menu items:
///
/// ```rust
/// use appcui::prelude::*;
///
/// #[Window(events = MenuEvents, commands = TextSize_Small+TextSize_Medium+TextSize_Large)]
/// struct MyWindow {
///     format_menu: Handle<Menu>,
///     text_size_small: Handle<menu::SingleChoice>,
///     text_size_medium: Handle<menu::SingleChoice>,
///     text_size_large: Handle<menu::SingleChoice>,
/// }
///
/// impl MyWindow {
///     fn new() -> Self {
///         let mut w = MyWindow {
///             base: window!("Example,d:c,w:40,h:10"),
///             format_menu: Handle::None,
///             text_size_small: Handle::None,
///             text_size_medium: Handle::None,
///             text_size_large: Handle::None,
///         };
///         
///         // Create a menu for format options
///         let mut format_menu = Menu::new("&Format");
///         
///         // Create a submenu for text size options
///         let mut text_size_menu = Menu::new("Text &Size");
///         
///         // Add single choice items with the medium option initially selected
///         w.text_size_small = text_size_menu.add(menu::SingleChoice::new(
///             "&Small", 
///             Key::None, 
///             mywindow::Commands::TextSize_Small, 
///             false // not initially selected
///         ));
///         
///         w.text_size_medium = text_size_menu.add(menu::SingleChoice::new(
///             "&Medium", 
///             Key::None, 
///             mywindow::Commands::TextSize_Medium, 
///             true // initially selected
///         ));
///         
///         w.text_size_large = text_size_menu.add(menu::SingleChoice::new(
///             "&Large", 
///             Key::None, 
///             mywindow::Commands::TextSize_Large, 
///             false // not initially selected
///         ));
///         
///         // Add the text size submenu to the format menu
///         format_menu.add(menu::SubMenu::new(text_size_menu));
///         
///         // Register the format menu with the window
///         w.format_menu = w.register_menu(format_menu);
///         
///         w
///     }
///     
///     // Method to programmatically set text size
///     fn set_text_size(&mut self, size: TextSize) {
///         // Set the appropriate menu item
///         match size {
///             TextSize::Small => {
///                 let h_mnu = self.format_menu;
///                 let h_item = self.text_size_small;
///                 if let Some(item) = self.menuitem_mut(h_mnu, h_item) {
///                     item.set_selected();
///                 }
///             },
///             TextSize::Medium => {
///                 let h_mnu = self.format_menu;
///                 let h_item = self.text_size_medium;
///                 if let Some(item) = self.menuitem_mut(h_mnu, h_item) {
///                     item.set_selected();
///                 }
///             },
///             TextSize::Large => {
///                 let h_mnu = self.format_menu;
///                 let h_item = self.text_size_large;
///                 if let Some(item) = self.menuitem_mut(h_mnu, h_item) {
///                     item.set_selected();
///                 }
///             },
///         }
///         
///         // Apply the text size change
///         self.apply_text_size(size);
///     }
///     
///     fn apply_text_size(&self, size: TextSize) {
///         // Implementation of text size change
///     }
/// }
///
/// // Define text size enum for the example
/// enum TextSize {
///     Small,
///     Medium,
///     Large,
/// }
///
/// impl MenuEvents for MyWindow {
///     fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, 
///                 command: mywindow::Commands) {
///         match command {
///             mywindow::Commands::TextSize_Small => {
///                 self.apply_text_size(TextSize::Small);
///             },
///             mywindow::Commands::TextSize_Medium => {
///                 self.apply_text_size(TextSize::Medium);
///             },
///             mywindow::Commands::TextSize_Large => {
///                 self.apply_text_size(TextSize::Large);
///             },
///         }
///     }
/// }
/// ```
pub struct SingleChoice {
    pub(super) enabled: bool,
    pub(super) selected: bool,
    pub(super) command_id: u32,
    pub(super) caption: Caption,
    pub(super) shortcut: Key,
    pub(super) menu_handle: Handle<Menu>,
    pub(super) handle: Handle<SingleChoice>
}
impl SingleChoice {
    /// Creates a new single choice menu item.
    ///
    /// # Parameters
    /// * `text` - The caption text to display. If it contains the `&` character,
    ///   the next character will be used as a hotkey.
    /// * `shortcut` - The keyboard shortcut that activates this item (e.g., F1 or Ctrl+C).
    /// * `command_id` - The command identifier that will be sent when the item is activated.
    /// * `selected` - The initial selection state of the item.
    ///
    /// # Returns
    /// A new `SingleChoice` instance.
    pub fn new<T, U>(text: &str, shortcut: T, command_id: U, selected: bool) -> Self
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
            selected,
            handle: Handle::None,
            menu_handle: Handle::None,
        }
    }
    
    /// Sets a new caption for the single choice item.
    ///
    /// # Parameters
    /// * `text` - The new caption text. If it contains the `&` character,
    ///   the next character will be used as a hotkey.
    #[inline(always)]
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::Key);
    }
    
    /// Returns the current caption text of the single choice item.
    ///
    /// # Returns
    /// The caption text as a string slice.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }
    
    /// Checks if the single choice item is enabled.
    ///
    /// # Returns
    /// `true` if the item is enabled, `false` otherwise.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Enables or disables the single choice item.
    ///
    /// # Parameters
    /// * `value` - `true` to enable the item, `false` to disable it.
    #[inline(always)]
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }
    
    /// Checks if the single choice item is currently selected.
    ///
    /// # Returns
    /// `true` if the item is selected, `false` otherwise.
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    
    /// Selects this single choice item and deselects others in the same group.
    ///
    /// This method should only be called after the item has been added to a menu.
    /// It will automatically deselect any other selected item in the same group.
    ///
    /// # Panics
    /// Panics if the item is not part of a registered menu.
    #[inline(always)]
    pub fn set_selected(&mut self) {
        if self.handle.is_none() {
            panic!("`set_selected` method should only be called after a sigle choice item was added to a registered menu !");
        }
        if self.menu_handle.is_none() {
            panic!("`set_selected` method should only be called after a sigle choice item was added to a registered menu !");
        }
        let index = self.handle.index();
        if let Some(menu) = RuntimeManager::get().get_menu(self.menu_handle) {
            menu.select_single_choice(index);
        }
    }
    
    /// Returns the keyboard shortcut associated with the single choice item.
    ///
    /// # Returns
    /// The `Key` representing the shortcut.
    #[inline(always)]
    pub fn shortcut(&self) -> Key {
        self.shortcut
    }
    
    /// Sets a new keyboard shortcut for the single choice item.
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
        format.x = 4;
        surface.write_text(self.caption.text(), format);
        let attr = super::utils::get_symbol_attr(self.enabled, current_item, color);
        let symbol = if self.selected {
            SpecialChar::CircleFilled
        } else {
            SpecialChar::CircleEmpty
        };
        surface.write_char(2, format.y, Character::with_attributes(symbol, attr));
        if self.shortcut.code != KeyCode::None {
            super::utils::paint_shortcut(self.shortcut, surface, format.y, width, self.enabled, current_item, color);
        }
    }
}
impl MenuItem for SingleChoice {
    fn into_menuitem(self) -> MenuItemWrapper {
        MenuItemWrapper::SingleChoice(self)
    }
    fn update_handles(&mut self, parent: Handle<crate::prelude::Menu>, me: Handle<()>) {
        self.menu_handle = parent;
        self.handle = me.cast();
    }
}
