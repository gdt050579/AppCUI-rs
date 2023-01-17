use crate::appcui::graphics::*;
use crate::appcui::input::*;

pub trait OnPaint {
    pub fn on_paint(&self, surface: &Surface);
}

pub trait OnKeyPressed {
    pub fn on_key_pressed(&mut self, key: Key, character: char);
}