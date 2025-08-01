use super::Type;
use crate::prelude::*;
use crate::ui::checkbox::events::EventData;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent,internal=true)]
pub struct CheckBox {
    caption: Caption,
    checked: bool,
    check_symbol: Symbol,
    uncheck_symbol: Symbol,
    symbol_width: u8,
}

impl CheckBox {
    /// Creates a new checkbox with the specified caption, layout and initial checked state.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut checkbox = CheckBox::new("Check me", layout!("x:1,y:1,w:20,h:1"), false);
    /// ```
    pub fn new(caption: &str, layout: Layout, checked: bool) -> Self {
        Self::with_type(caption, layout, checked, Type::Standard)
    }

    pub fn with_type(caption: &str, layout: Layout, checked: bool, checkbox_type: Type) -> Self {
        let cs = Symbol::new(checkbox_type.check_symbol());
        let us = Symbol::new(checkbox_type.uncheck_symbol());
        if cs.width() != us.width() {
            panic!("CheckBox: check and uncheck symbols must have the same width (1, 2 or 3 characters)");
        }
        if cs.width() == 0 {
            panic!("CheckBox: check and uncheck symbols must have at least one character");
        }
        let symbol_width = cs.width() + 1;
        let mut cb = CheckBox {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            caption: Caption::new(caption, ExtractHotKeyMethod::AltPlusKey),
            checked,
            check_symbol: cs,
            uncheck_symbol: us,
            symbol_width,
        };
        cb.set_size_bounds(5, 1, u16::MAX, u16::MAX);
        let hotkey = cb.caption.hotkey();
        cb.set_hotkey(hotkey);
        cb
    }

    /// Returns **true** if the checkbox is checked, **false** otherwise.
    #[inline(always)]
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    /// Sets the checkbox state to checked or unchecked.
    #[inline(always)]
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    /// Sets the checkbox caption. The caption can contain a hotkey, which is indicated by an ampersand (&) before the character.
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
        let attr_text = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        let enabled = self.is_enabled();
        let col_hot_key = if enabled { theme.text.hot_key } else { theme.text.inactive };
        let sz = self.size();

        if sz.width > self.symbol_width as u32 {
            let mut format = TextFormatBuilder::new()
                .position(self.symbol_width as i32, 0)
                .attribute(attr_text)
                .align(TextAlignment::Left)
                .chars_count(self.caption.chars_count() as u16)
                .build();
            if sz.height > 1 {
                format.set_wrap_type(WrapType::WordWrap(sz.width as u16 - self.symbol_width as u16));
            }
            if self.caption.has_hotkey() {
                format.set_hotkey(col_hot_key, self.caption.hotkey_pos().unwrap() as u32);
            }
            surface.write_text(self.caption.text(), &format);
        }
        if self.checked {
            let attr_symbol = if enabled { theme.symbol.checked } else { theme.symbol.inactive };
            let attr_margin = if self.symbol_width == 4 { attr_text } else { attr_symbol };
            self.check_symbol.paint(surface, 0, 0, attr_margin, attr_symbol, attr_margin);
        } else {
            let attr_symbol = if enabled { theme.symbol.unchecked } else { theme.symbol.inactive };
            let attr_margin = if self.symbol_width == 4 { attr_text } else { attr_symbol };
            self.uncheck_symbol.paint(surface, 0, 0, attr_margin, attr_symbol, attr_margin);
        }
        if self.has_focus() {
            surface.set_cursor(if self.symbol_width == 4 { 1 } else { 0 }, 0);
        }
    }
}
impl OnDefaultAction for CheckBox {
    fn on_default_action(&mut self) {
        self.checked = !self.checked;
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::CheckBox(EventData { checked: self.checked }),
        });
    }
}
impl OnKeyPressed for CheckBox {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if (key.modifier == KeyModifier::None) && ((key.code == KeyCode::Space) || (key.code == KeyCode::Enter)) {
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
