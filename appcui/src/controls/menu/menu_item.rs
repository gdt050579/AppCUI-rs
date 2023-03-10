use crate::{utils::Caption, input::Key};

pub struct MenuItem {
    checked: bool,
    enabled: bool,
    commandID: u32,
    caption: Caption,
    shortcut: Key,
    // type
    // submenu
}