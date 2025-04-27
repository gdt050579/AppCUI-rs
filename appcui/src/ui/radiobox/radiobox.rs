use crate::prelude::*;
use crate::ui::radiobox::events::EventData;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnSiblingSelected,internal=true)]
pub struct RadioBox {
    caption: Caption,
    selected: bool,
}

impl RadioBox {
    /// Creates a new RadioBox with the specified caption, layout and initial selected state.
    /// When a radio box is selected, it will notify its parent control to update the selection state of its siblings. 
    /// Usually multiple radioboxes are being used as part of the same parent control (e.g., a panel)
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let mut panel = Panel::new("Radioboxes group",
    ///                            Layout::new("x:1,y:1,w:20,h:10"),
    ///                            panel::Type::Border);
    /// panel.add(RadioBox::new("Select me &1", Layout::new("x:1,y:1,w:20,h:1"), false));
    /// panel.add(RadioBox::new("Select me &2", Layout::new("x:1,y:1,w:20,h:1"), false));
    /// panel.add(RadioBox::new("Select me &3", Layout::new("x:1,y:1,w:20,h:1"), true));
    /// ```
    /// All o the obove radio boxes will be part of the same parent control (the panel).
    /// When one of them is selected, the others will be automatically deselected.
    pub fn new(caption: &str, layout: Layout, selected: bool) -> Self {
        let mut cb = RadioBox {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            caption: Caption::new(caption, ExtractHotKeyMethod::AltPlusKey),
            selected,
        };
        cb.set_size_bounds(5, 1, u16::MAX, u16::MAX);
        let hotkey = cb.caption.hotkey();
        cb.set_hotkey(hotkey);
        cb
    }

    /// Returns **true** if the radio box is selected, **false** otherwise.
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Sets the radio box state to selected.
    /// When a radio box is selected, it will notify its parent control to update the selection state of its siblings to **unselected**.
    #[inline(always)]
    pub fn set_selected(&mut self) {
        if self.handle.is_none() {
            return; 
        }
        if let Some(parent) = RuntimeManager::get().get_controls_mut().get(self.parent) {
            parent.base().notify_children_of_selection(self.handle);
        }
    }

    /// Sets the radio box caption. The caption can contain a hotkey, which is indicated by an ampersand (&) before the character.
    pub fn set_caption(&mut self, caption: &str) {
        self.caption.set_text(caption, ExtractHotKeyMethod::AltPlusKey);
        let hotkey = self.caption.hotkey();
        self.set_hotkey(hotkey);
    }
    /// Returns the RadioBox caption.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
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
            let mut format = TextFormatBuilder::new()
                .position(4, 0)
                .attribute(col_text)
                .align(TextAlignament::Left)
                .chars_count(self.caption.chars_count() as u16)
                .build();
            if sz.height > 1 {
                format.set_wrap_type(WrapType::WordWrap(sz.width as u16 - 4));
            }
            if self.caption.has_hotkey() {
                format.set_hotkey(col_hot_key, self.caption.hotkey_pos().unwrap() as u32);
            }
            surface.write_text(self.caption.text(), &format);
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
            data: ControlEventData::RadioBox(EventData {}),
        });
    }
}
impl OnKeyPressed for RadioBox {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if (key.modifier == KeyModifier::None) && ((key.code == KeyCode::Space) || (key.code == KeyCode::Enter)) {
            self.on_default_action();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for RadioBox {
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
impl OnSiblingSelected for RadioBox {
    #[allow(private_interfaces)]
    fn on_sibling_selected(&mut self, handle: Handle<()>) {
        self.selected = self.handle == handle;
    }
}