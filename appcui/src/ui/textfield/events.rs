use super::TextField;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

#[derive(Copy, Clone, Eq, PartialEq)]
/// Which [`TextField`] event is being delivered to the host.
///
/// `OnValidate` is raised when the user confirms the text; `OnTextChanged` after
/// each edit.
pub enum TextFieldEventsType {
    /// The user confirmed the text (Enter or similar).
    OnValidate,
    /// The text content changed.
    OnTextChanged,
}

pub trait TextFieldEvents {
    fn on_validate(&mut self, _handle: Handle<TextField>, _text: &str) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_text_changed(&mut self, _handle: Handle<TextField>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) evtype: TextFieldEventsType,
}
