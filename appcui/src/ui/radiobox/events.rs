use super::RadioBox;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait RadioBoxEvents {
    fn on_selected(&mut self, _handle: Handle<RadioBox>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {}
