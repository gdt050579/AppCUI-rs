use super::Flags;
use appcui_proc_macro::CustomControl;

#[CustomControl(overwrite = OnPaint, internal = true)]
pub struct VLine {
    flags: Flags,
}

impl VLine {
    /// Creates a new vertical line control.
    /// The `layout` parameter specifies the position and size of the control.
    /// The `flags` parameter can be used to specify additional options, such as whether the line should be double-width.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut vline = VLine::new(layout!("x:1,y:1,w:1,h:10"),
    ///                            vline::Flags::DoubleLine | vline::Flags::MergeBorders);
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::new(layout, false),
            flags,
        };
        obj.set_size_bounds(1, 1, 1, u16::MAX);
        obj
    }
}

impl OnPaint for VLine {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = if self.is_enabled() { theme.lines.normal } else { theme.lines.inactive };
        let h = surface.size().height;

        surface.draw_vertical_line_with_size(
            0,
            0,
            h,
            if self.flags.contains(Flags::DoubleLine) {
                LineType::Double
            } else {
                LineType::Single
            },
            attr,
        );
        if self.flags.contains(Flags::MergeBorders) {
            surface.write_box_junction(0, -1);
            surface.write_box_junction(0, self.size().height as i32);
        }
    }
}
