use crate::{graphics::Surface, system::Theme, utils::Caption};

use super::MenuHandle;

pub(super) struct MenuBarItem {
    pub(super) caption: Caption,
    pub(super) x: i32,
    pub(super) handle: MenuHandle,
}

impl MenuBarItem {
    pub(super) fn paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
