use crate::prelude::*;
use crate::ui::threestatebox::{events::EventData, initialization_flags::Type, State};

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent,internal=true)]
pub struct ThreeStateBox {
    caption: Caption,
    state: State,
    check_symbol: Symbol,
    uncheck_symbol: Symbol,
    unknown_symbol: Symbol,
    symbol_width: u8,
}

impl ThreeStateBox {
    /// Creates a new threestatebox with the specified caption, layout and state.
    /// The caption can contain a hotkey (e.g. "This is a &test").
    /// The hotkey will be underlined and pressing ALT+T will trigger the default action.
    /// The default action is to change the state of the threestatebox.
    /// The state can be one of the following: Checked, Unchecked, Unknown.
    pub fn new(caption: &str, layout: Layout, state: State) -> Self {
        Self::with_type(caption, layout, state, Type::Standard)
    }

    /// Creates a new threestatebox with the specified caption, layout and state.
    /// The caption can contain a hotkey (e.g. "This is a &test").
    /// The hotkey will be underlined and pressing ALT+T will trigger the default action.
    /// The default action is to change the state of the threestatebox.
    /// The state can be one of the following: Checked, Unchecked, Unknown.
    /// The threestatebox_type defines the symbols used to represent the state of the threestatebox.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::ui::*;
    ///
    /// let threestatebox = ThreeStateBox::with_type("This is a &test",
    ///                                              Layout::new("d:c,w:10,h:1"),
    ///                                              threestatebox::State::Unchecked,
    ///                                              threestatebox::Type::Ascii);
    /// ```
    pub fn with_type(caption: &str, layout: Layout, state: State, threestatebox_type: Type) -> Self {
        let cs = Symbol::new(threestatebox_type.check_symbol());
        let us = Symbol::new(threestatebox_type.uncheck_symbol());
        let ks = Symbol::new(threestatebox_type.unknown_symbol());
        if cs.width() != us.width() || cs.width() != ks.width() {
            panic!("ThreeStateBox: check, uncheck and unknown symbols must have the same width (1, 2 or 3 characters)");
        }
        if cs.width() == 0 {
            panic!("ThreeStateBox: check, uncheck and unknown symbols must have at least one character");
        }
        let symbol_width = cs.width() + 1;
        let mut cb = ThreeStateBox {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            caption: Caption::new(caption, ExtractHotKeyMethod::AltPlusKey),
            state,
            check_symbol: cs,
            uncheck_symbol: us,
            unknown_symbol: ks,
            symbol_width,
        };
        cb.set_size_bounds(5, 1, u16::MAX, u16::MAX);
        let hotkey = cb.caption.hotkey();
        cb.set_hotkey(hotkey);
        cb
    }

    /// Returns the current state of the threestatebox.
    #[inline(always)]
    pub fn state(&self) -> State {
        self.state
    }

    /// Sets the state of the threestatebox.
    #[inline]
    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    /// Sets the caption of the threestatebox.
    #[inline]
    pub fn set_caption(&mut self, caption: &str) {
        self.caption.set_text(caption, ExtractHotKeyMethod::AltPlusKey);
        let hotkey = self.caption.hotkey();
        self.set_hotkey(hotkey);
    }

    /// Returns the caption of the threestatebox.
    #[inline]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }
}

impl OnPaint for ThreeStateBox {
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
                .align(TextAlignament::Left)
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

        let attr_symbol = if enabled {
            match self.state {
                State::Checked => theme.symbol.checked,
                State::Unchecked => theme.symbol.unchecked,
                State::Unknown => theme.symbol.unknown,
            }
        } else {
            theme.symbol.inactive
        };

        let attr_margin = if self.symbol_width == 4 { attr_text } else { attr_symbol };

        match self.state {
            State::Checked => self.check_symbol.paint(surface, 0, 0, attr_margin, attr_symbol, attr_margin),
            State::Unchecked => self.uncheck_symbol.paint(surface, 0, 0, attr_margin, attr_symbol, attr_margin),
            State::Unknown => self.unknown_symbol.paint(surface, 0, 0, attr_margin, attr_symbol, attr_margin),
        }

        if self.has_focus() {
            surface.set_cursor(if self.symbol_width == 4 { 1 } else { 0 }, 0);
        }
    }
}

impl OnDefaultAction for ThreeStateBox {
    fn on_default_action(&mut self) {
        self.state = match self.state {
            State::Checked => State::Unchecked,
            State::Unchecked => State::Unknown,
            State::Unknown => State::Checked,
        };
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::ThreeStateBox(EventData { state: self.state }),
        });
    }
}

impl OnKeyPressed for ThreeStateBox {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if (key.modifier == KeyModifier::None) && ((key.code == KeyCode::Space) || (key.code == KeyCode::Enter)) {
            self.on_default_action();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

impl OnMouseEvent for ThreeStateBox {
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
