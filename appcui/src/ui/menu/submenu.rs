use crate::{
    graphics::{Character, SpecialChar, Surface, TextFormat},
    system::{Handle, MenuTheme, RuntimeManager},
    utils::Caption,
    utils::ExtractHotKeyMethod
};

use super::Menu;
use super::{menu_item::MenuItem, MenuItemWrapper};

pub struct SubMenu {
    pub(super) enabled: bool,
    pub(super) caption: Caption,
    pub(super) submenu_handle: Handle<Menu>,
    pub(super) menu_handle: Handle<Menu>,
    pub(super) handle: Handle<SubMenu>,
}
impl SubMenu {
    pub fn new(menu: Menu) -> Self {
        let mut caption = menu.caption.clone();        
        let handle = RuntimeManager::get().get_menus().add(menu);
        // submenu hotkey should be a letter while a menu hotkey shoult be Alt+Letter
        // as such, we will clear the Alt if it is set up
        caption.clear_hotkey_modifier();
        SubMenu {
            enabled: true,
            caption: caption,
            submenu_handle: handle,
            handle: Handle::None,
            menu_handle: Handle::None,
        }
    }
    #[inline(always)]
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::Key);
    }
    #[inline(always)]
    pub fn get_caption(&self) -> &str {
        self.caption.get_text()
    }
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
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
        surface.write_text(self.caption.get_text(), format);
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
    fn update_handles(&mut self, parent: Handle<crate::prelude::Menu>, me: Handle<crate::prelude::common::UIElement>) {
        self.menu_handle = parent;
        self.handle = me.cast();
        if let Some(menu) = RuntimeManager::get().get_menu(self.submenu_handle) {
            menu.parent_handle = parent;
        }
    }
}
