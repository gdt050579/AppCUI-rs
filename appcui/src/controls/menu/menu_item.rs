use crate::{
    graphics::{CharAttribute, Character, LineType, SpecialChar, Surface, TextFormat},
    input::{Key, KeyCode},
    system::{MenuTheme},
    utils::Caption,
};

use super::{menu::Menu, menu_item_type::MenuItemType, MenuCheckBoxItem, MenuCommandItem, MenuRadioBoxItem};

pub enum MenuItem {
    Command(MenuCommandItem),
    CheckBox(MenuCheckBoxItem),
    RadioBox(MenuRadioBoxItem),
}

impl MenuItem {

    #[inline(always)]
    fn paint_line(&self, surface: &mut Surface, format: &TextFormat, width: u16) {
        surface.draw_horizontal_line_with_size(
            1,
            format.y,
            width as u32,
            LineType::Single,
            format.char_attr,
        );
    }

    #[inline(always)]
    fn paint_submenu(&self, surface: &mut Surface, format: &mut TextFormat, width: u16) {
        format.x = 2;
        surface.write_text(self.caption.get_text(), format);
        surface.write_char(
            1 + width as i32,
            format.y,
            Character::with_attributes(SpecialChar::TriangleRight, format.char_attr),
        );
    }
    pub(super) fn paint(
        &self,
        surface: &mut Surface,
        format: &mut TextFormat,
        width: u16,
        current_item: bool,
        color: &MenuTheme,
    ) {

        match self {
            MenuItem::Command(item) => item.paint(surface, format, width, current_item, color),
            MenuItem::CheckBox(item) => item.paint(surface, format, width, current_item, color),
            MenuItem::RadioBox(item) => item.paint(surface, format, width, current_item, color),
        }
    }
}
