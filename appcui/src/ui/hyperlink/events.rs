use super::HyperLink;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait HyperLinkEvents {
    fn on_open(&mut self, _handle: Handle<HyperLink>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;
