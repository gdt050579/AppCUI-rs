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

/// Events from a [`struct@super::TextField`].
///
/// Implement this on a window to react when the user confirms or edits the text.
/// Default methods return [`EventProcessStatus::Ignored`].
pub trait TextFieldEvents {
    /// Called when the user confirms the text (Enter, if [`super::Flags::ProcessEnter`] is set).
    fn on_validate(&mut self, _handle: Handle<TextField>, _text: &str) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called after the text content changes.
    fn on_text_changed(&mut self, _handle: Handle<TextField>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) evtype: TextFieldEventsType,
}
