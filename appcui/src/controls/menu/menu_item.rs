use crate::{utils::Caption, input::Key};

use super::{menu::Menu, menu_item_type::MenuItemType};

pub struct MenuItem {
    checked: bool,
    enabled: bool,
    commandID: u32,
    caption: Caption,
    shortcut: Key,
    menu_type: MenuItemType,
    submenu: Option<Box<Menu>>
}
