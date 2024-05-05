use appcui::prelude::*;

#[CustomControl(overwrite: OnPaint+OnKeyPressed+OnMouseEvent)]
pub struct Board {}

impl Board {
    pub fn new() -> Self {
        Self {
            base: ControlBase::new(Layout::new("x:2,y:1,w:34,h:18"), true),
        }
    }
}

impl OnPaint for Board {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',black,black"));
        surface.draw_rect(Rect::new(0,0,33,17), LineType::Single, CharAttribute::with_color(Color::White, Color::Black));
        surface.fill_horizontal_line(1, 6, 32, char!("BoxHorizontalSingleLine,white,black"));
        surface.fill_horizontal_line(1, 12, 32, char!("BoxHorizontalSingleLine,white,black"));
        surface.fill_vertical_line(11, 1, 16, char!("BoxVerticalSingleLine,white,black"));
        surface.fill_vertical_line(22, 1, 16, char!("BoxVerticalSingleLine,white,black"));
        surface.write_char(11, 6, char!("BoxCrossSingleLine,white,black"));
        surface.write_char(11, 12, char!("BoxCrossSingleLine,white,black"));
        surface.write_char(22, 6, char!("BoxCrossSingleLine,white,black"));
        surface.write_char(22, 12, char!("BoxCrossSingleLine,white,black"));
    }
}
impl OnKeyPressed for Board {}
impl OnMouseEvent for Board {}
