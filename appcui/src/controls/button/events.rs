use crate::system::Handle;

pub trait ButtonEvents {
    fn on_pressed(&mut self, button_handle: Handle) {}
}
#[derive(Copy,Clone)]
pub(crate) struct EventData {
    button_handle: Handle
}