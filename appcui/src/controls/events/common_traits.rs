use crate::graphics::*;
use crate::input::*;
use crate::system::Theme;

pub trait OnPaint {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
pub trait OnKeyPressed {
    fn on_key_pressed(&mut self, _key: Key, _character: char) {}
}
pub trait OnMouseEvent {
    fn on_mouse_event(&mut self, _event: &MouseEvent) {}
}
pub trait Control: OnPaint + OnKeyPressed + OnMouseEvent {
}