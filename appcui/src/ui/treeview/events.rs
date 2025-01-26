use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait GenericTreeViewEvents {
    fn on_current_item_changed(&mut self, _handle: Handle<()>, _type_id: TypeId, _current_item: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_collapsed(&mut self, _handle: Handle<()>, _type_id: TypeId, _item: Handle<()>, _recursive: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_expanded(&mut self, _handle: Handle<()>, _type_id: TypeId, _item: Handle<()>, _recursive: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_action(&mut self, _handle: Handle<()>, _type_id: TypeId, _current_item: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

}

#[derive(Copy,Clone)]
pub(crate) enum TreeViewEventTypes {
    CurrentItemChanged(Handle<()>),
    ItemCollapsed(Handle<()>, bool),
    ItemExpanded(Handle<()>, bool),
    ItemAction(Handle<()>)
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: TreeViewEventTypes,
    pub(crate) type_id: std::any::TypeId
}