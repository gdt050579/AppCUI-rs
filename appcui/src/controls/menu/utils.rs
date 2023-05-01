use crate::{
    graphics::{CharAttribute, Surface, TextFormat},
    input::Key,
    system::MenuTheme,
    utils::Caption,
};

#[inline(always)]
pub(super) fn get_text_attr(enabled: bool, current_item: bool, color: &MenuTheme) -> CharAttribute {
    match () {
        _ if enabled == false => color.text.inactive,
        _ if current_item => color.text.hovered,
        _ => color.text.normal,
    }
}
#[inline(always)]
pub(super) fn get_hotkey_attr(
    enabled: bool,
    current_item: bool,
    color: &MenuTheme,
) -> CharAttribute {
    match () {
        _ if enabled == false => color.hotkey.inactive,
        _ if current_item => color.hotkey.hovered,
        _ => color.hotkey.normal,
    }
}
#[inline(always)]
pub(super) fn get_shortcut_attr(
    enabled: bool,
    current_item: bool,
    color: &MenuTheme,
) -> CharAttribute {
    match () {
        _ if enabled == false => color.shortcut.inactive,
        _ if current_item => color.shortcut.hovered,
        _ => color.shortcut.normal,
    }
}
#[inline(always)]
pub(super) fn get_symbol_attr(
    enabled: bool,
    current_item: bool,
    color: &MenuTheme,
) -> CharAttribute {
    match () {
        _ if enabled == false => color.symbol.inactive,
        _ if current_item => color.symbol.hovered,
        _ => color.symbol.normal,
    }
}

#[inline(always)]
pub(super) fn paint_shortcut(
    shortcut: Key,
    surface: &mut Surface,
    format: &mut TextFormat,
    width: u16,
    enabled: bool,
    current_item: bool,
    color: &MenuTheme,
) {
    let name = shortcut.code.get_name();
    let modifier_name = shortcut.modifier.get_name();
    let sz = name.len() + modifier_name.len();
    let attr = get_shortcut_attr(enabled, current_item, color);
    let x = (width as i32) - (sz as i32);
    if modifier_name.len() > 0 {
        surface.write_string(x, format.y, modifier_name, attr, false);
    }
    surface.write_string(
        x + (modifier_name.len() as i32),
        format.y,
        name,
        attr,
        false,
    );
}

pub(super) fn update_format_with_caption(
    caption: &Caption,
    format: &mut TextFormat,
    enabled: bool,
    current_item: bool,
    color: &MenuTheme,
) {
    format.char_attr = get_text_attr(enabled, current_item, color);
    format.hotkey_pos = caption.get_hotkey_pos();
    if caption.has_hotkey() {
        format.hotkey_attr = Some(get_hotkey_attr(enabled, current_item, color));
    }
    format.chars_count = Some(caption.get_chars_count() as u16);
}
