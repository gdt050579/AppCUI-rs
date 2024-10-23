use super::initialization_flags::Flags;
use crate::prelude::*;

#[CustomControl(overwrite: [OnPaint, OnMouseEvent, OnResize], internal=true)]
pub struct NumericSlider {
    flags: Flags,
    
}
impl NumericSlider {
    pub fn new(text: &str, layout: Layout, flags: Flags) -> Self {
        let control = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
        };
        control
    }
}
impl OnPaint for NumericSlider {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}

impl OnMouseEvent for NumericSlider {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl OnResize for NumericSlider{
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}
