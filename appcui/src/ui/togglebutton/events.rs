use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::ToggleButton;

pub trait ToggleButtonEvents {
    fn on_pressed(&mut self, _handle: Handle<ToggleButton>, _pressed: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;