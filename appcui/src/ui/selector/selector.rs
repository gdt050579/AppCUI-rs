use std::marker::PhantomData;

use super::EnumSelector;
use super::Flags;
use crate::prelude::*;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnExpand, internal=true)]
pub struct Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    current_index: u32,
    flags: Flags,
    _phanton: PhantomData<T>
}
impl<T> Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    pub fn new(value: Option<T>, layout: Layout, flags: Flags) -> Self {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            current_index: u32::MAX,
            flags,
            _phanton: PhantomData,
        };
        if let Some(val) = value {
            let count = T::count();
            for i in 0..count {
                if T::from_index(i) == Some(val) {
                    obj.current_index = i;
                    break;
                }
            }
        } else {
            // value is None
            if !obj.flags.contains(Flags::AllowNoneVariant) {
                panic!(
                    "You can not instantiate a selector with `None` value without setting the flags `AllowNoneVariant`. Have you forgot to do this ?"
                );
            }
        }
        obj.set_size_bounds(7, 1, u16::MAX, 1);
        obj
    }
    #[inline(always)]
    pub fn value(&self) -> T {
        EnumSelector::from_index(self.current_index).unwrap()
    }
    #[inline(always)]
    pub fn try_value(&self)->Option<T> {
        EnumSelector::from_index(self.current_index)
    }
}
impl<T> OnPaint for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
impl<T> OnExpand for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_expand(&mut self, _direction: ExpandedDirection) {}

    fn on_pack(&mut self) {}
}
impl<T> OnDefaultAction for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_default_action(&mut self) {}
}
impl<T> OnKeyPressed for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl<T> OnMouseEvent for Selector<T>
where
    T: EnumSelector + Copy + Eq,
{
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
