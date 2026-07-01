use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait GenericBufferViewEvents {
    fn on_current_pos_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
pub(crate) enum BufferViewEventTypes {
    CurrentPosChanged,
    SelectionChanged,
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: BufferViewEventTypes,
    pub(crate) type_id: std::any::TypeId,
}
