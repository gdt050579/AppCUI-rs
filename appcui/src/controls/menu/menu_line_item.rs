use crate::{
    graphics::{Surface, TextFormat, LineType}, system::MenuTheme,
};

pub(super) struct MenuLineItem {
}
impl MenuLineItem {
    pub (super) fn paint(
        &self,
        surface: &mut Surface,
        y: i32,
        width: u16,
        color: &MenuTheme
    ) {

        surface.draw_horizontal_line_with_size(
            1,
            y,
            width as u32,
            LineType::Single,
            color.text.normal,
        );

    }
}
