use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::Selector`].
///
/// Because [`Selector`](struct@super::Selector) is generic, the control is identified by a
/// type-erased `handle` and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericSelectorEvents {
    /// Called when the selected enum variant changes.
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: std::any::TypeId
}