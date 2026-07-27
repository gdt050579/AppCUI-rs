use super::HSlider;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait HSliderEvents {
    fn on_open(&mut self, _handle: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;
