use super::KeySelector;
use crate::{input::Key, system::Handle, ui::common::traits::EventProcessStatus};

pub trait KeySelectorEvents {
    fn on_key_selected(&mut self, handle: Handle<KeySelector>, new_key: Key, old_key: Key) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    new_key: Key,
    old_key: Key,
}
