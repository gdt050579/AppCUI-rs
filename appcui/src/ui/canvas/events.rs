use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Canvas;

pub trait CanvasEvents {
    fn on_resize(&mut self, _handle: Handle<Canvas>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;