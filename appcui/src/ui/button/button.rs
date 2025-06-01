use crate::prelude::*;
use crate::ui::button::{events::EventData, Type};

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct Button {
    button_type: Type,
    caption: Caption,
    pressed: bool,
}
impl Button {
    /// Creates a new button with the specified caption, layout and flags
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    /// let mut button = Button::new("Click me!", Layout::new("x:1,y:1,w:15"), button::Type::Normal);
    /// ```
    pub fn new(caption: &str, layout: Layout, button_type: Type) -> Self {
        let mut but = Button {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            caption: Caption::new(caption, ExtractHotKeyMethod::AltPlusKey),
            button_type,
            pressed: false,
        };

        if button_type == super::Type::Flat {
            but.set_size_bounds(3, 1, u16::MAX, 1);
        } else {
            but.set_size_bounds(4, 2, u16::MAX, 2);
        }
        let hotkey = but.caption.hotkey();
        but.set_hotkey(hotkey);
        but
    }
    /// Sets the caption of a button. Using `&` in the provided text followed by a letter or a number will automatically assign Alt+**<number|letter>** hotkey to that button.
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    /// let mut button = button!("one,x:1,y:1,w:15"); // the caption is `one`
    /// button.set_caption("&two");   // now the caption is `two` and Alt+T is a hotkey
    /// button.set_caption("three");  // caption is `three`, and no hot-key
    /// ```
    pub fn set_caption(&mut self, caption: &str) {
        self.caption.set_text(caption, ExtractHotKeyMethod::AltPlusKey);
        let hotkey = self.caption.hotkey();
        self.set_hotkey(hotkey);
    }
    /// Returns the button caption.
    pub fn caption(&self) -> &str {
        self.caption.text()
    }
}
impl OnDefaultAction for Button {
    fn on_default_action(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::Button(EventData {}),
        });
    }
}
impl OnKeyPressed for Button {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Space") | key!("Enter") => {
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
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
        let flat = self.button_type == super::Type::Flat;
        let w = if flat { self.size().width } else { self.size().width - 1 };
        let x = (w / 2) as i32;
        let mut format = TextFormatBuilder::new()
            .position(x, 0)
            .attribute(col_text)
            .align(TextAlignament::Center)
            .chars_count(self.caption.chars_count() as u16)
            .wrap_type(WrapType::SingleLineWrap(w as u16))
            .build();

        if self.caption.has_hotkey() {
            format.set_hotkey(
                match () {
                    _ if !self.is_enabled() => theme.button.hotkey.inactive,
                    _ if self.has_focus() => theme.button.hotkey.focused,
                    _ if self.is_mouse_over() => theme.button.hotkey.hovered,
                    _ => theme.button.hotkey.normal,
                },
                self.caption.hotkey_pos().unwrap() as u32,
            );
        }
        if flat {
            surface.clear(Character::with_attributes(' ', col_text));
            surface.write_text(self.caption.text(), &format);
        } else if self.pressed {
            surface.fill_horizontal_line_with_size(1, 0, w, Character::with_attributes(' ', col_text));
            format.x += 1;
            surface.write_text(self.caption.text(), &format);
        } else {
            surface.fill_horizontal_line_with_size(0, 0, w, Character::with_attributes(' ', col_text));
            surface.write_text(self.caption.text(), &format);
            surface.fill_horizontal_line_with_size(1, 1, w, Character::with_attributes(SpecialChar::BlockUpperHalf, theme.button.shadow));
            surface.write_char(w as i32, 0, Character::with_attributes(SpecialChar::BlockLowerHalf, theme.button.shadow));
        }
    }
}
impl OnMouseEvent for Button {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                if self.caption.chars_count() > (self.size().width - 2) as usize {
                    self.show_tooltip(self.caption.text());
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
