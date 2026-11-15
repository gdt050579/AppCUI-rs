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
        Self {
            base: ControlBase::new(layout, true),
            flags,
            _marker: PhantomData,
        }
    }
}
