use super::Group;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use std::any::TypeId;

pub trait GenericListViewEvents {
    fn on_current_item_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_group_collapsed(&mut self, _handle: Handle<()>, _type_id: TypeId, _group: Group) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_group_expanded(&mut self, _handle: Handle<()>, _type_id: TypeId, _group: Group) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_item_action(&mut self, _handle: Handle<()>, _type_id: TypeId, _index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
pub(crate) enum ListViewEventTypes {
    CurrentItemChanged,
    GroupFoldedOrUnfolded(Group, bool),
    SelectionChanged,
    ItemAction(usize),
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: ListViewEventTypes,
    pub(crate) type_id: std::any::TypeId,
}
