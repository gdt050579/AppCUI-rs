use super::ToggleButton;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait ToggleButtonEvents {
    fn on_selection_changed(&mut self, _handle: Handle<ToggleButton>, _selected: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) status: bool,
}
