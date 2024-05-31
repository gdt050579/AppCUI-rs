use std::ops::{Add, Sub};

use super::events::EventData;
use super::Buttons;
use super::Flags;
use crate::prelude::*;

pub trait Numeric: Add<Output = Self> + Sub<Output = Self> + Copy + Clone + PartialOrd + PartialEq {}

impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for u128 {}
impl Numeric for usize {}
impl Numeric for isize {}
impl Numeric for f32 {}
impl Numeric for f64 {}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct NumericSelector<T>
where
    T: Numeric + 'static,
{
    value: T,
    min: T,
    max: T,
    flags: Flags,
    buttons: Buttons,
}
impl<T> NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn to_interval(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
    pub fn new(value: T, min: T, max: T, layout: Layout, flags: Flags) -> Self {
        let v_min = if min < max { min } else { max };
        let v_max = if max > min { max } else { min };
        let v = Self::to_interval(value, v_min, v_max);
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            min: v_min,
            max: v_max,
            value: v,
            flags,
            buttons: Buttons::new(),
        };
        obj.buttons.update_width(obj.size().width as u16);
        obj.set_size_bounds(11, 1, u16::MAX, 1);
        obj
    }
    #[inline(always)]
    pub fn value(&self) -> T {
        self.value.clone()
    }
    pub fn set_value(&mut self, value: T) {
        self.value = Self::to_interval(value, self.min, self.max);
    }

    fn emit_on_selection_changed_event(&mut self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::Selector(EventData {
        //         type_id: std::any::TypeId::of::<T>(),
        //     }),
        // });
    }
}
impl<T> OnPaint for NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        self.buttons.paint(surface, theme, self.is_enabled());
    }
}
impl<T> OnKeyPressed for NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl<T> OnMouseEvent for NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl<T> OnResize for NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.buttons.update_width(new_size.width as u16);
    }
}
