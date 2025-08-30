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

fn write_text(text: &str, f: &mut dyn std::fmt::Write, size: Size) -> std::fmt::Result {
    let w = size.width.max(1);
    let h = size.height.max(1);
    let mut x = 0;
    let mut y = 0;
    for c in text.chars() {
        f.write_char(c)?;
        if (c == '\n') || (c == '\r') {
            x = 0;
            y += 1;
            if y >= h {
                break;
            }
            continue;
        }
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
fn compute_text_prefered_size(text: &str) -> Size {
    let mut w = 0u32;
    let mut h = 0u32;
    let mut current_width = 0;
    for c in text.chars() {
        if (c == '\n') || (c == '\r') {
            h += 1;
            if current_width > w {
                w = current_width;
            }
            current_width = 0;
            continue;
        }
        current_width += 1;
    }
    if current_width > w {
        w = current_width;
    }
    if current_width > 0 {
        h += 1;
    }
    Size::new(w, h)
}

impl GraphNode for &str {
    fn prefered_size(&self) -> Size {
        compute_text_prefered_size(self)
    }
    fn write_label(&self, f: &mut dyn std::fmt::Write, size: Size) -> std::fmt::Result {
        write_text(self, f, size)
    }
    fn write_description(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        f.write_str(self)
    }
}
impl GraphNode for String {
    fn prefered_size(&self) -> Size {
        compute_text_prefered_size(self.as_str())
    }
    fn write_label(&self, f: &mut dyn std::fmt::Write, size: Size) -> std::fmt::Result {
        write_text(self.as_str(), f, size)
    }
    fn write_description(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}