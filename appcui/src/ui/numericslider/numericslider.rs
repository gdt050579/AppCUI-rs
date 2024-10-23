use super::initialization_flags::Flags;
use crate::prelude::*;
use crate::ui::common::Number;

#[CustomControl(overwrite: [OnPaint, OnMouseEvent, OnResize], internal=true)]
pub struct NumericSlider<T>
where
    T: Number + 'static,
{
    flags: Flags,
    min: T,
    max: T,
    step: T,
    value: T,
}
impl<T> NumericSlider<T>
where
    T: Number + 'static,
{
    pub fn new(min:T, max:T, step: T, value:T , layout: Layout, flags: Flags) -> Self {
        let control = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            min,
            max,
            step,
            value,
        };
        control
    }
}
impl<T> OnPaint for NumericSlider<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        //surface.clear(Character::new(SpecialChar::SingleLineDownT, Color::Blue, Color::Black, CharFlags::None));
        
        surface.clear(char!("^|^,b,black"));
    }
}

impl<T> OnMouseEvent for NumericSlider<T>
where
    T: Number + 'static,
{
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl<T> OnResize for NumericSlider<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}
