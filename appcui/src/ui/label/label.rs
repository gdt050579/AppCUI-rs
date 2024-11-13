use crate::prelude::*;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct Label {
    caption: Caption,
}
impl Label {
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
    pub fn caption(&self)->&str {
        self.caption.text()
    }
}
impl OnPaint for Label {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.size();
        let mut format = TextFormat::new(
            0,
            0,
            CharAttribute::default(),
            TextAlignament::Left,
            sz.height > 1,
        );
        format.chars_count = Some(self.caption.chars_count() as u16);
        format.char_attr = if self.is_enabled() {
            theme.text.normal
        } else {
            theme.text.inactive
        };
        format.hotkey_pos = self.caption.hotkey_pos();
        if self.caption.has_hotkey() {
            format.hotkey_attr = Some(if self.is_enabled() {
                theme.text.hot_key
            } else {
                theme.text.inactive
            });
        }
        if format.multi_line {
            format.text_wrap = TextWrap::Word;
            format.width = Some(sz.width as u16);
        }
        format.chars_count = Some(self.caption.chars_count() as u16);
        surface.write_text_old(self.caption.text(), &format);
    }
}
