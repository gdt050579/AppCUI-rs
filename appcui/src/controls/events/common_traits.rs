use crate::graphics::*;
use crate::input::*;
use crate::system::Theme;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum EventProcessStatus {
    Processed,
    Ignored,
    Update,
    Cancel
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
pub trait Control: OnPaint + OnKeyPressed + OnMouseEvent + OnDefaultAction + OnResize {}
