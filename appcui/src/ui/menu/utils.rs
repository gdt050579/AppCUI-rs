use crate::{
    graphics::{CharAttribute, Surface},
    input::Key,
    prelude::TextFormat,
    system::MenuTheme,
    utils::Caption,
};

#[inline(always)]
pub(super) fn get_text_attr(enabled: bool, current_item: bool, color: &MenuTheme) -> CharAttribute {
    match () {
        _ if !enabled => color.text.inactive,
        _ if current_item => color.text.hovered,
        _ => color.text.normal,
    }
}
#[inline(always)]
pub(super) fn get_hotkey_attr(enabled: bool, current_item: bool, color: &MenuTheme) -> CharAttribute {
    match () {
        _ if !enabled => color.hotkey.inactive,
        _ if current_item => color.hotkey.hovered,
        _ => color.hotkey.normal,
    }
}
#[inline(always)]
pub(super) fn get_shortcut_attr(enabled: bool, current_item: bool, color: &MenuTheme) -> CharAttribute {
    match () {
        _ if !enabled => color.shortcut.inactive,
        _ if current_item => color.shortcut.hovered,
        _ => color.shortcut.normal,
    }
}
#[inline(always)]
pub(super) fn get_symbol_attr(enabled: bool, current_item: bool, color: &MenuTheme) -> CharAttribute {
    match () {
        _ if !enabled => color.symbol.inactive,
        _ if current_item => color.symbol.hovered,
        _ => color.symbol.normal,
    }
}

#[inline(always)]
pub(super) fn paint_shortcut(shortcut: Key, surface: &mut Surface, y: i32, width: u16, enabled: bool, current_item: bool, color: &MenuTheme) {
    let name = shortcut.code.name();
    let modifier_name = shortcut.modifier.name();
    let sz = name.len() + modifier_name.len();
    let attr = get_shortcut_attr(enabled, current_item, color);
    let x = (width as i32) - (sz as i32);
    if !modifier_name.is_empty() {
        surface.write_string(x, y, modifier_name, attr, false);
    }
    surface.write_string(x + (modifier_name.len() as i32), y, name, attr, false);
}

#[inline(always)]
pub(super) fn update_format_with_caption(caption: &Caption, format: &mut TextFormat, enabled: bool, current_item: bool, color: &MenuTheme) {
    format.set_hotkey_from_caption(get_hotkey_attr(enabled, current_item, color), caption);
    format.set_attribute(get_text_attr(enabled, current_item, color));
    format.set_chars_count(caption.chars_count() as u16);
}
