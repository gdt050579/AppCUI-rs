//! Events emitted by a [`struct@super::TimePicker`].
//!
//! Implement [`TimePickerEvents`] on a window (or other parent) to react when
//! the chosen time changes.

use super::TimePicker;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use chrono::NaiveTime;

/// Events from a [`struct@super::TimePicker`].
///
/// Implement this on a window to react when the chosen time changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait TimePickerEvents {
    /// Called when the selected `time` changes.
    fn on_time_changed(&mut self, _handle: Handle<TimePicker>, _time: NaiveTime) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub time: NaiveTime,
}
