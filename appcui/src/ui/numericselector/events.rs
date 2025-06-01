use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use std::any::TypeId;

pub trait GenericNumericSelectorEvents {
    fn on_value_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: std::any::TypeId,
}
