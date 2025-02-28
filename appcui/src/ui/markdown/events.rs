use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Markdown;

pub trait MarkdownEvents {
    fn on_external_link(&mut self, _handle: Handle<Markdown>, link: &str) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) link: String
};
