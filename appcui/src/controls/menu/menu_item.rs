use crate::{
    graphics::{Character, LineType, SpecialChar, Surface, TextFormat},
    system::MenuTheme,
};

use super::{MenuCheckBoxItem, MenuCommandItem, MenuLineItem, MenuRadioBoxItem};

pub enum MenuItem {
    Command(MenuCommandItem),
    CheckBox(MenuCheckBoxItem),
    RadioBox(MenuRadioBoxItem),
    Line(MenuLineItem),
}

impl MenuItem {
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
            MenuItem::Line(item) => item.paint(surface, format, width),
        }
    }
}
