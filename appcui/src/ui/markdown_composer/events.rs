//! Events emitted by a [`struct@super::MarkdownComposer`].
//!
//! Implement [`MarkdownComposerEvents`] on a window (or other parent) to react when
//! the user confirms or edits the mark
//! down text.

use super::MarkdownComposer;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

#[derive(Copy, Clone, Eq, PartialEq)]
/// Which [`MarkdownComposer`] event is being delivered to the host.
///
/// `OnValidate` is raised when the user confirms the text; `OnTextChanged` after
/// each edit.
pub enum MarkdownComposerEventsType {
    /// The user confirmed the text (Ctrl+Enter).
    OnValidate,
    /// The text content changed.
    OnTextChanged,
}

/// Events from a [`struct@super::MarkdownComposer`].
///
/// Implement this on a window to react when the user confirms or edits the text.
/// Default methods return [`EventProcessStatus::Ignored`].
pub trait MarkdownComposerEvents {
    /// Called when the user confirms the text with Ctrl+Enter.
    ///
    /// Plain Enter inserts a new line and does not raise this event.
    fn on_validate(&mut self, _handle: Handle<MarkdownComposer>, _text: &str) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called after the text content changes.
    ///
    /// Only edits made by the user raise this event; [`MarkdownComposer::set_text`]
    /// does not.
    fn on_text_changed(&mut self, _handle: Handle<MarkdownComposer>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) evtype: MarkdownComposerEventsType,
}
