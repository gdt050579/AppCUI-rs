use crate::{system::Handle, utils::Caption, utils::ExtractHotKeyMethod};

use super::Menu;

pub(super) struct MenuBarItem {
    pub(super) caption: Caption,
    pub(super) x: i32,
    pub(super) handle: Handle<Menu>,
    pub(super) receiver_control_handle: Handle<()>,
}
impl MenuBarItem {
    pub(super) fn new(handle: Handle<Menu>, control_handle: Handle<()>, caption: &Caption) -> Self {
        Self {
            x: 0,
            handle,
            receiver_control_handle: control_handle,
            caption: if caption.chars_count() == 0 {
                Caption::new("?", ExtractHotKeyMethod::NoHotKey)
            } else {
                caption.clone()
            },
        }
    }
    pub(super) fn set(&mut self, handle: Handle<Menu>, control_handle: Handle<()>, caption: &Caption) {
        self.x = 0;
        self.handle = handle;
        self.receiver_control_handle = control_handle;
        if caption.chars_count() == 0 {
            self.caption.set_text("?", ExtractHotKeyMethod::NoHotKey);
        } else {
            self.caption.copy_from(caption);
        }
    }
}
