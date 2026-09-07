use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Markdown;

/// Events from a [`struct@super::Markdown`].
///
/// Implement this on a window to react to link clicks and history navigation.
/// Default methods return [`EventProcessStatus::Ignored`].
pub trait MarkdownEvents {
    /// Called when the user activates an external `link` in the document.
    fn on_external_link(&mut self, _handle: Handle<Markdown>, _link: &str) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// Called when the user navigates back with Backspace.
    fn on_backspace_navigation(&mut self, _handle: Handle<Markdown>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

#[derive(Clone)]
pub(crate) enum Data {
    BackEvent,
    LinkClickEvent(String),
}

#[derive(Clone)]
pub(crate) struct EventData {
    pub(crate) event_type: Data
}