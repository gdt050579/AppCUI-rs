use crate::{
    graphics::{LineType, Surface},
    system::MenuTheme,
    ui::menu::Menu,
    system::Handle, 
};

use super::{menu_item::MenuItem, MenuItemWrapper};

/// A separator item for visually separating groups of menu items.
///
/// A separator is a non-interactive menu item that displays as a horizontal line.
/// Separators are used to visually group related menu items together and improve
/// the organization and readability of menus.
pub struct Separator {
    pub(super) menu_handle: Handle<Menu>,
    pub(super) handle: Handle<Separator>
}
impl Separator {
    /// Creates a new separator menu item.
    ///
    /// # Returns
    /// A new `Separator` instance.
    pub fn new() -> Self {
        Self {
            menu_handle: Handle::None,
            handle: Handle::None,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, y: i32, width: u16, color: &MenuTheme) {
        surface.draw_horizontal_line_with_size(1, y, width as u32, LineType::Single, color.text.normal);
    }
}
impl MenuItem for Separator {
    fn into_menuitem(self) -> super::MenuItemWrapper {
        MenuItemWrapper::Separator(self)
    }
    fn update_handles(&mut self, parent: Handle<crate::prelude::Menu>, me: Handle<()>) {
        self.menu_handle = parent;
        self.handle = me.cast();
    }
}
