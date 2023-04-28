use crate::{
    graphics::{Surface, TextFormat},
    system::MenuTheme,
};

use super::{MenuCheckBoxItem, MenuCommandItem, MenuLineItem, MenuRadioBoxItem, MenuSubMenuItem};

pub enum MenuItem {
    Command(MenuCommandItem),
    CheckBox(MenuCheckBoxItem),
    RadioBox(MenuRadioBoxItem),
    Line(MenuLineItem),
    SubMenu(MenuSubMenuItem),
}

impl MenuItem {
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
            MenuItem::SubMenu(item) => item.paint(surface, format, width, current_item, color),
            MenuItem::Line(item) => item.paint(surface, format, width),
        }
    }
    #[inline(always)]
    pub(super) fn is_enabled(&self) -> bool {
        match self {
            MenuItem::Command(item) => item.enabled,
            MenuItem::CheckBox(item) => item.enabled,
            MenuItem::RadioBox(item) => item.enabled,
            MenuItem::Line(_) => true,
            MenuItem::SubMenu(item) => item.enabled,
        }
    }
    #[inline(always)]
    pub(super) fn is_line(&self)-> bool {
        match self {
            MenuItem::Line(_) => true,
            _ => false,
        }
    }
}
