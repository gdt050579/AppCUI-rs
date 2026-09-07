use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::NumericSelector`].
///
/// Because the selector is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericNumericSelectorEvents {
    /// Called when the numeric value changes.
    fn on_value_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: std::any::TypeId
}