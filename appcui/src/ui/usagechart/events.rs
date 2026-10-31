//! Events emitted by a [`struct@super::UsageChart`].
//!
//! Implement [`UsageChartEvents`] on a window (or other parent) to react when the
//! user selects a segment or clears the selection.

use crate::{system::Handle, ui::common::traits::EventProcessStatus, ui::usagechart::UsageChart};

/// Events from a [`struct@super::UsageChart`].
///
/// These events are raised by the user (a click or a key), not when the selection is
/// changed from code with [`set_selected_segment`](super::UsageChart::set_selected_segment)
/// or when the segments are removed with [`clear`](super::UsageChart::clear).
/// Default methods return [`EventProcessStatus::Ignored`].
pub trait UsageChartEvents {
    /// Called when the user selects the segment at `index` (by clicking it, or with `Left` / `Right`).
    ///
    /// `index` is the segment's position after sorting. See [`SortType`](super::SortType).
    fn on_usage_segment_selected(&mut self, _handle: Handle<UsageChart>, _index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the user clears the selection (by clicking the free part of the bar, or with `Escape`).
    fn on_clear_selection(&mut self, _handle: Handle<UsageChart>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) index: Option<u32>,
}