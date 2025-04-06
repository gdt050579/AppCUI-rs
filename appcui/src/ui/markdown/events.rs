use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Markdown;

pub trait MarkdownEvents {
    fn on_external_link(&mut self, _handle: Handle<Markdown>, _link: &str) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
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