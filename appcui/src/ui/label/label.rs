use crate::prelude::*;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct Label {
    caption: Caption,
}
impl Label {
    /// Creates a new label with the specified caption and layout.
    /// The caption can contain a hotkey (e.g. "File &Open") and the layout is used to position the label on the screen.
    /// The label is a read-only control that can be used to display text on the screen.
    /// The label is not a focusable control and it does not accept any input.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut label = Label::new("My Label", Layout::new("x:1,y:1,w:30"));
    /// ```
    pub fn new(caption: &str, layout: Layout) -> Self {
        Label {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled),
            caption: Caption::new(caption, ExtractHotKeyMethod::AltPlusKey),
        }
    }
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::AltPlusKey);
    }
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }
}
impl OnPaint for Label {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.size();
        let mut format = TextFormatBuilder::new()
            .position(0, 0)
            .attribute(if self.is_enabled() { theme.text.normal } else { theme.text.inactive })
            .align(TextAlignament::Left)
            .chars_count(self.caption.chars_count() as u16)
            .build();

        if self.caption.has_hotkey() {
            format.set_hotkey(
                if self.is_enabled() { theme.text.hot_key } else { theme.text.inactive },
                self.caption.hotkey_pos().unwrap() as u32,
            );
        }
        if sz.height > 1 {
            format.set_wrap_type(WrapType::WordWrap(sz.width as u16));
        }

        surface.write_text(self.caption.text(), &format);
    }
}
