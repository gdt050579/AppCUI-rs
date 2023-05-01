use crate::utils::Caption;
use super::MenuHandle;

pub(super) struct MenuBarItem {
    pub(super) caption: Caption,
    pub(super) x: i32,
    pub(super) handle: MenuHandle,
}
