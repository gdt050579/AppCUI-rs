use crate::ui::common::traits::*;
pub trait DesktopEvents {
    fn on_start(&mut self) { }
    fn on_close(&mut self) -> ActionRequest {
        ActionRequest::Allow
    }
}