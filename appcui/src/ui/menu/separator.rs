use crate::{
    graphics::{LineType, Surface},
    system::MenuTheme,
    ui::menu::Menu,
    system::Handle
};

use super::{menu_item::MenuItem, MenuItemWrapper};

pub struct Separator {
    pub(super) menu_handle: Handle<Menu>,
    pub(super) handle: Handle<Separator>
}
impl Separator {
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
    fn update_handles(&mut self, parent: Handle<crate::prelude::Menu>, me: Handle<crate::prelude::common::UIElement>) {
        self.menu_handle = parent;
        self.handle = me.cast();
    }
}
