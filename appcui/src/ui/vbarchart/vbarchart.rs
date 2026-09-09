use super::Flags;
use crate::prelude::*;
use std::marker::PhantomData;

#[CustomControl(overwrite=OnPaint, internal=true)]
/// A vertical bar chart for a numeric series of type `T`.
///
/// `VBarChart` displays one vertical bar per value. Visual options are controlled
/// by [`Flags`].
pub struct VBarChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    _phantom: PhantomData<T>,
}

impl<T> VBarChart<T>
where
    T: Number + 'static,
{
    /// Creates an empty vertical bar chart with the specified layout and flags.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let chart = VBarChart::<i32>::new(layout!("x:1,y:1,w:30,h:10"), vbarchart::Flags::None);
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled),
            flags,
            _phantom: PhantomData,
        }
    }
}

impl<T> OnPaint for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
