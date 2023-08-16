use crate::system::Handle;

pub trait CheckBoxEvents {
    fn on_status_changed(&mut self, checbox_handle: Handle, checked: bool) {}
}
#[derive(Copy,Clone)]
pub(crate) struct EventData {
    pub (crate) checbox_handle: Handle,
    pub (crate) checked: bool
}
