use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait GenericSelectorEvents {
    fn on_selection_changed(&mut self, _handle: Handle<()>, _hash: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {}