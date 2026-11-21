//! Events emitted by a [`struct@super::LineChart`].
//!
//! Implement [`GenericLineChartEvents`] on a window (or other parent) to react to
//! line chart interaction. With the `#[Window]` macro, use `events = LineChartEvents<T>`
//! to get a typed version of these callbacks.

use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use std::any::TypeId;

/// Events from a [`struct@super::LineChart`].
///
/// Because the chart is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericLineChartEvents {
    /// Called when a point (one series, one index on the X axis) is selected with the mouse or the keyboard.
    ///
    /// `series_index` is the index of the series (as returned by `add_series`) and
    /// `point_index` the index of the point inside that series (its position on the X axis).
    fn on_point_selected(&mut self, _handle: Handle<()>, _type_id: TypeId, _series_index: u32, _point_index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the selection is cleared (a click outside any point, or `Escape`).
    fn on_clear_selection(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
pub(crate) enum EventType {
    PointSelected { series_index: u32, point_index: u32 },
    ClearSelection,
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: TypeId,
    pub(crate) event_type: EventType,
}
