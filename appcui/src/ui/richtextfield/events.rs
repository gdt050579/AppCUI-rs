//! Events emitted by a [`struct@super::RichTextField`].
//!
//! Implement [`RichTextFieldEvents`] on a window (or other parent) to react when
//! the user confirms or edits the text.

use crate::system::Handle;
use crate::ui::common::traits::EventProcessStatus;

pub use crate::ui::textfield::events::TextFieldEventsType;

/// Events from a [`struct@super::RichTextField`].
///
/// Implement this on a window to react when the user confirms or edits the text.
/// Default methods return [`EventProcessStatus::Ignored`].
pub trait RichTextFieldEvents {
    /// Called when the user confirms the text (Enter or similar).
    fn on_validate(&mut self, _handle: Handle<super::RichTextField>, _text: &str) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called after the text content changes.
    fn on_text_changed(&mut self, _handle: Handle<super::RichTextField>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
