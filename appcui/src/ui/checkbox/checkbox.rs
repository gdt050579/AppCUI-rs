use crate::ui::checkbox::events::EventData;
use crate::prelude::*;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent,internal=true)]
pub struct CheckBox {
    caption: Caption,
    checked: bool,
}

impl CheckBox {
    pub fn new(caption: &str, layout: Layout, checked: bool) -> Self {
        let mut cb = CheckBox {
            base: ControlBase::with_status_flags(
                layout,
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
            caption: Caption::new(caption, ExtractHotKeyMethod::AltPlusKey),
            checked,
        };
        cb.set_size_bounds(5, 1, u16::MAX, u16::MAX);
        let hotkey = cb.caption.hotkey();
        cb.set_hotkey(hotkey);
        cb
    }
    #[inline(always)]
    pub fn is_checked(&self) -> bool {
        self.checked
    }
    #[inline(always)]
    pub fn set_checked(&mut self, checked: bool)  {
        self.checked = checked;
    }
    pub fn set_caption(&mut self, caption: &str) {
        self.caption.set_text(caption, ExtractHotKeyMethod::AltPlusKey);
        let hotkey = self.caption.hotkey();
        self.set_hotkey(hotkey);
    }
    /// Returns the checkbox caption.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }
}
impl OnPaint for CheckBox {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        let col_hot_key = if self.is_enabled() {
            theme.text.hot_key
        } else {
            theme.text.inactive
        };

        surface.write_string(0, 0, "[ ] ", col_text, false);
        let sz = self.size();

        if sz.width > 4 {
            let mut format = TextFormat::new(4, 0, col_text, TextAlignament::Left, sz.height > 1);
            if format.multi_line {
                format.text_wrap = TextWrap::Word;
                format.width = Some(sz.width as u16 - 4);
            }
            if self.caption.has_hotkey() {
                format.hotkey_pos = self.caption.hotkey_pos();
                format.hotkey_attr = Some(col_hot_key);
            }
            format.chars_count = Some(self.caption.chars_count() as u16);
            surface.write_text(self.caption.text(), &format);
        }
        if self.checked {
            let col = if self.is_enabled() {
                theme.symbol.checked
            } else {
                theme.symbol.inactive
            };
            surface.write_char(
                1,
                0,
                Character::with_attributes(SpecialChar::CheckMark, col),
            );
        }
        if self.has_focus() {
            surface.set_cursor(1, 0);
        }
    }
}
impl OnDefaultAction for CheckBox {
    fn on_default_action(&mut self) {
        self.checked = !self.checked;
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::CheckBoxEvent(EventData {
                checked: self.checked,
            }),
        });
    }
}
impl OnKeyPressed for CheckBox {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if (key.modifier == KeyModifier::None)
            && ((key.code == KeyCode::Space) || (key.code == KeyCode::Enter))
        {
            self.on_default_action();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for CheckBox {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if self.caption.chars_count() > (self.size().width - 4) as usize {
                    self.show_tooltip(self.caption.text());
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Leave => EventProcessStatus::Processed,
            MouseEvent::Released(data) => {
                if self.is_coord_in_control(data.x, data.y) {
                    self.on_default_action();
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
