use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Group;

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
}

#[derive(Copy,Clone)]
pub(crate) enum ListViewEventTypes {
    CurrentItemChanged,
    GroupFoldedOrUnfolded(Group, bool),
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: ListViewEventTypes,
    pub(crate) type_id: std::any::TypeId
}