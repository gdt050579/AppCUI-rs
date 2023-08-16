pub trait ButtonEvents {
    fn on_clicked(&mut self, button_handle: Handle) {}
}
#[derive(Copy,Clone)]
pub(crate) struct ButtonEventsWrapper {
    button_handle: Handle
}