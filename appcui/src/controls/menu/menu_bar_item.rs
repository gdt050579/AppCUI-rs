use super::MenuHandle;
use crate::utils::Caption;

pub(super) struct MenuBarItem {
    pub(super) caption: Caption,
    pub(super) x: i32,
    pub(super) handle: MenuHandle,
}
impl MenuBarItem {
    pub(super) fn new(handle: MenuHandle, caption: &Caption) -> Self {
        Self {
            x: 0,
            handle,
            caption: if caption.get_chars_count() == 0 {
                Caption::new("?", false)
            } else {
                caption.clone()
            },
        }
    }
    pub (super) fn set(&mut self, handle: MenuHandle, caption: &Caption) {
        self.x = 0;
        self.handle = handle;
        if caption.get_chars_count() == 0 {
            self.caption.set_text("?", false);
        } else {
            self.caption.copy_from(caption);
        }
    }
}
