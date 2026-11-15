use crate::prelude::*;
use std::marker::PhantomData;

#[CustomControl(internal = true)]
pub struct VStackedBarChart<T>
where
    T: Number + 'static,
{
    _marker: PhantomData<T>,
}

impl<T> VStackedBarChart<T>
where
    T: Number + 'static,
{
    pub fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, true),
            _marker: PhantomData,
        }
    }
}
