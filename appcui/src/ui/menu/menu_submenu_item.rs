use crate::{
    graphics::{Surface, TextFormat, Character, SpecialChar},
    system::{MenuTheme, Handle},
    utils::Caption,
};

use super::Menu;

pub(super) struct MenuSubMenuItem {
    pub(super) enabled: bool,
    pub(super) caption: Caption,
    pub(super) submenu_handle: Handle<Menu>
}
impl MenuSubMenuItem {
    pub (super) fn paint(
        &self,
        surface: &mut Surface,
        format: &mut TextFormat,
        width: u16,
        current_item: bool,
        color: &MenuTheme,
    ) {
        super::utils::update_format_with_caption(
            &self.caption,
            format,
            self.enabled,
            current_item,
            color,
        );
        if current_item && self.enabled {
            // highlight current item
            surface.fill_horizontal_line_with_size(
                1,
                format.y,
                width as u32,
                Character::with_attributes(' ', color.text.hovered),
            );
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
