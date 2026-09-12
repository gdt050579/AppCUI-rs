//! Events emitted by a [`struct@super::ListView`].
//!
//! Implement [`GenericListViewEvents`] on a window (or other parent) to react to
//! item, group, and selection changes.

use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Group;

/// Events from a [`struct@super::ListView`].
///
/// Because the list is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericListViewEvents {
    /// Called when the highlighted item changes.
    fn on_current_item_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when `group` is collapsed.
    fn on_group_collapsed(&mut self, _handle: Handle<()>, _type_id: TypeId, _group: Group) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when `group` is expanded.
    fn on_group_expanded(&mut self, _handle: Handle<()>, _type_id: TypeId, _group: Group) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the set of selected items changes.
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the user activates the item at `index` (Enter or double-click).
    fn on_item_action(&mut self, _handle: Handle<()>, _type_id: TypeId, _index: usize) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

}

#[derive(Copy,Clone)]
pub(crate) enum ListViewEventTypes {
    CurrentItemChanged,
    GroupFoldedOrUnfolded(Group, bool),
    SelectionChanged,
    ItemAction(usize),
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: ListViewEventTypes,
    pub(crate) type_id: std::any::TypeId
}