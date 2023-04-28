use crate::{
    graphics::{Surface, TextFormat},
    input::{Key, KeyCode},
    system::MenuTheme,
    utils::Caption,
};

pub(super) struct MenuCommandItem {
    pub(super) enabled: bool,
    pub(super) commandID: u32,
    pub(super) caption: Caption,
    pub(super) shortcut: Key,
}
impl MenuCommandItem {
    fn paint(
        &self,
        surface: &mut Surface,
        format: &mut TextFormat,
        width: u16,
        current_item: bool,
        color: &MenuTheme,
    ) {
        super::utils::update_format_with_caption(&self.caption, format, self.enabled, current_item, color);
        if self.shortcut.code != KeyCode::None {
            super::utils::paint_shortcut(
                self.shortcut,
                surface,
                format,
                width,
                self.enabled,
                current_item,
                color,
            );
        }
    }
}
