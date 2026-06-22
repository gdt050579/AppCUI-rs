use super::initialization_flags::{BufferAccess, Flags};
use crate::prelude::*;

#[CustomControl(overwrite = [OnPaint, OnKeyPressed, OnMouseEvent], internal = true)]
pub struct BufferView<T> where T: BufferAccess {
    flags: Flags,
    buffer: T,
}

impl<T: BufferAccess> BufferView<T> {
    pub fn new(buffer: T, layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            buffer,
        }
    }
}

impl<T: BufferAccess> OnPaint for BufferView<T> {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {
        // TODO: implement painting of the buffer view
    }
}

impl<T: BufferAccess> OnKeyPressed for BufferView<T> {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        // TODO: implement keyboard handling
        EventProcessStatus::Ignored
    }
}

impl<T: BufferAccess> OnMouseEvent for BufferView<T> {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        // TODO: implement mouse handling
        EventProcessStatus::Ignored
    }
}
