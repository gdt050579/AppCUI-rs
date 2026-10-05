//! Events emitted by a [`struct@super::HBarChart`].
//!
//! Implement [`GenericHBarChartEvents`] on a window (or other parent) to react to
//! bar chart interaction.

use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use std::any::TypeId;

/// Events from a [`struct@super::HBarChart`].
///
/// Because the chart is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericHBarChartEvents {
    /// Called when a bar is selected (clicked or double-clicked).
    fn on_bar_selected(&mut self, _handle: Handle<()>, _type_id: TypeId, _bar_index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the selection is cleared (clicked or double-clicked outside any bar).
    fn on_clear_selection(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub(crate) enum EventType {
    BarSelected(u32),
    ClearSelection,
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: std::any::TypeId,
    pub(crate) event_type: EventType,
}
