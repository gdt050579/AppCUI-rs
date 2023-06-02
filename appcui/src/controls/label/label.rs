use super::super::events::*;
use super::super::menu::*;
use super::super::ControlBase;
use super::super::Layout;
use super::super::StatusFlags;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::utils::*;

use AppCUIProcMacro::AppCUIControl;

#[AppCUIControl(overwrite=OnPaint)]
pub struct Label {
    caption: Caption,
}
impl Label {
    pub fn new(caption: &str, layout: Layout) -> Self {
        Label {
            base: ControlBase::new(layout, StatusFlags::Visible | StatusFlags::Enabled),
            caption: Caption::new(caption, true),
        }
    }
    pub fn set_text(&mut self, text: &str) {
        self.caption.set_text(text, false);
    }
}
impl OnPaint for Label {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.get_size();
        let mut format = TextFormat::new(
            0,
            0,
            CharAttribute::default(),
            TextAlignament::Left,
            sz.height > 1,
        );
        format.chars_count = Some(self.caption.get_chars_count() as u16);
        format.char_attr = if self.is_enabled() {
            theme.text.normal
        } else {
            theme.text.inactive
        };
        format.hotkey_pos = self.caption.get_hotkey_pos();
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
        format.chars_count = Some(self.caption.get_chars_count() as u16);
        surface.write_text(self.caption.get_text(), &format);
    }
}
