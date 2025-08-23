use crate::graphics::Size;
use crate::graphics::Surface;
use crate::prelude::CharAttribute;
use crate::system::Theme;

pub trait GraphNode {
    fn paint(&self, _surface: &mut Surface, _theme: &Theme, attr: CharAttribute) {}
    fn write(&self, out: &mut String, size: Size);
    fn prefered_size(&self) -> Size;
}

impl GraphNode for &str {
    fn write(&self, out: &mut String, size: Size) {
       out.push_str(self)
    }

    fn prefered_size(&self) -> Size {
        Size::new(self.chars().count() as u32, 1)
    }
}
