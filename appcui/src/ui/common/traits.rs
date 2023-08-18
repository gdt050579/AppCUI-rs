use crate::{
    ui::{
        button::events::ButtonEvents, 
        command_bar::events::CommandBarEvents,
        menu::events::MenuEvents, checkbox::events::CheckBoxEvents,
    },
    graphics::{Size, Surface},
    input::{Key, MouseEvent},
    system::Theme,
};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum EventProcessStatus {
    Processed,
    Ignored,
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

pub trait Control:
    OnPaint
    + OnKeyPressed
    + OnMouseEvent
    + OnDefaultAction
    + OnResize
    + OnFocus
    /* events from each control */
    + ButtonEvents
    + CheckBoxEvents
    + CommandBarEvents
    + MenuEvents
{
}

pub trait DesktopControl {}
pub trait WindowControl {}
