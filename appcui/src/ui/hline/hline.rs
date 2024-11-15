use super::Flags;
use AppCUIProcMacro::CustomControl;

#[CustomControl(overwrite = OnPaint, internal = true)]
pub struct HLine {
    flags: Flags,
    title: String,
}

impl HLine {
    pub fn new(title: &str, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            title: String::from(title),
            flags,
            base: ControlBase::new(layout, false),
        };
        obj.set_size_bounds(1, 1, u16::MAX, 1);
        obj
    }

    pub fn title(&self) -> &str {
        &self.title
    }

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
                .singleline_width(w as u16 - 4)
                .build();
            surface.write_text(&self.title, &format);
        }
    }
}
