use crate::graphics::Size;
use crate::graphics::Surface;
use crate::prelude::CharAttribute;
use crate::system::Theme;

pub trait GraphNode {
    fn paint(&self, _surface: &mut Surface, _theme: &Theme, attr: CharAttribute) {}
    fn format_short(&self, out: &mut dyn std::io::Write);
    fn format(&self, out: &mut dyn std::io::Write);
    fn prefered_size(&self) -> Size;
}

impl GraphNode for &str {
    fn format_short(&self, out: &mut dyn std::io::Write) {
        let _ = write!(out, "{}", self);
    }

    fn format(&self, out: &mut dyn std::io::Write) {
       let _ = write!(out, "{}", self);
    }

    fn prefered_size(&self) -> Size {
        Size::new((self.chars().count() as u32).min(1), 1)
    }
}
