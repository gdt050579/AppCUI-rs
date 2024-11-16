use crate::prelude::*;
use crate::ui::togglebutton::events::EventData;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct ToggleButton {
    caption: String,
    tooltip: String,
    state: bool,
}
impl ToggleButton {
    // pub fn new(caption: &str, tooltip: &str, layout: Layout) -> Self {
    //     let mut but = ToggleButton {
    //         base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
    //         caption: Caption::new(caption, ExtractHotKeyMethod::AltPlusKey),
    //         state: false,
    //     };

    //     but.set_size_bounds(1, 1, u16::MAX, 1);
    //     but
    // }
    // pub fn set_caption(&mut self, caption: &str) {
    //     self.caption.set_text(caption, ExtractHotKeyMethod::AltPlusKey);
    //     let hotkey = self.caption.hotkey();
    //     self.set_hotkey(hotkey);
    // }
    // /// Returns the toggle button caption.
    // pub fn caption(&self) -> &str {
    //     self.caption.text()
    // }
}
impl OnDefaultAction for ToggleButton {
    fn on_default_action(&mut self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::ToggleButton(EventData {}),
        // });
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
            _ if self.has_focus() => theme.button.text.focused,
            _ if self.is_mouse_over() => theme.button.text.hovered,
            _ => theme.button.text.normal,
        };
        // let w = self.size().width;
        // let mut format = TextFormatBuilder::new()
        //     .position((w / 2) as i32, 0)
        //     .attribute(col_text)
        //     .align(TextAlignament::Center)
        //     .chars_count(self.caption.chars_count() as u16)
        //     .wrap_type(WrapType::SingleLineWrap(w as u16))
        //     .build();

        // surface.clear(Character::with_attributes(' ', col_text));
        // surface.write_text(self.caption.text(), &format);
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
