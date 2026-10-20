//! Events emitted by a [`struct@super::PieChart`].
//!
//! Implement [`PieChartEvents`] on a window (or other parent) to react when the
//! user selects a slice or clears the selection.

use super::PieChart;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::PieChart`].
///
/// These events are raised by the user (mouse clicks, focus changes), not when the
/// selection is changed from code with
/// [`set_selected_slice`](super::PieChart::set_selected_slice) or
/// [`clear_selected_slice`](super::PieChart::clear_selected_slice).
/// Default methods return [`EventProcessStatus::Ignored`].
pub trait PieChartEvents {
    /// Called when the user selects the slice at `index` (by clicking it).
    fn on_slice_selected(&mut self, _handle: Handle<PieChart>, _index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the selection is cleared by the user (by clicking outside any slice,
    /// or when the chart loses the focus).
    fn on_clear_selection(&mut self, _handle: Handle<PieChart>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
pub(crate) enum EventType {
    SliceSelected(usize),
    ClearSelection,
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: EventType,
}
