use crate::{
    graphics::{Surface, TextFormat},
    input::{Key, KeyCode},
    system::MenuTheme,
    utils::Caption,
};

pub(super) struct MenuCheckBoxItem {
    pub(super) enabled: bool,
    pub(super) checked: bool,
    pub(super) commandID: u32,
    pub(super) caption: Caption,
    pub(super) shortcut: Key,
}