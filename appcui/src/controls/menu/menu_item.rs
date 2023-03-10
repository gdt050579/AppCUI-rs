use crate::{utils::Caption, input::Key};

use super::{menu::Menu, menu_item_type::MenuItemType};

pub struct MenuItem {
    pub(super) checked: bool,
    pub(super) enabled: bool,
    pub(super) commandID: u32,
    pub(super) caption: Caption,
    pub(super) shortcut: Key,
    pub(super) menu_type: MenuItemType,
    pub(super) submenu: Option<Box<Menu>>
}
