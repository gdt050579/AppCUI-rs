use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Tab;

pub trait TabEvents {
    fn on_tab_changed(&mut self, _handle: Handle<Tab>, _new_tab_index: u32, _old_tabl_index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) new_tab_index: u32,
    pub(crate) old_tab_index: u32,
}
