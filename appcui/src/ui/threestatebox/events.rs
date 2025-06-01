use super::{State, ThreeStateBox};
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait ThreeStateBoxEvents {
    fn on_status_changed(&mut self, _handle: Handle<ThreeStateBox>, _state: State) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) state: State,
}
