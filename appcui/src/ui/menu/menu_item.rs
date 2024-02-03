use crate::{
    graphics::{Surface, TextFormat},
    input::{Key, KeyCode},
    system::{MenuTheme, Handle},
};

use super::{
    CheckBox, Command, Separator, SingleChoice,
    SubMenu, Menu,
};

pub(super) trait IntoMenuItem {
    fn into_menuitem(self)->MenuItem;
}

pub(super) enum MenuItem {
    Command(Command),
    CheckBox(CheckBox),
    SingleChoice(SingleChoice),
    Separator(Separator),
    SubMenu(SubMenu),
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
            MenuItem::SingleChoice(item) => item.paint(surface, format, width, current_item, color),
            MenuItem::SubMenu(item) => item.paint(surface, format, width, current_item, color),
            MenuItem::Separator(item) => item.paint(surface, format.y, width, color),
        }
    }
    #[inline(always)]
    pub(super) fn is_enabled(&self) -> bool {
        match self {
            MenuItem::Command(item) => item.enabled,
            MenuItem::CheckBox(item) => item.enabled,
            MenuItem::SingleChoice(item) => item.enabled,
            MenuItem::Separator(_) => true,
            MenuItem::SubMenu(item) => item.enabled,
        }
    }
    #[inline(always)]
    pub(super) fn is_line(&self) -> bool {
        match self {
            MenuItem::Separator(_) => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub(super) fn is_radiobox(&self) -> bool {
        match self {
            MenuItem::SingleChoice(_) => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub(super) fn can_be_selected(&self) -> bool {
        match self {
            MenuItem::Separator(_) => false,
            _ => true,
        }
    }
    #[inline(always)]
    pub(super) fn is_checkable(&self) -> bool {
        match self {
            MenuItem::CheckBox(_) => true,
            MenuItem::SingleChoice(_) => true,
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
            MenuItem::Command(item) => Some(item.command_id),
            MenuItem::CheckBox(item) => Some(item.command_id),
            MenuItem::SingleChoice(item) => Some(item.command_id),
            MenuItem::Separator(_) => None,
            MenuItem::SubMenu(_) => None,
        }
    }
    #[inline(always)]
    pub(super) fn get_shortcut(&self) -> Option<Key> {
        let key = match self {
            MenuItem::Command(item) => item.shortcut,
            MenuItem::CheckBox(item) => item.shortcut,
            MenuItem::SingleChoice(item) => item.shortcut,
            MenuItem::Separator(_) => Key::default(),
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
            MenuItem::SingleChoice(item) => item.caption.get_hotkey(),
            MenuItem::Separator(_) => Key::default(),
            MenuItem::SubMenu(item) => item.caption.get_hotkey(),
        };
        if key.code != KeyCode::None {
            return Some(key);
        } else {
            None
        }
    }
    #[inline(always)]
    pub(super) fn set_checked(&mut self, value: bool) {
        match self {
            MenuItem::CheckBox(item) => item.checked = value,
            MenuItem::SingleChoice(item) => item.selected = value,
            _ => {}
        }
    }
    #[inline(always)]
    pub(super) fn get_caption_chars_count(&self) -> usize {
        match self {
            MenuItem::Command(item) => item.caption.get_chars_count(),
            MenuItem::CheckBox(item) => item.caption.get_chars_count(),
            MenuItem::SingleChoice(item) => item.caption.get_chars_count(),
            MenuItem::Separator(_) => 0,
            MenuItem::SubMenu(item) => item.caption.get_chars_count(),
        }
    }
    #[inline(always)]
    pub(super) fn get_submenu(&self) -> Option<Handle<Menu>> {
        match self {
            MenuItem::SubMenu(item) => Some(item.submenu_handle),
            _ => None,
        }
    }
}
