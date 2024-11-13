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
    #[inline(always)]
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::Key);
    }
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
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
    pub fn is_selected(&self) -> bool {
        self.selected
    }
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
    #[inline(always)]
    pub fn shortcut(&self) -> Key {
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
        format.x = 4;
        surface.write_text_old(self.caption.text(), format);
        let attr = super::utils::get_symbol_attr(self.enabled, current_item, color);
        let symbol = if self.selected {
            SpecialChar::CircleFilled
        } else {
            SpecialChar::CircleEmpty
        };
        surface.write_char(2, format.y, Character::with_attributes(symbol, attr));
        if self.shortcut.code != KeyCode::None {
            super::utils::paint_shortcut(self.shortcut, surface, format, width, self.enabled, current_item, color);
        }
    }
}
impl MenuItem for SingleChoice {
    fn into_menuitem(self) -> MenuItemWrapper {
        MenuItemWrapper::SingleChoice(self)
    }
    fn update_handles(&mut self, parent: Handle<crate::prelude::Menu>, me: Handle<crate::prelude::common::UIElement>) {
        self.menu_handle = parent;
        self.handle = me.cast();
    }
}
