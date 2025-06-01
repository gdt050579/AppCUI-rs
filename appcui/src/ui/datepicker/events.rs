use super::DatePicker;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use chrono::NaiveDate;

pub trait DatePickerEvents {
    fn on_date_change(&mut self, _handle: Handle<DatePicker>, _date: NaiveDate) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub date: NaiveDate,
}
