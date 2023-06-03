use crate::controls::menu::Menu;
use crate::controls::menu::MenuBar;
use crate::system::CommandBar;
use crate::graphics::*;
use crate::input::*;
use crate::system::Theme;

use super::Event;
use super::MenuEvent;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum EventProcessStatus {
    Processed,
    Ignored,
    Update,
    Cancel,
}
impl EventProcessStatus {
    pub(crate) fn is_processed_or_update(&self) -> bool {
        match self {
            EventProcessStatus::Processed => true,
            EventProcessStatus::Update => true,
            _ => false, }
    }
}

pub trait OnPaint {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
pub trait OnKeyPressed {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
pub trait OnMouseEvent {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
pub trait OnDefaultAction {
    fn on_default_action(&mut self) {}
}

pub trait OnResize {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}

pub trait OnFocus {
    fn on_focus(&mut self) {}
    fn on_lose_focus(&mut self) {}
}

pub trait OnEvent {
    fn on_event(&mut self, _event: Event) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

pub trait OnMenuEvents {
	fn on_menu_open(&self, _menu: &mut Menu) {}
	fn on_event(&self, _event: MenuEvent) {}
	fn on_update_menubar(&self, _menubar: &mut MenuBar) {}
}


pub trait CommandBarEvents {
    fn on_update_commandbar(&self, _commandbar: &mut CommandBar) {}
    fn on_event(&mut self, _command_id: u32) {}
}


pub trait Control:
    OnPaint + OnKeyPressed + OnMouseEvent + OnDefaultAction + OnResize + OnFocus + OnEvent + CommandBarEvents + OnMenuEvents
{
}
