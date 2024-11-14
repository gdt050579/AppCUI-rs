use crate::{
    graphics::{Surface, TextFormat},
    input::{Key, KeyCode},
    system::{Handle, MenuTheme}, prelude::common::UIElement,
};

use super::{CheckBox, Command, Menu, Separator, SingleChoice, SubMenu};

pub(crate) trait MenuItem {
    fn into_menuitem(self) -> MenuItemWrapper;
    fn update_handles(&mut self, parent: Handle<Menu>, me: Handle<UIElement>);
}

pub(crate) enum MenuItemWrapper {
    Command(Command),
    CheckBox(CheckBox),
    SingleChoice(SingleChoice),
    Separator(Separator),
    SubMenu(SubMenu),
}

impl MenuItemWrapper {
    pub(super) fn paint(&self, surface: &mut Surface, format: &mut TextFormat, width: u16, current_item: bool, color: &MenuTheme) {
        match self {
            MenuItemWrapper::Command(item) => item.paint(surface, format, width, current_item, color),
            MenuItemWrapper::CheckBox(item) => item.paint(surface, format, width, current_item, color),
            MenuItemWrapper::SingleChoice(item) => item.paint(surface, format, width, current_item, color),
            MenuItemWrapper::SubMenu(item) => item.paint(surface, format, width, current_item, color),
            MenuItemWrapper::Separator(item) => item.paint(surface, format.y, width, color),
        }
    }
    #[inline(always)]
    pub(super) fn is_enabled(&self) -> bool {
        match self {
            MenuItemWrapper::Command(item) => item.enabled,
            MenuItemWrapper::CheckBox(item) => item.enabled,
            MenuItemWrapper::SingleChoice(item) => item.enabled,
            MenuItemWrapper::Separator(_) => true,
            MenuItemWrapper::SubMenu(item) => item.enabled,
        }
    }
    #[inline(always)]
    pub(super) fn is_line(&self) -> bool {
        matches!(self, MenuItemWrapper::Separator(_))
    }
    #[inline(always)]
    pub(super) fn is_singlechoice(&self) -> bool {
        matches!(self, MenuItemWrapper::SingleChoice(_))
    }
    #[inline(always)]
    pub(super) fn can_be_selected(&self) -> bool {
        !matches!(self, MenuItemWrapper::Separator(_))
    }
    #[inline(always)]
    pub(super) fn is_checkable(&self) -> bool {
        matches!(self, MenuItemWrapper::CheckBox(_) | MenuItemWrapper::SingleChoice(_))
    }
    #[inline(always)]
    pub(super) fn is_submenu(&self) -> bool {
        matches!(self, MenuItemWrapper::SubMenu(_))
    }
    #[inline(always)]
    pub(super) fn shortcut(&self) -> Option<Key> {
        let key = match self {
            MenuItemWrapper::Command(item) => item.shortcut,
            MenuItemWrapper::CheckBox(item) => item.shortcut,
            MenuItemWrapper::SingleChoice(item) => item.shortcut,
            MenuItemWrapper::Separator(_) => Key::default(),
            MenuItemWrapper::SubMenu(_) => Key::default(),
        };
        if key.code != KeyCode::None {
            Some(key)
        } else {
            None
        }
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Option<Key> {
        let key = match self {
            MenuItemWrapper::Command(item) => item.caption.hotkey(),
            MenuItemWrapper::CheckBox(item) => item.caption.hotkey(),
            MenuItemWrapper::SingleChoice(item) => item.caption.hotkey(),
            MenuItemWrapper::Separator(_) => Key::default(),
            MenuItemWrapper::SubMenu(item) => item.caption.hotkey(),
        };
        if key.code != KeyCode::None {
            Some(key)
        } else {
            None
        }
    }
    #[inline(always)]
    pub(super) fn set_checked(&mut self, value: bool) {
        match self {
            MenuItemWrapper::CheckBox(item) => item.checked = value,
            MenuItemWrapper::SingleChoice(item) => item.selected = value,
            _ => {}
        }
    }
    #[inline(always)]
    pub(super) fn get_caption_chars_count(&self) -> usize {
        match self {
            MenuItemWrapper::Command(item) => item.caption.chars_count(),
            MenuItemWrapper::CheckBox(item) => item.caption.chars_count(),
            MenuItemWrapper::SingleChoice(item) => item.caption.chars_count(),
            MenuItemWrapper::Separator(_) => 0,
            MenuItemWrapper::SubMenu(item) => item.caption.chars_count(),
        }
    }
    #[inline(always)]
    pub(super) fn get_submenu(&self) -> Option<Handle<Menu>> {
        match self {
            MenuItemWrapper::SubMenu(item) => Some(item.submenu_handle),
            _ => None,
        }
    }
    #[inline(always)]
    pub(super) fn get_handle(&self) -> Handle<UIElement> {
        match self {
            MenuItemWrapper::Command(item) => item.handle.cast(),
            MenuItemWrapper::CheckBox(item) => item.handle.cast(),
            MenuItemWrapper::SingleChoice(item) => item.handle.cast(),
            MenuItemWrapper::Separator(item) => item.handle.cast(),
            MenuItemWrapper::SubMenu(item) => item.handle.cast(),
        }
    }
}
