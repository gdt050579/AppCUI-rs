use crate::graphics::*;
use crate::input::*;
use crate::system::Theme;

pub trait OnPaint {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
#[repr(u8)]
#[derive(Copy,Clone)]
pub enum KeyPressedResult {
    Processed, Ignored
}
pub trait OnKeyPressed {
    fn on_key_pressed(&mut self, _key: Key, _character: char)->KeyPressedResult { KeyPressedResult::Ignored }
}
pub trait OnMouseEvent {
    fn on_mouse_event(&mut self, _event: &MouseEvent) {}
}
pub trait OnDefaultAction {
    fn on_default_action(&mut self) {}
}
pub trait Control: OnPaint + OnKeyPressed + OnMouseEvent + OnDefaultAction {
}