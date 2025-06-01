use super::Button;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait ButtonEvents {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;
