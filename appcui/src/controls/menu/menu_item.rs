use crate::{
    graphics::{CharAttribute, Character, LineType, SpecialChar, Surface, TextFormat},
    input::{Key, KeyCode},
    system::{MenuTheme, Theme},
    utils::Caption,
};

use super::{menu::Menu, menu_item_type::MenuItemType};

pub struct MenuItem {
    pub(super) checked: bool,
    pub(super) enabled: bool,
    pub(super) commandID: u32,
    pub(super) caption: Caption,
    pub(super) shortcut: Key,
    pub(super) item_type: MenuItemType,
    pub(super) submenu: Option<Box<Menu>>,
}

impl MenuItem {
    #[inline(always)]
    fn get_text_attr(&self, current_item: bool, color: &MenuTheme) -> CharAttribute {
        match () {
            _ if self.enabled == false => color.text.inactive,
            _ if current_item => color.text.hovered,
            _ => color.text.normal,
        }
    }
    #[inline(always)]
    fn get_hotkey_attr(&self, current_item: bool, color: &MenuTheme) -> CharAttribute {
        match () {
            _ if self.enabled == false => color.hotkey.inactive,
            _ if current_item => color.hotkey.hovered,
            _ => color.hotkey.normal,
        }
    }
    #[inline(always)]
    fn get_shortcut_attr(&self, current_item: bool, color: &MenuTheme) -> CharAttribute {
        match () {
            _ if self.enabled == false => color.shortcut.inactive,
            _ if current_item => color.shortcut.hovered,
            _ => color.shortcut.normal,
        }
    }
    #[inline(always)]
    fn get_symbol_attr(&self, current_item: bool, color: &MenuTheme) -> CharAttribute {
        match () {
            _ if self.enabled == false => color.symbol.inactive,
            _ if current_item => color.symbol.hovered,
            _ => color.symbol.normal,
        }
    }

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
    fn paint_command(&self, surface: &mut Surface, format: &mut TextFormat) {
        format.x = 2;
        surface.write_text(self.caption.get_text(), format);
    }
    #[inline(always)]
    fn paint_check(
        &self,
        surface: &mut Surface,
        format: &mut TextFormat,
        current_item: bool,
        color: &MenuTheme,
    ) {
        format.x = 4;
        surface.write_text(self.caption.get_text(), format);
        if self.checked {
            let attr = self.get_symbol_attr(current_item, color);
            surface.write_char(
                2,
                format.y,
                Character::with_attributes(SpecialChar::CheckMark, attr),
            );
        }
    }
    #[inline(always)]
    fn paint_radio(
        &self,
        surface: &mut Surface,
        format: &mut TextFormat,
        current_item: bool,
        color: &MenuTheme,
    ) {
        format.x = 4;
        surface.write_text(self.caption.get_text(), format);
        let attr = self.get_symbol_attr(current_item, color);
        let symbol = if self.checked {
            SpecialChar::CircleFilled
        } else {
            SpecialChar::CircleEmpty
        };
        surface.write_char(2, format.y, Character::with_attributes(symbol, attr));
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
        format.char_attr = self.get_text_attr(current_item, color);
        format.hotkey_pos = self.caption.get_hotkey_pos();
        if self.caption.has_hotkey() {
            format.hotkey_attr = Some(self.get_hotkey_attr(current_item, color));
        }
        format.chars_count = Some(self.caption.get_chars_count() as u16);
        if current_item && self.enabled {
            // highlight current item
            surface.fill_horizontal_line_with_size(
                1,
                format.y,
                width as u32,
                Character::with_attributes(' ', color.text.hovered),
            );
        }

        match self.item_type {
            MenuItemType::Command => self.paint_command(surface, format),
            MenuItemType::Check => self.paint_check(surface, format, current_item, color),
            MenuItemType::Radio => self.paint_check(surface, format, current_item, color),
            MenuItemType::Line => self.paint_line(surface, format, width),
            MenuItemType::SubMenu => self.paint_submenu(surface, format, width),
        }

        if self.shortcut.code != KeyCode::None {
            let name = self.shortcut.code.get_name();
            let modifier_name = self.shortcut.modifier.get_name();
            let attr = self.get_shortcut_attr(current_item, color);
            let x = (width as i32) - modifier_name.len() as i32;
            surface.write_string(x, format.y, modifier_name, attr, false);
            surface.write_string(
                x + (modifier_name.len() as i32),
                format.y,
                name,
                attr,
                false,
            );
        }
    }
}
