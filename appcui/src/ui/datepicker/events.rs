use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::DatePicker;
use chrono::NaiveDate;

/// Events from a [`struct@super::DatePicker`].
///
/// Implement this on a window to react when the chosen date changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait DatePickerEvents {
    /// Called when the selected `date` changes.
    fn on_date_changed(&mut self, _handle: Handle<DatePicker>, _date: NaiveDate) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData{
    pub date: NaiveDate,
}