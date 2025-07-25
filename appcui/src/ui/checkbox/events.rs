use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::CheckBox;

pub trait CheckBoxEvents {
    fn on_status_changed(&mut self, _handle: Handle<CheckBox>, _checked: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy,Clone)]
pub(crate) struct EventData {
    pub (crate) checked: bool
}
