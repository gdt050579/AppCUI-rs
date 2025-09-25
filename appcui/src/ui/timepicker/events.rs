use super::TimePicker;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};


pub trait TimePickerEvents {
    fn on_time_changed(&mut self, _handle: Handle<TimePicker>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {}
