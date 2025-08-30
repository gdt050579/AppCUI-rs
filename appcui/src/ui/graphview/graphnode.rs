use crate::graphics::Size;
use crate::graphics::Surface;
use crate::prelude::CharAttribute;
use crate::system::Theme;

pub trait GraphNode {
    fn paint(&self, _surface: &mut Surface, _theme: &Theme, _attr: CharAttribute) {}
    
    /// Write the label of the node into the provided formatter
    fn write_label(&self, f: &mut dyn std::fmt::Write, size: Size) -> std::fmt::Result;

    /// Optional tooltip/hover text (a description of the node)
    fn write_description(&self, _f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        Ok(())
    }
    fn prefered_size(&self) -> Size;
}

impl GraphNode for &str {

    fn prefered_size(&self) -> Size {
        Size::new(self.chars().count() as u32, 1)
    }
    
    fn write_label(&self, f: &mut dyn std::fmt::Write, size: Size) -> std::fmt::Result {
        let w = size.width.min(1);
        let h = size.height.min(1);
        let mut x = 0;
        let mut y = 0;
        for c in self.chars() {
            f.write_char(c)?;
            x += 1;
            if x >= w {
                x = 0;
                y += 1;
                if y >= h {
                    break;
                }
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}
