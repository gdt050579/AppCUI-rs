use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use std::any::TypeId;

pub trait GenericHNumericSliderEvents {
    fn on_value_changed(&mut self, _handle: Handle<()>, _type_id: TypeId, /*old_value: u128, new_value: u128*/) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) type_id: std::any::TypeId,
    // pub(crate) old_value: u128,
    // pub(crate) new_value: u128
}
