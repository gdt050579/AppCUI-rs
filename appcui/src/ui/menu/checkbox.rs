use super::{menu_item::IntoMenuItem, MenuItem};
use crate::{
    graphics::{Character, SpecialChar, Surface, TextFormat},
    input::{Key, KeyCode},
    system::MenuTheme,
    ui::common::traits::CommandID,
    utils::Caption,
};
pub struct CheckBox {
    pub(super) enabled: bool,
    pub(super) checked: bool,
    pub(super) command_id: u32,
    pub(super) caption: Caption,
    pub(super) shortcut: Key,
}
impl CheckBox {
    pub fn new<T, U>(text: &str, shortcut: T, command_id: U, checked: bool) -> Self
    where
        Key: From<T>,
        u32: From<U>,
        U: CommandID + Copy,
    {
        Self {
            enabled: true,
            command_id: u32::from(command_id),
            caption: Caption::new(text, true),
            shortcut: Key::from(shortcut),
            checked,
        }
    }
    pub(super) fn paint(&self, surface: &mut Surface, format: &mut TextFormat, width: u16, current_item: bool, color: &MenuTheme) {
        super::utils::update_format_with_caption(&self.caption, format, self.enabled, current_item, color);
        if current_item && self.enabled {
            // highlight current item
            surface.fill_horizontal_line_with_size(1, format.y, width as u32, Character::with_attributes(' ', color.text.hovered));
        }
        format.x = 4;
        surface.write_text(self.caption.get_text(), format);
        if self.checked {
            let attr = super::utils::get_symbol_attr(self.enabled, current_item, color);
            surface.write_char(2, format.y, Character::with_attributes(SpecialChar::CheckMark, attr));
        }
        if self.shortcut.code != KeyCode::None {
            super::utils::paint_shortcut(self.shortcut, surface, format, width, self.enabled, current_item, color);
        }
    }
}
impl IntoMenuItem for CheckBox {
    fn into_menuitem(self) -> MenuItem {
        MenuItem::CheckBox(self)
    }
}
