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

pub struct Command {
    pub(super) enabled: bool,
    pub(super) command_id: u32,
    pub(super) caption: Caption,
    pub(super) shortcut: Key,
    pub(super) menu_handle: Handle<Menu>,
    pub(super) handle: Handle<Command>,
}
impl Command {
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
    #[inline(always)]
    pub fn get_shortcut(&self) -> Key {
        self.shortcut
    }
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
        surface.write_text(self.caption.get_text(), format);
        if self.shortcut.code != KeyCode::None {
            super::utils::paint_shortcut(self.shortcut, surface, format, width, self.enabled, current_item, color);
        }
    }
}
impl MenuItem for Command {
    fn into_menuitem(self) -> MenuItemWrapper {
        MenuItemWrapper::Command(self)
    }
    fn update_handles(&mut self, parent: Handle<crate::prelude::Menu>, me: Handle<crate::prelude::common::UIElement>) {
        self.menu_handle = parent;
        self.handle = me.cast();
    }
}
