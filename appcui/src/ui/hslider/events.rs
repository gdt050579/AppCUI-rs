use std::any::TypeId;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from an [`struct@super::HSlider`].
///
/// Because the slider is generic, the control is identified by a type-erased `handle`
/// and `type_id`. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericHSliderEvents {
    /// Called when the slider value changes.
    fn on_value_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: std::any::TypeId
}
