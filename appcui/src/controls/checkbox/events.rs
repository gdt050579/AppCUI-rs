use crate::system::Handle;

pub trait CheckBoxEvents {
    fn on_status_changed(&mut self, _checbox_handle: Handle, _checked: bool) {}
}
#[derive(Copy,Clone)]
pub(crate) struct EventData {
    pub (crate) checked: bool
}
