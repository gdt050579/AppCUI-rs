use super::CharPicker;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait CharPickerEvents {
    fn on_char_changed(&mut self, _handle: Handle<CharPicker>, _code: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) code: char
}
