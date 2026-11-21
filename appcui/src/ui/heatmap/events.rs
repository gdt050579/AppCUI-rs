//! Events emitted by a [`struct@super::HeatMap`].
//!
//! Implement [`GenericHeatMapEvents`] on a window (or other parent) to react when the
//! user selects a cell or clears the selection. With the `#[Window]` macro, use
//! `events = HeatMapEvents<T>` to get a typed version of these callbacks.

use std::any::TypeId;

use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::HeatMap`].
///
/// Because the heat map is generic, the control is identified by a type-erased `handle`
/// and `type_id`. These events are raised by the user (a click or a key), not when the
/// selection is changed from code with [`set_selected_cell`](super::HeatMap::set_selected_cell)
/// or when the cells are removed with [`clear`](super::HeatMap::clear) or
/// [`clear_all`](super::HeatMap::clear_all).
/// Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericHeatMapEvents {
    /// Called when the user selects the cell at `row` and `column` (by clicking it, or with the arrow keys).
    fn on_cell_selected(&mut self, _handle: Handle<()>, _type_id: TypeId, _row: u32, _column: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the user clears the selection (by clicking an empty cell or the margin, or with `Escape`).
    fn on_clear_selection(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
pub(crate) enum EventType {
    CellSelected { row: u32, column: u32 },
    ClearSelection,
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: TypeId,
    pub(crate) event_type: EventType,
}
