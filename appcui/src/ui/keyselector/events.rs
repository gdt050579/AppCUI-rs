use super::KeySelector;
use crate::{input::Key, system::Handle, ui::common::traits::EventProcessStatus};

pub trait KeySelectorEvents {
    fn on_key_changed(&mut self, _handle: Handle<KeySelector>, _new_key: Key, _old_key: Key) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) new_key: Key,
    pub(crate) old_key: Key,
}
