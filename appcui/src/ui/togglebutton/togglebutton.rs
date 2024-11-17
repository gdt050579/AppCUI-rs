use crate::prelude::*;
use crate::ui::togglebutton::{events::EventData, Type};
use flat_string::FlatString;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct ToggleButton {
    caption: FlatString<22>,
    tooltip: String,
    state: bool,
    button_type: Type,
}
impl ToggleButton {
    pub fn new(caption: &str, tooltip: &str, layout: Layout, selected: bool, button_type: Type) -> Self {
        let mut but = ToggleButton {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            caption: FlatString::from_str(caption),
            tooltip: tooltip.to_string(),
            state: selected,
            button_type,
        };

        but.set_size_bounds(1, 2, u16::MAX, 2);
        but
    }
    /// Sets the caption of a toggle button.
    pub fn set_caption(&mut self, caption: &str) {
        self.caption.set(caption);
    }
    /// Returns the toggle button caption.
    pub fn caption(&self) -> &str {
        self.caption.as_str()
    }
    /// Returns the state of the toggle button (pressed or not)
    pub fn is_selected(&self) -> bool {
        self.state
    }
    /// Sets the state of the toggle button (pressed or not)
    pub fn set_selected(&mut self, selected: bool) {
        self.state = selected;
    }
}
impl OnDefaultAction for ToggleButton {
    fn on_default_action(&mut self) {
        self.state = !self.state;
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::ToggleButton(EventData { status: self.state }),
        });
    }
}
impl OnKeyPressed for ToggleButton {
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

impl OnPaint for ToggleButton {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let col_text = match () {
            _ if !self.is_enabled() => theme.button.text.inactive,
            _ if self.has_focus() => {
                if self.state {
                    CharAttribute::with_color(Color::Black, Color::Yellow)
                } else {
                    CharAttribute::with_color(Color::Black, Color::Silver)
                }
            }
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => {
                if self.state {
                    CharAttribute::with_color(Color::Yellow, Color::Transparent)
                } else {
                    CharAttribute::with_color(Color::Silver, Color::Transparent)
                }
            }
        };
        let w = self.size().width;
        let format = TextFormatBuilder::new()
            .position((w / 2) as i32, 0)
            .attribute(col_text)
            .align(TextAlignament::Center)
            .chars_count(self.caption.chars_count() as u16)
            .wrap_type(WrapType::SingleLineWrap(w as u16))
            .build();

        surface.clear(Character::with_attributes(' ', col_text));
        surface.write_text(self.caption.as_str(), &format);
        if self.state {
            surface.fill_horizontal_line(0, 1, self.size().width as i32, Character::with_attributes('▔', col_text));
        }
    }
}
impl OnMouseEvent for ToggleButton {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                self.show_tooltip(&self.tooltip);
                EventProcessStatus::Processed
            }
            MouseEvent::Leave => EventProcessStatus::Processed,
            MouseEvent::Pressed(_) => {
                self.on_default_action();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
