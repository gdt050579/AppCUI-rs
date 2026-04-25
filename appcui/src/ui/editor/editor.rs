use crate::prelude::*;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct Editor {}

impl Editor {
    pub fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
        }
    }
}

impl OnPaint for Editor {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {
        // Intentionally empty: this is a scaffold control.
    }
}

impl OnKeyPressed for Editor {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl OnMouseEvent for Editor {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
