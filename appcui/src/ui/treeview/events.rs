use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::TreeView`].
///
/// Because the tree is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Item handles are similarly type-erased. Default methods return
/// [`EventProcessStatus::Ignored`].
pub trait GenericTreeViewEvents {
    /// Called when the highlighted item changes to `current_item`.
    fn on_current_item_changed(&mut self, _handle: Handle<()>, _type_id: TypeId, _current_item: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when `item` is collapsed. `recursive` is true if children were collapsed too.
    fn on_item_collapsed(&mut self, _handle: Handle<()>, _type_id: TypeId, _item: Handle<()>, _recursive: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when `item` is expanded. `recursive` is true if children were expanded too.
    fn on_item_expanded(&mut self, _handle: Handle<()>, _type_id: TypeId, _item: Handle<()>, _recursive: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the set of selected items changes.
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the user activates `current_item` (Enter or double-click).
    fn on_item_action(&mut self, _handle: Handle<()>, _type_id: TypeId, _current_item: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

}

#[derive(Copy,Clone)]
pub(crate) enum TreeViewEventTypes {
    CurrentItemChanged(Handle<()>),
    ItemCollapsed(Handle<()>, bool),
    ItemExpanded(Handle<()>, bool),
    ItemAction(Handle<()>),
    SelectionChanged,
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: TreeViewEventTypes,
    pub(crate) type_id: std::any::TypeId
}