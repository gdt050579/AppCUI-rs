use super::initialization_flags::Flags;

use crate::prelude::*;

#[CustomControl(overwrite: [OnPaint, OnKeyPressed, OnMouseEvent, OnResize, OnFocus], internal=true)]
pub struct TextArea {
    flags: Flags,
    text: String,
}

impl TextArea {
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        let mut control = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            text: text.to_string(),
        };

        control
    }
}

impl OnPaint for TextArea {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(Character::new('x', Color::Blue, Color::Green, CharFlags::None));
    }
}

impl OnFocus for TextArea {}

impl OnResize for TextArea {}

impl OnKeyPressed for TextArea {}

impl OnMouseEvent for TextArea {}
