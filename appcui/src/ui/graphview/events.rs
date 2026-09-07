use std::any::TypeId;
use crate::{prelude::Point, system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::GraphView`].
///
/// Because the view is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericGraphViewEvents {
    /// Called when the highlighted node changes.
    fn on_current_node_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the user activates the node at `node_index` (Enter or double-click).
    fn on_node_action(&mut self, _handle: Handle<()>, _type_id: TypeId, _node_index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the user requests a new node at point `p`.
    fn on_request_new_node(&mut self, _handle: Handle<()>, _type_id: TypeId, _p: Point) -> EventProcessStatus {
        EventProcessStatus::Ignored
    } 
    /// Called when the user requests a new edge from node `from` to node `to`.
    fn on_request_new_edge(&mut self, _handle: Handle<()>, _type_id: TypeId, _from: u32, _to: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }   
    /// Called when the set of selected nodes changes.
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy,Clone)]
pub(crate) enum GraphViewEventTypes {
    CurrentNodeChanged,
    NodeAction(usize),
    RequestNewNode(Point),
    RequestNewEdge(u32, u32),
    SelectionChanged,
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: GraphViewEventTypes,
    pub(crate) type_id: std::any::TypeId
}
