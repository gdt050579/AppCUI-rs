use std::any::TypeId;

use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait GenericVStackedBarChartEvents {
    fn on_clear_selection(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_stack_selected(&mut self, _handle: Handle<()>, _type_id: TypeId, _index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
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

