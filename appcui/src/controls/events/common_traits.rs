use crate::appcui::graphics::*;
use crate::appcui::input::*;

pub trait OnPaint {
    pub fn on_paint(self, surface: &Surface);
}