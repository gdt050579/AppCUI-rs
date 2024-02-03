use crate::{
    graphics::{Character, SpecialChar, Surface, TextFormat},
    system::{Handle, MenuTheme},
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
    pub fn new(&mut self, mut menu: Menu) {
        menu.parent_handle = self.handle;
        let caption = menu.caption.clone();
        let handle = RuntimeManager::get().get_menus().add(menu);
        let item = SubMenu {
            enabled: true,
            caption: caption,
            submenu_handle: handle,
        };
        self.items.push(MenuItem::SubMenu(item));
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
}