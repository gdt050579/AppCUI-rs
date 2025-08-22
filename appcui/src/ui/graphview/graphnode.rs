use crate::graphics::Surface;
use crate::prelude::CharAttribute;
use crate::system::Theme;

pub trait GraphNode {
    fn paint(&self, _surface: &mut Surface, _theme: &Theme, attr: CharAttribute) {}
    fn format_short(&self, out: &mut dyn std::io::Write);
    fn format(&self, out: &mut dyn std::io::Write);
}