use std::any::TypeId;
use crate::{prelude::Point, system::Handle, ui::common::traits::EventProcessStatus};

pub trait GenericGraphViewEvents {
    fn on_current_node_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_node_action(&mut self, _handle: Handle<()>, _type_id: TypeId, _node_index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_request_new_node(&mut self, _handle: Handle<()>, _type_id: TypeId, _p: Point) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }    
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy,Clone)]
pub(crate) enum GraphViewEventTypes {
    CurrentNodeChanged,
    NodeAction(usize),
    RequestNewNode(Point),
    SelectionChanged,
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: GraphViewEventTypes,
    pub(crate) type_id: std::any::TypeId
}
