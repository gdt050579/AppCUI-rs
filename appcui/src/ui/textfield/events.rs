use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::TextField;

pub trait TextFieldEvents {
    fn on_something(&mut self, _handle: Handle<TextField>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;
