use crate::{
    graphics::{LineType, Surface},
    system::MenuTheme,
};

use super::{menu_item::IntoMenuItem, MenuItem};

pub struct Separator {}
impl Separator {
    pub fn new() -> Self {
        Self {}
    }
    pub(super) fn paint(&self, surface: &mut Surface, y: i32, width: u16, color: &MenuTheme) {
        surface.draw_horizontal_line_with_size(1, y, width as u32, LineType::Single, color.text.normal);
    }
}
impl IntoMenuItem for Separator {
    fn into_menuitem(self) -> super::MenuItem {
        MenuItem::Separator(self)
    }
}
