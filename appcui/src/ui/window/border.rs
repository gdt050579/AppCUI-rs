use crate::graphics::*;
use crate::system::*;

#[derive(Default)]
pub(super) struct Border {
    size: Size
}
impl Border {
    pub(super) fn new() -> Self {
        Self {
            size: Size::new(0, 0)
        }
    }
    pub(super) fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme) {
    }
}
