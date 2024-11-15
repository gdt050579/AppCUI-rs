use crate::prelude::*;
use crate::ui::password::events::EventData;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct Password {
    pass: String,
    visible: bool,
}
impl Password {
    pub fn new(layout: Layout) -> Self {
        let mut p = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            pass: String::new(),
            visible: false,
        };
        p.set_size_bounds(4, 1, u16::MAX, 1);
        p
    }
    pub fn set_password(&mut self, password: &str) {
        self.pass.clear();
        self.pass.push_str(password);
    }
    #[inline(always)]
    pub fn password(&self) -> &str {
        &self.pass
    }
}
impl OnPaint for Password {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        let mut sz = self.pass.len() as u32;
        let w = self.size().width;
        if (sz + 3) >= w {
            sz = w - 3;
        }
        surface.fill_horizontal_line(0, 0, w as i32, Character::with_attributes(' ', attr));
        if self.visible {
            if sz > 0 {
                let format = TextFormatBuilder::new()
                    .position(1, 0)
                    .attribute(attr)
                    .align(TextAlignament::Left)
                    .singleline_width(sz as u16)
                    .build();
                surface.write_text(&self.pass, &format);
            }
            surface.write_char(
                w as i32 - 1,
                0,
                Character::with_attributes(SpecialChar::CircleFilled, theme.symbol.checked),
            );
        } else {
            surface.fill_horizontal_line_with_size(1, 0, sz, Character::with_attributes('*', attr));
            surface.write_char(
                w as i32 - 1,
                0,
                Character::with_attributes(SpecialChar::CircleEmpty, theme.symbol.unchecked),
            );
        }
        if self.has_focus() {
            surface.set_cursor(1 + sz as i32, 0);
        }
    }
}
impl OnKeyPressed for Password {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        if character as u32 > 0 {
            self.pass.push(character);
            return EventProcessStatus::Processed;
        }
        match key.value() {
            key!("Back") => {
                self.pass.pop();
                return EventProcessStatus::Processed;
            }
            key!("Enter") => {
                self.raise_event(ControlEvent {
                    emitter: self.handle,
                    receiver: self.event_processor,
                    data: ControlEventData::Password(EventData { accept: true }),
                });
                return EventProcessStatus::Processed;
            }
            key!("Esc") => {
                self.raise_event(ControlEvent {
                    emitter: self.handle,
                    receiver: self.event_processor,
                    data: ControlEventData::Password(EventData { accept: false }),
                });
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for Password {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => {
                self.show_tooltip("Click and hold to see the password");
                EventProcessStatus::Processed
            }
            MouseEvent::Leave => {
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
            MouseEvent::Over(_) => EventProcessStatus::Ignored,
            MouseEvent::Pressed(_) => {
                self.visible = true;
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.visible = false;
                EventProcessStatus::Processed
            }
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(_) => EventProcessStatus::Ignored,
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}
