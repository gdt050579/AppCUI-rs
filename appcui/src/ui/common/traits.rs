use crate::{
    graphics::{Size, Surface},
    input::{Key, MouseEvent},
    system::Theme,
    ui::{
        button::events::ButtonEvents, 
        checkbox::events::CheckBoxEvents, 
        command_bar::events::CommandBarEvents, 
        desktop::events::DesktopEvents,
        menu::events::MenuEvents, 
        window::events::WindowEvents,
        window::events::ToolBarEvents,
    }, prelude::colorpicker::events::ColorPickerEvents
};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum EventProcessStatus {
    Processed,
    Ignored,
}
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum ActionRequest {
    Allow,
    Deny,
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

pub trait OnWindowRegistered {
    fn on_registered(&mut self) {}
}

pub trait Control:
    OnPaint
    + OnKeyPressed
    + OnMouseEvent
    + OnDefaultAction
    + OnResize
    + OnFocus
    + OnWindowRegistered
    /* events from each control */
    + ButtonEvents
    + CheckBoxEvents
    + ColorPickerEvents
    + CommandBarEvents
    + WindowEvents
    + ToolBarEvents
    + DesktopEvents
    + MenuEvents
{
}

pub trait DesktopControl {}
pub trait WindowControl {}
pub trait NotWindow {}
pub trait NotDesktop {}
pub trait NotModalWindow {}
