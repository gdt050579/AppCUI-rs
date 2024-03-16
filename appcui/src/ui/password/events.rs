use crate::{system::Handle, ui::common::traits::EventProcessStatus};
use super::Password;

pub trait PasswordEvents {
    fn on_accept(&mut self, _handle: Handle<Password>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_cancel(&mut self, _handle: Handle<Password>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData;
