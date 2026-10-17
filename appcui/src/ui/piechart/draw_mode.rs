use crate::graphics::Surface;
use crate::graphics::Theme;
use crate::graphics::LineType;
use super::Slice;
pub enum DrawModeShape {
    Circle,
    Square,
    RoundSquare,
    Rectangle,
    RoundRectangle,
    Ellipse,
}
pub enum DrawModeRenderer {
    HalfBlocks,
    Braille,
    Custom(char),
}
pub enum HollowThickness {
    None,
    Percentage(u8), // value between 0 and 100
    Absolute(u8), // value (characters) between 0 and 100 
    Rectangle(u8,u8), // width and height in characters
}
pub(crate) struct DrawMode {
    pub(crate) shape: DrawModeShape,
    pub(crate) renderer: DrawModeRenderer,
    pub(crate) hollow_thickness: HollowThickness,
    pub(crate) margins: Option<LineType>,
}
impl DrawMode {
    pub(crate) fn new(shape: DrawModeShape, renderer: DrawModeRenderer, hollow_thickness: HollowThickness, margins: Option<LineType>) -> Self {
        Self { shape, renderer, hollow_thickness, margins }
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice]) {
        match self.shape {
            DrawModeShape::Circle => self.paint_circle(surface, theme, slices),
            DrawModeShape::Square => self.paint_square(surface, theme, slices),
            DrawModeShape::RoundSquare => self.paint_round_square(surface, theme, slices),
            DrawModeShape::Rectangle => self.paint_rectangle(surface, theme, slices),
            DrawModeShape::RoundRectangle => self.paint_round_rectangle(surface, theme, slices),
            DrawModeShape::Ellipse => self.paint_ellipse(surface, theme, slices),
        }
    }
    fn paint_circle(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice]) {
        todo!()
    }
    fn paint_square(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice]) {
        todo!()
    }
    fn paint_round_square(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice]) {
        todo!()
    }
    fn paint_rectangle(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice]) {
        todo!()
    }
    fn paint_round_rectangle(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice]) {
        todo!()
    }
    fn paint_ellipse(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice]) {
        todo!()
    }
}