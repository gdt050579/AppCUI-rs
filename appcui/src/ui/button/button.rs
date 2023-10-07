use crate::prelude::*;
use crate::ui::button::{events::EventData, Flags};

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct Button {
    flags: Flags,
    caption: Caption,
    pressed: bool,
}
impl Button {
    /// Creates a new button with the specified caption, layout and flags
    /// # Examples
    /// ```
    /// use appcui::prelude::*;
    /// let mut button = Button::new("Click me!", Layout::new("x:1,y:1,w:15"), button::Flags::None);
    /// ```
    pub fn new(caption: &str, layout: Layout, flags: Flags) -> Self {
        let mut but = Button {
            base: ControlBase::new(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            caption: Caption::new(caption, true),
            flags,
            pressed: false,
        };

        if flags.contains(super::Flags::Flat) {
            but.set_size_bounds(3, 1, u16::MAX, 1);
        } else {
            but.set_size_bounds(4, 2, u16::MAX, 2);
        }
        let hotkey = but.caption.get_hotkey();
        but.set_hotkey(hotkey);
        but
    }
    /// Sets the caption of a button. Using `&` in the provided text followed by a letter or a number will automatically assign Alt+**<number|letter>** hotkey to that button.
    /// # Examples
    /// ```
    /// use appcui::prelude::*;
    /// let mut button = button!("one,x:1,y:1,w:15"); // the caption is `one`
    /// button.set_caption("&two");   // now the caption is `two` and Alt+T is a hotkey
    /// button.set_caption("three");  // caption is `three`, and no hot-key
    /// ```
    pub fn set_caption(&mut self, caption: &str) {
        self.caption.set_text(caption, true);
        let hotkey = self.caption.get_hotkey();
        self.set_hotkey(hotkey);
    }
    /// Returns the button caption.
    pub fn get_caption(&self) -> &str {
        self.caption.get_text()
    }
}
impl OnDefaultAction for Button {
    fn on_default_action(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::ButtonEvent(EventData {}),
        });
    }
}
impl OnKeyPressed for Button {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.get_compact_code() {
            key!("Space") | key!("Enter") => {
                self.on_default_action();
                return EventProcessStatus::Processed;
            }
            _ => return EventProcessStatus::Ignored,
        }
    }
}

impl OnPaint for Button {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.text.inactive,
            _ if self.has_focus() => theme.button.text.focused,
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };
        let flat = self.flags.contains(Flags::Flat);
        let w = if flat { self.get_size().width } else { self.get_size().width - 1 };
        let mut format = TextFormat::single_line((w / 2) as i32, 0, col_text, TextAlignament::Center);
        format.chars_count = Some(self.caption.get_chars_count() as u16);
        format.width = Some(w as u16);
        if self.caption.has_hotkey() {
            format.hotkey_attr = Some(match () {
                _ if !self.is_enabled() => theme.button.hotkey.inactive,
                _ if self.has_focus() => theme.button.hotkey.focused,
                _ if self.is_mouse_over() => theme.button.hotkey.hovered,
                _ => theme.button.hotkey.normal,
            });
            format.hotkey_pos = self.caption.get_hotkey_pos();
        }
        if flat {
            surface.clear(Character::with_attributes(' ', col_text));
            surface.write_text(self.caption.get_text(), &format);
        } else {
            if self.pressed {
                surface.fill_horizontal_line_with_size(1, 0, w, Character::with_attributes(' ', col_text));
                format.x += 1;
                surface.write_text(self.caption.get_text(), &format);
            } else {
                surface.fill_horizontal_line_with_size(0, 0, w, Character::with_attributes(' ', col_text));
                surface.write_text(self.caption.get_text(), &format);
                surface.fill_horizontal_line_with_size(1, 1, w, Character::with_attributes(SpecialChar::BlockUpperHalf, theme.button.shadow));
                surface.write_char(
                    (w as i32),
                    0,
                    Character::with_attributes(SpecialChar::BlockLowerHalf, theme.button.shadow),
                );
            }
        }
    }
}
impl OnMouseEvent for Button {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if self.caption.get_chars_count() > (self.get_size().width - 2) as usize {
                    self.show_tooltip(self.caption.get_text());
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Leave => EventProcessStatus::Processed,
            MouseEvent::Released(data) => {
                self.pressed = false;
                if self.is_coord_in_control(data.x, data.y) {
                    self.on_default_action();
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(data) => {
                if self.pressed && (!self.is_coord_in_control(data.x, data.y)) {
                    self.pressed = false;
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Pressed(_) => {
                self.pressed = true;
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
