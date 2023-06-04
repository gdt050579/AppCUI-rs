use super::MenuHandle;
use crate::{utils::Caption, system::Handle};

pub(super) struct MenuBarItem {
    pub(super) caption: Caption,
    pub(super) x: i32,
    pub(super) handle: MenuHandle,
    pub(super) receiver_control_handle: Handle,
}
impl MenuBarItem {
    pub(super) fn new(handle: MenuHandle, control_handle: Handle, caption: &Caption) -> Self {
        Self {
            x: 0,
            handle,
            receiver_control_handle: control_handle,
            caption: if caption.get_chars_count() == 0 {
                Caption::new("?", false)
            } else {
                caption.clone()
            },
        }
    }
    pub (super) fn set(&mut self, handle: MenuHandle, control_handle: Handle, caption: &Caption) {
        self.x = 0;
        self.handle = handle;
        self.receiver_control_handle = control_handle;
        if caption.get_chars_count() == 0 {
            self.caption.set_text("?", false);
        } else {
            self.caption.copy_from(caption);
        }
    }
}
