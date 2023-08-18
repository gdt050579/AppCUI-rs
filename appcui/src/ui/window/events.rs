use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait WindowEvents {
    fn on_activate(&mut self, _window_handle: Handle) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_close(&mut self, _window_handle: Handle) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;
