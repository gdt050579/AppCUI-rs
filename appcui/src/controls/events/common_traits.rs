use crate::controls::BasicControl;
use crate::graphics::*;
use crate::input::*;
use crate::system::Theme;

pub trait OnPaint {
    fn on_paint(&self, _surface: &Surface, _theme: &Theme) {}
}

pub trait OnKeyPressed {
    fn on_key_pressed(&mut self, _key: Key, _character: char) {}
}
pub trait Control: OnPaint + OnKeyPressed {
    fn get_basic_control(&self) -> &BasicControl;
    fn get_mut_basic_control(&mut self)-> &mut BasicControl;
}