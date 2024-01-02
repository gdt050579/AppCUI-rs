use crate::graphics::*;
use crate::system::Theme;

pub struct ScrollBar {
    x: i32,
    y: i32,
    dimension: u16,
    vertical: bool,
    max_value: u64,
    value: u64,
}
impl ScrollBar {
    pub fn new(x: i32, y: i32, dimension: u16, vertical: bool, value: u64, max_value: u64) -> Self {
        Self {
            x,
            y,
            vertical,
            value: value.min(max_value),
            max_value,
            dimension: dimension.min(3),
        }
    }
    pub fn paint(&self, surface: &mut Surface, theme: &Theme) {
        
    }
}
