use super::Flags;
use appcui_proc_macro::CustomControl;

#[CustomControl(overwrite = OnPaint, internal = true)]
pub struct HLine {
    flags: Flags,
    title: String,
}

impl HLine {
    /// Creates a new horizontal line with the specified title, layout and flags.
    /// The flags can be a combination of the following values:
    /// * `Flags::DoubleLine` - if set, the line will be drawn with double lines
    /// * `Flags::HasTitle` - if set, the line will have a title (otherwise the title field will be ignored)
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let mut hline = HLine::new("My Title", Layout::new("x:1,y:1,w:30"), Flags::DoubleLine | Flags::HasTitle);
    /// ```
    pub fn new(title: &str, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            title: String::from(title),
            flags,
            base: ControlBase::new(layout, false),
        };
        obj.set_size_bounds(1, 1, u16::MAX, 1);
        obj
    }

    /// Returns the title of the line (if any)
    #[inline(always)]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Sets the title of the line.
    pub fn set_title(&mut self, new_title: &str) {
        self.title.clear();
        self.title.push_str(new_title);
    }
}

impl OnPaint for HLine {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = if self.is_enabled() { theme.lines.normal } else { theme.lines.inactive };
        let w = self.size().width;
        surface.draw_horizontal_line_with_size(
            0,
            0,
            w,
            if self.flags.contains(Flags::DoubleLine) {
                LineType::Double
            } else {
                LineType::Single
            },
            attr,
        );
        if self.flags.contains(Flags::HasTitle) && w >= 5 {
            let format = TextFormatBuilder::new()
                .position(w as i32 / 2, 0)
                .attribute(if self.is_enabled() { theme.text.normal } else { theme.text.inactive })
                .align(TextAlignament::Center)
                .wrap_type(WrapType::SingleLineWrap(w as u16 - 4))
                .build();
            surface.write_text(&self.title, &format);
        }
    }
}
