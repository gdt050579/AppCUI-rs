use crate::graphics::*;
use crate::input::*;

pub trait OnPaint {
    fn on_paint(&self, surface: &Surface);
}

pub trait OnKeyPressed {
    fn on_key_pressed(&mut self, key: Key, character: char);
}