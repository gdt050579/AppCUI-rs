use super::ComboBox;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

pub trait ComboBoxEvents {
    fn on_selection_changed(&mut self, _handle: Handle<ComboBox>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {}
