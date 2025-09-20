use super::Button;
use super::ItemBase;
use super::Label;
use super::MenuButton;
use super::Separator;
use super::ToggleButton;
use super::SwitchButton;
use crate::graphics::Surface;
use crate::input::*;
use crate::system::MenuHandleManager;
use crate::{
    system::{Handle, HandleSupport, Theme},
    ui::appbar::ItemStatus,
};

pub(crate) enum AppBarItem {
    Separator(Separator),
    MenuButton(MenuButton),
    Label(Label),
    Button(Button),
    ToggleButton(ToggleButton),
    SwitchButton(SwitchButton),
}

impl From<super::MenuButton> for AppBarItem {
    fn from(value: super::MenuButton) -> Self {
        AppBarItem::MenuButton(value)
    }
}
impl From<super::Separator> for AppBarItem {
    fn from(value: super::Separator) -> Self {
        AppBarItem::Separator(value)
    }
}
impl From<super::Label> for AppBarItem {
    fn from(value: super::Label) -> Self {
        AppBarItem::Label(value)
    }
}
impl From<super::Button> for AppBarItem {
    fn from(value: super::Button) -> Self {
        AppBarItem::Button(value)
    }
}
impl From<super::ToggleButton> for AppBarItem {
    fn from(value: super::ToggleButton) -> Self {
        AppBarItem::ToggleButton(value)
    }
}
impl From<super::SwitchButton> for AppBarItem {
    fn from(value: super::SwitchButton) -> Self {
        AppBarItem::SwitchButton(value)
    }
}

impl AppBarItem {
    pub(super) fn base(&self) -> &ItemBase {
        match self {
            AppBarItem::Separator(obj) => &obj.base,
            AppBarItem::MenuButton(obj) => &obj.base,
            AppBarItem::Label(obj) => &obj.base,
            AppBarItem::Button(obj) => &obj.base,
            AppBarItem::ToggleButton(obj) => &obj.base,
            AppBarItem::SwitchButton(obj) => &obj.base,
        }
    }
    pub(super) fn base_mut(&mut self) -> &mut ItemBase {
        match self {
            AppBarItem::Separator(obj) => &mut obj.base,
            AppBarItem::MenuButton(obj) => &mut obj.base,
            AppBarItem::Label(obj) => &mut obj.base,
            AppBarItem::Button(obj) => &mut obj.base,
            AppBarItem::ToggleButton(obj) => &mut obj.base,
            AppBarItem::SwitchButton(obj) => &mut obj.base,
        }
    }
    #[inline(always)]
    pub(super) fn is_enabled(&self) -> bool {
        self.base().is_enabled()
    }
    #[inline(always)]
    pub(super) fn hotkey(&self) -> Key {
        match self {
            AppBarItem::Separator(_) => Key::None,
            AppBarItem::MenuButton(obj) => obj.hotkey(),
            AppBarItem::Label(_) => Key::None,
            AppBarItem::Button(but) => but.hotkey(),
            AppBarItem::ToggleButton(but) => but.hotkey(),
            AppBarItem::SwitchButton(but) => but.hotkey(),
        }
    }
    #[inline(always)]
    pub(super) fn tooltip(&self) -> Option<&str> {
        let result = match self {
            AppBarItem::Separator(_) => "",
            AppBarItem::MenuButton(_) => "",
            AppBarItem::Label(obj) => obj.tooltip(),
            AppBarItem::Button(obj) => obj.tooltip(),
            AppBarItem::ToggleButton(obj) => obj.tooltip(),
            AppBarItem::SwitchButton(obj) => obj.tooltip(),
        };
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
    #[inline(always)]
    pub(super) fn is_menu(&self) -> bool {
        matches!(self, AppBarItem::MenuButton(_))
    }
    #[inline(always)]
    pub(super) fn process_shortcut(&self, key: Key, menus: &mut MenuHandleManager) -> bool {
        match self {
            AppBarItem::Separator(_) => false,
            AppBarItem::MenuButton(obj) => obj.process_shortcut(key, menus),
            AppBarItem::Label(_) => false,
            AppBarItem::Button(_) => false,
            AppBarItem::ToggleButton(_) => false,
            AppBarItem::SwitchButton(_) => false,
        }
    }
    pub(super) fn activate(&mut self) {
        match self {
            AppBarItem::Separator(_) => {}
            AppBarItem::MenuButton(obj) => obj.on_activate(),
            AppBarItem::Label(_) => {}
            AppBarItem::Button(_) => {}
            AppBarItem::ToggleButton(_) => {}
            AppBarItem::SwitchButton(_) => {}
        }
    }
    pub(super) fn execute_action(&mut self) {
        match self {
            AppBarItem::Separator(_) => {}
            AppBarItem::MenuButton(_) => {}
            AppBarItem::Label(_) => {}
            AppBarItem::Button(obj) => obj.on_execute(),
            AppBarItem::ToggleButton(obj) => obj.on_execute(),
            AppBarItem::SwitchButton(obj) => obj.on_execute(),
        }
    }
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, status: ItemStatus) {
        match self {
            AppBarItem::Separator(obj) => obj.paint(surface, theme),
            AppBarItem::MenuButton(obj) => obj.paint(surface, theme, status),
            AppBarItem::Label(obj) => obj.paint(surface, theme),
            AppBarItem::Button(obj) => obj.paint(surface, theme, status),
            AppBarItem::ToggleButton(obj) => obj.paint(surface, theme, status),
            AppBarItem::SwitchButton(obj) => obj.paint(surface, theme, status),
        }
    }
    pub(super) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        match self {
            AppBarItem::Separator(_) => {}
            AppBarItem::MenuButton(obj) => obj.set_receiver_control_handle(handle),
            AppBarItem::Label(_) => {}
            AppBarItem::Button(obj) => obj.set_receiver_control_handle(handle),
            AppBarItem::ToggleButton(obj) => obj.set_receiver_control_handle(handle),
            AppBarItem::SwitchButton(obj) => obj.set_receiver_control_handle(handle),
        }
    }
}

impl HandleSupport<AppBarItem> for AppBarItem {
    fn handle(&self) -> Handle<AppBarItem> {
        self.base().handle()
    }

    fn set_handle(&mut self, handle: Handle<AppBarItem>) {
        self.base_mut().update_handle(handle);
    }
}
