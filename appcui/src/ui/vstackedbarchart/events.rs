//! Events emitted by a [`struct@super::VStackedBarChart`].
//!
//! Implement [`GenericVStackedBarChartEvents`] on a window (or other parent) to react to
//! stacked bar chart interaction.

use std::any::TypeId;

use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::VStackedBarChart`].
///
/// Because the chart is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericVStackedBarChartEvents {
    /// Called when the selection is cleared by clicking outside any segment.
    fn on_clear_selection(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when a whole stack is selected.
    ///
    /// A double-click on a stack, or Left and Right, select the whole column. `index` is the stack index.
    fn on_stack_selected(&mut self, _handle: Handle<()>, _type_id: TypeId, _index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when a segment is selected.
    ///
    /// A click on a segment, or Up and Down, select one segment. `stack_index` is the column
    /// and `cell_index` is the segment within that column.
    fn on_cell_selected(&mut self, _handle: Handle<()>, _type_id: TypeId, _stack_index: u32, _cell_index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum EventType {
    ClearSelection,
    StackSelected(u32),
    CellSelected { stack_index: u32, cell_index: u32 },
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: TypeId,
    pub(crate) event_type: EventType,
}

