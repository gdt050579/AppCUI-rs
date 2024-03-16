use crate::prelude::*;
use crate::ui::radiobox::events::EventData;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnSiblingSelected,internal=true)]
pub struct RadioBox {
    caption: Caption,
    selected: bool,
}

impl RadioBox {
    pub fn new(caption: &str, layout: Layout, selected: bool) -> Self {
        let mut cb = RadioBox {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            caption: Caption::new(caption, ExtractHotKeyMethod::AltPlusKey),
            selected,
        };
        cb.set_size_bounds(5, 1, u16::MAX, u16::MAX);
        let hotkey = cb.caption.get_hotkey();
        cb.set_hotkey(hotkey);
        cb
    }
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    #[inline(always)]
    pub fn set_selected(&mut self) {
        if self.handle.is_none() {
            return; 
        }
        if let Some(parent) = RuntimeManager::get().get_controls_mut().get(self.parent) {
            parent.get_base().notify_children_of_selection(self.handle);
        }
    }
    pub fn set_caption(&mut self, caption: &str) {
        self.caption.set_text(caption, ExtractHotKeyMethod::AltPlusKey);
        let hotkey = self.caption.get_hotkey();
        self.set_hotkey(hotkey);
    }
    /// Returns the RadioBox caption.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.get_text()
    }
}
impl OnPaint for RadioBox {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        let col_hot_key = if self.is_enabled() { theme.text.hot_key } else { theme.text.inactive };

        surface.write_string(0, 0, "( ) ", col_text, false);
        let sz = self.size();

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
        if self.selected {
            let col = if self.is_enabled() {
                theme.symbol.checked
            } else {
                theme.symbol.inactive
            };
            surface.write_char(1, 0, Character::with_attributes(SpecialChar::CircleFilled, col));
        }
        if self.has_focus() {
            surface.set_cursor(1, 0);
        }
    }
}
impl OnDefaultAction for RadioBox {
    fn on_default_action(&mut self) {
        self.set_selected();
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::RadioBoxEvent(EventData {}),
        });
    }
}
impl OnKeyPressed for RadioBox {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if (key.modifier == KeyModifier::None) && ((key.code == KeyCode::Space) || (key.code == KeyCode::Enter)) {
            self.on_default_action();
            return EventProcessStatus::Processed;
        }
        return EventProcessStatus::Ignored;
    }
}
impl OnMouseEvent for RadioBox {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if self.caption.get_chars_count() > (self.size().width - 4) as usize {
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
impl OnSiblingSelected for RadioBox {
    #[allow(private_interfaces)]
    fn on_sibling_selected(&mut self, handle: Handle<UIElement>) {
        self.selected = self.handle == handle;
    }
}