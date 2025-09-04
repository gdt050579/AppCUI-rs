use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait GenericGraphViewEvents {
    fn on_current_node_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_node_action(&mut self, _handle: Handle<()>, _type_id: TypeId, _node_index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy,Clone)]
pub(crate) enum GraphViewEventTypes {
    CurrentItemChanged,
    NodeAction(usize),
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: GraphViewEventTypes,
    pub(crate) type_id: std::any::TypeId
}
