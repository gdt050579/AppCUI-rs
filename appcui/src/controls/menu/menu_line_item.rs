use crate::{
    graphics::{Surface, TextFormat, LineType},
};

pub(super) struct MenuLineItem {
}
impl MenuLineItem {
    pub (super) fn paint(
        &self,
        surface: &mut Surface,
        format: &TextFormat,
        width: u16,
    ) {

        surface.draw_horizontal_line_with_size(
            1,
            format.y,
            width as u32,
            LineType::Single,
            format.char_attr,
        );

    }
}
