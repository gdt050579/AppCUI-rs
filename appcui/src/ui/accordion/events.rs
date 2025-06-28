use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Accordion;

pub trait AccordionEvents {
    fn on_panel_changed(&mut self, _handle: Handle<Accordion>, _new_panel_index: u32, _old_panel_index: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) new_panel_index: u32,
    pub(crate) old_panel_index: u32,
}
