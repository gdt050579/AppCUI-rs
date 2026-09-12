//! Events emitted by a [`struct@super::DropDownList`].
//!
//! Implement [`GenericDropDownListEvents`] on a window (or other parent) to react
//! when the selected item changes.

use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::DropDownList`].
///
/// Because the list is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericDropDownListEvents {
    /// Called when the selected item changes.
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: std::any::TypeId
}