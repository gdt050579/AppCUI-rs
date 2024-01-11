use crate::prelude::*;
use crate::ui::threestatebox::events::EventData;

#[derive(Copy, Clone)]
pub enum ThreeStateBoxSelection {
    Checked,
    Unchecked,
    Unknown,
}

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent,internal=true)]
pub struct ThreeStateBox {
    caption: Caption,
    state: ThreeStateBoxSelection,
}

impl ThreeStateBox {
    pub fn new(caption: &str, layout: Layout, state: ThreeStateBoxSelection) -> Self {
        let mut cb = ThreeStateBox {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            caption: Caption::new(caption, true),
            state,
        };
        cb.set_size_bounds(5, 1, u16::MAX, u16::MAX);
        let hotkey = cb.caption.get_hotkey();
        cb.set_hotkey(hotkey);
        cb
    }
    #[inline]
    pub fn get_state(&self) -> ThreeStateBoxSelection {
        self.state
    }
    #[inline]
    pub fn set_state(&mut self, new_state: ThreeStateBoxSelection) {
        self.state = new_state;
    }
}
impl OnPaint for ThreeStateBox {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        let col_hot_key = if self.is_enabled() { theme.text.hot_key } else { theme.text.inactive };

        surface.write_string(0, 0, "[ ] ", col_text, false);
        let sz = self.get_size();

        if sz.width > 4 {
            let mut format = TextFormat::new(4, 0, col_text, TextAlignament::Left, sz.height > 1);
            if format.multi_line {
                format.text_wrap = TextWrap::Word;
                format.width = Some(sz.width as u16 - 4);
            }
            if self.caption.has_hotkey() {
                format.hotkey_pos = self.caption.get_hotkey_pos();
                format.hotkey_attr = Some(col_hot_key);
            }
            format.chars_count = Some(self.caption.get_chars_count() as u16);
            surface.write_text(&self.caption.get_text(), &format);
        }

        match self.state {
            ThreeStateBoxSelection::Checked => {
                let col = if self.is_enabled() {
                    theme.symbol.checked
                } else {
                    theme.symbol.inactive
                };
                surface.write_char(1, 0, Character::with_attributes(SpecialChar::CheckMark, col));
            }
            ThreeStateBoxSelection::Unchecked => {}
            ThreeStateBoxSelection::Unknown => {
                let col = if self.is_enabled() {
                    theme.symbol.unknown
                } else {
                    theme.symbol.inactive
                };
                surface.write_char(1, 0, Character::with_attributes('?', col));
            }
        }

        if self.has_focus() {
            surface.set_cursor(1, 0);
        }
    }
}
impl OnDefaultAction for ThreeStateBox {
    fn on_default_action(&mut self) {
        self.state = match self.state {
            ThreeStateBoxSelection::Checked => ThreeStateBoxSelection::Unchecked,
            ThreeStateBoxSelection::Unchecked => ThreeStateBoxSelection::Unknown,
            ThreeStateBoxSelection::Unknown => ThreeStateBoxSelection::Checked,
        };
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::ThreeStateBoxEvent(EventData { state: self.state }),
        });
    }
}
impl OnKeyPressed for ThreeStateBox {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if (key.modifier == KeyModifier::None) && ((key.code == KeyCode::Space) || (key.code == KeyCode::Enter)) {
            self.on_default_action();
            return EventProcessStatus::Processed;
        }
        return EventProcessStatus::Ignored;
    }
}
impl OnMouseEvent for ThreeStateBox {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if self.caption.get_chars_count() > (self.get_size().width - 4) as usize {
                    self.show_tooltip(self.caption.get_text());
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
