use crate::{
    graphics::{Character, SpecialChar, Surface, TextFormat},
    system::{Handle, MenuTheme, RuntimeManager},
    utils::Caption,
};

use super::Menu;
use super::{menu_item::IntoMenuItem, MenuItem};

pub struct SubMenu {
    pub(super) enabled: bool,
    pub(super) caption: Caption,
    pub(super) submenu_handle: Handle<Menu>,
}
impl SubMenu {
    pub fn new(mut menu: Menu) -> Self {        
        let caption = menu.caption.clone();
        let handle = RuntimeManager::get().get_menus().add(menu);
        SubMenu {
            enabled: true,
            caption: caption,
            submenu_handle: handle,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, format: &mut TextFormat, width: u16, current_item: bool, color: &MenuTheme) {
        super::utils::update_format_with_caption(&self.caption, format, self.enabled, current_item, color);
        if current_item && self.enabled {
            // highlight current item
            surface.fill_horizontal_line_with_size(1, format.y, width as u32, Character::with_attributes(' ', color.text.hovered));
        }
        format.x = 2;
        surface.write_text(self.caption.get_text(), format);
        surface.write_char(
            width as i32,
            format.y,
            Character::with_attributes(SpecialChar::TriangleRight, format.char_attr),
        );
    }
}
impl IntoMenuItem for SubMenu {
    fn into_menuitem(self) -> MenuItem {
        MenuItem::SubMenu(self)
    }
    fn update_parent_handle(&mut self, parent: Handle<Menu>) {
        if let Some(menu) = RuntimeManager::get().get_menu(self.submenu_handle) {
            menu.parent_handle = parent;
        }
    }
}
