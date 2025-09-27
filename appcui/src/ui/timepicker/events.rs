use super::TimePicker;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use chrono::NaiveTime;

pub trait TimePickerEvents {
    fn on_time_changed(&mut self, _handle: Handle<TimePicker>, _time: NaiveTime) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub time: NaiveTime,
}
