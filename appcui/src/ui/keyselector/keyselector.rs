use crate::prelude::*;
use crate::ui::keyselector::{events::EventData, Flags};

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct KeySelector {
    flags: Flags,
    key: Key,
}
impl KeySelector {

    /// Creates a new KeySelector object with the specified key, layout and flags.
    /// The flags can be a combination of the following values:
    /// * `keyselector::Flags::AcceptEnter` - if set, the Enter key will be accepted
    /// * `keyselector::Flags::AcceptEscape` - if set, the Escape key will be accepted
    /// * `keyselector::Flags::AcceptTab` - if set, the Tab key will be accepted
    /// * `keyselector::Flags::ReadOnly` - if set, the KeySelector will be read-only and will not accept any key
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// 
    /// let mut keyselector = KeySelector::new(Key::from(KeyCode::F1), 
    ///                                        Layout::new("x:1,y:1,w:30"), 
    ///                                        keyselector::Flags::AcceptEnter | 
    ///                                        keyselector::Flags::AcceptEscape);
    /// ```
    pub fn new(key: Key, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            key,
        };
        obj.set_size_bounds(5, 1, u16::MAX, 1);
        obj
    }

    /// Returns the current key selected by the KeySelector.
    #[inline(always)]
    pub fn key(&self) -> Key {
        self.key
    }

    /// Manually sets the key selected by the KeySelector.
    #[inline(always)]
    pub fn set_key(&mut self, key: Key) {
        self.key = key;
    }
}
impl OnPaint for KeySelector {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        let right = self.size().width as i32;
        surface.fill_horizontal_line(0, 0, right, Character::with_attributes(' ', attr));
        surface.reduce_clip_by(1, 0, 1, 0);
        let m = self.key.modifier.name();
        let k = self.key.code.name();
        if !m.is_empty() {
            surface.write_string(1, 0, m, attr, false);
            surface.write_string(1 + m.len() as i32, 0, k, attr, false);
        } else if self.key == Key::None {
            if self.has_focus() {
                surface.write_string(1, 0, "None", attr, false);
            } else {
                surface.write_string(1, 0, "None", theme.editor.inactive, false);
            }
        } else {
            surface.write_string(1, 0, k, attr, false);
        }
        if self.has_focus() {
            surface.set_cursor(1, 0);
        }
    }
}
impl OnKeyPressed for KeySelector {
    fn on_key_pressed(&mut self, key: Key, _: char) -> EventProcessStatus {
        match key.code {
            KeyCode::Enter => {
                if (!self.flags.contains(Flags::AcceptEnter)) || (self.flags.contains(Flags::ReadOnly)) {
                    return EventProcessStatus::Ignored;
                }
            }
            KeyCode::Escape => {
                if (!self.flags.contains(Flags::AcceptEscape)) || (self.flags.contains(Flags::ReadOnly)) {
                    return EventProcessStatus::Ignored;
                }
            }
            KeyCode::Tab => {
                if (!self.flags.contains(Flags::AcceptTab)) || (self.flags.contains(Flags::ReadOnly)) {
                    return EventProcessStatus::Ignored;
                }
            }
            _ => {}
        }
        if !self.flags.contains(Flags::ReadOnly) && self.key != key {
            let old = self.key;
            self.key = key;
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::KeySelector(EventData { new_key: key, old_key: old }),
            });
        }
        EventProcessStatus::Processed
    }
}
impl OnMouseEvent for KeySelector {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => EventProcessStatus::Processed,
            _ => EventProcessStatus::Ignored,
        }
    }
}
