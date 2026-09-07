use super::CharPicker;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::CharPicker`].
///
/// Implement this on a window to react when the chosen character changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait CharPickerEvents {
    /// Called when the selection changes. `code` is `None` if the picker was cleared.
    fn on_char_changed(&mut self, _handle: Handle<CharPicker>, _code: Option<char>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) code: char
}
