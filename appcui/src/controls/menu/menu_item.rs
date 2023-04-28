use crate::{
    graphics::{Surface, TextFormat},
    input::{Key, KeyCode},
    system::MenuTheme,
};

use super::{MenuCheckBoxItem, MenuCommandItem, MenuLineItem, MenuRadioBoxItem, MenuSubMenuItem, Menu};

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
    pub(super) fn is_line(&self) -> bool {
        match self {
            MenuItem::Line(_) => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub(super) fn is_radiobox(&self) -> bool {
        match self {
            MenuItem::RadioBox(_) => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub(super) fn can_be_selected(&self) -> bool {
        match self {
            MenuItem::Line(_) => false,
            _ => true,
        }
    }
    #[inline(always)]
    pub(super) fn is_checkable(&self) -> bool {
        match self {
            MenuItem::CheckBox(_) => true,
            MenuItem::RadioBox(_) => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub(super) fn is_submenu(&self) -> bool {
        match self {
            MenuItem::SubMenu(_) => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub(super) fn get_command(&self) -> Option<u32> {
        match self {
            MenuItem::Command(item) => Some(item.commandID),
            MenuItem::CheckBox(item) => Some(item.commandID),
            MenuItem::RadioBox(item) => Some(item.commandID),
            MenuItem::Line(_) => None,
            MenuItem::SubMenu(_) => None,
        }
    }
    #[inline(always)]
    pub(super) fn get_shortcut(&self) -> Option<Key> {
        let key = match self {
            MenuItem::Command(item) => item.shortcut,
            MenuItem::CheckBox(item) => item.shortcut,
            MenuItem::RadioBox(item) => item.shortcut,
            MenuItem::Line(_) => Key::default(),
            MenuItem::SubMenu(_) => Key::default(),
        };
        if key.code != KeyCode::None {
            return Some(key);
        } else {
            None
        }
    }
    #[inline(always)]
    pub(super) fn get_hotkey(&self) -> Option<Key> {
        let key = match self {
            MenuItem::Command(item) => item.caption.get_hotkey(),
            MenuItem::CheckBox(item) => item.caption.get_hotkey(),
            MenuItem::RadioBox(item) => item.caption.get_hotkey(),
            MenuItem::Line(_) => Key::default(),
            MenuItem::SubMenu(item) => item.caption.get_hotkey(),
        };
        if key.code != KeyCode::None {
            return Some(key);
        } else {
            None
        }
    }
    #[inline(always)]
    pub(super) fn set_checked(&self, value: bool) {
        match self {
            MenuItem::CheckBox(item) => item.checked = value,
            MenuItem::RadioBox(item) => item.checked = value,
            _ => {}
        }
    }
    #[inline(always)]
    pub(super) fn get_caption_chars_count(&self)->usize {
        match self {
            MenuItem::Command(item) => item.caption.get_chars_count(),
            MenuItem::CheckBox(item) => item.caption.get_chars_count(),
            MenuItem::RadioBox(item) => item.caption.get_chars_count(),
            MenuItem::Line(_) => 0,
            MenuItem::SubMenu(item) => item.caption.get_chars_count(),
        }
    }
    #[inline(always)]
    pub(super) fn get_submenu(&self) -> Option<&Menu> {
        match self {
            MenuItem::SubMenu(item) => { 
                if let Some(menu) = item.submenu.as_ref() {
                    Some(menu)
                } else {
                    None
                }
             }
            _ => None,
        }
    }
}
