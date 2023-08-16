use crate::system::Handle;

pub trait ButtonEvents {
    fn on_pressed(&mut self, button_handle: Handle) {}
}
#[derive(Copy,Clone)]
pub(crate) struct ButtonEvents_OnPressed {
    button_handle: Handle
}