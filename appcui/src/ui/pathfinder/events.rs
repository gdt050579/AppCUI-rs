use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::PathFinder;

pub trait PathFinderEvents {
    fn on_path_updated(&mut self, _handle: Handle<PathFinder>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;

