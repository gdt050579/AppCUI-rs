use std::marker::PhantomData;

use super::Flags;
use crate::prelude::*;

#[CustomControl(internal = true)]
/// A line chart for one or more numeric series of type `T`.
///
/// Each series is a sequence of values sharing the same category (X) axis, so several
/// lines can be drawn over the same range. A single-series chart is simply the case of
/// one series. Optional behavior is controlled by [`Flags`].
pub struct LineChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    _phantom: PhantomData<T>,
}

impl<T> LineChart<T>
where
    T: Number + 'static,
{
    /// Creates an empty line chart.
    ///
    /// `layout` places the control. `flags` selects optional behavior; see [`Flags`].
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let chart = LineChart::<i32>::new(layout!("d:f"), linechart::Flags::None);
    /// ```
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            _phantom: PhantomData,
        }
    }

    /// Returns the flags the chart was created with.
    #[inline(always)]
    pub fn flags(&self) -> Flags {
        self.flags
    }
}
