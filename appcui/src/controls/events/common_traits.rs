use crate::graphics::*;
use crate::input::*;
use crate::system::Theme;

pub trait OnPaint {
    fn on_paint(&self, surface: &Surface, theme: &Theme);
}

pub trait OnKeyPressed {
    fn on_key_pressed(&mut self, key: Key, character: char);
}