use crate::prelude::*;
use super::Flags;
use std::marker::PhantomData;

#[CustomControl(internal = true)]
pub struct VStackedBarChart<T>
where
    T: Number + 'static,
{
    _marker: PhantomData<T>,
    flags: Flags,
}

impl<T> VStackedBarChart<T>
where
    T: Number + 'static,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        let extra = if flags.contains(Flags::ScrollBars) {
            StatusFlags::IncreaseBottomMarginOnFocus
        } else {
            StatusFlags::None
        };        
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | extra),
            flags,
            _marker: PhantomData,
        }
    }
}
