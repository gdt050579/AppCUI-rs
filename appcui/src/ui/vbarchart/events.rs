//! Events emitted by a [`struct@super::VBarChart`].
//!
//! Implement [`GenericVBarChartEvents`] on a window (or other parent) to react to
//! bar chart interaction.

use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::VBarChart`].
///
/// Because the chart is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericVBarChartEvents {
    /// Called when a bar is selected (double-clicked).
    fn on_bar_selected(&mut self, _handle: Handle<()>, _type_id: TypeId, _bar_index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the selection is cleared.
    fn on_clear_selection(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub(crate) struct EventData {
    pub(crate) type_id: std::any::TypeId,
    pub(crate) bar_index: u32,
}
