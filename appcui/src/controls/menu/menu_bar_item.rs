use crate::{graphics::Surface, system::Theme, utils::Caption};

use super::{Menu, MenuHandle};

pub(super) struct MenuBarItem {
    pub(super) caption: Caption,
    pub(super) menu: Menu,
    pub(super) x: i32,
    pub(super) handle: MenuHandle,
}

impl MenuBarItem {
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme) {}
}
