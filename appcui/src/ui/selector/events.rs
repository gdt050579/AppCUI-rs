use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use std::any::TypeId;

pub trait GenericSelectorEvents {
    fn on_selection_changed(&mut self, _handle: Handle<()>, _type_id: TypeId) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: std::any::TypeId,
}
