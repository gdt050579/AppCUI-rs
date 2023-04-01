use crate::{utils::Caption, graphics::Surface, system::Theme};

use super::Menu;

pub (super) struct MenuBarItem {
    pub (super) caption: Caption,
    pub (super) menu: Menu,
    pub (super) x: i32,
}

impl MenuBarItem {
    pub (super) fn paint(&self, surface: &mut Surface, theme: &Theme) {

    }
}