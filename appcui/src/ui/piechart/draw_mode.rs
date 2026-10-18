use super::shapes::Ellipse;
use super::{Slice, PaintMap};
use crate::graphics::Rect;
use crate::graphics::Size;
use crate::graphics::Surface;
use crate::system::Theme;

pub(crate) const PADDING: u32 = 1;

/// The outline a [`PieChart`](super::PieChart) is drawn in.
///
/// Shapes are fitted in the plot area: the control minus one character
/// on every side.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum DrawModeShape {
    /// The largest circle that fits the plot area, centered. This is the default.
    #[default]
    Circle,
    /// The largest square that fits the plot area, centered.
    Square,
    /// Like [`Square`](Self::Square), with rounded corners.
    RoundSquare,
    /// A rectangle that fills the plot area.
    Rectangle,
    /// Like [`Rectangle`](Self::Rectangle), with rounded corners.
    RoundRectangle,
    /// An ellipse that fills the plot area.
    Ellipse,
}

/// How the pixels of a [`PieChart`](super::PieChart) are turned into characters.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum DrawModeRenderer {
    /// Half blocks (`▀`): two pixels per character, each with its own color. This is the default.
    #[default]
    HalfBlocks,
    /// Braille dots: 2x4 pixels per character. A character takes the color of the
    /// slice that covers most of its dots.
    Braille,
    /// One pixel per character, drawn with the given character in the slice color.
    Custom(char),
}

/// The size of the hole in the middle of a [`PieChart`](super::PieChart).
///
/// Every variant describes the hole itself; the slices are drawn in the band
/// between the hole and the outline. A hole larger than the shape is capped at
/// the shape's size (nothing is drawn).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum HollowThickness {
    /// No hole: the shape is filled completely. This is the default.
    #[default]
    None,
    /// The hole is this percentage (`0`..=`100`) of the shape's size, with the
    /// same proportions as the shape.
    Percentage(u8),
    /// The hole is this many characters wide (`0`..=`100`), with the same
    /// proportions as the shape.
    Absolute(u8),
    /// The hole is `width` x `height` characters.
    Rectangle(u8, u8),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) struct DrawMode {
    pub(crate) shape: DrawModeShape,
    pub(crate) renderer: DrawModeRenderer,
    pub(crate) hollow_thickness: HollowThickness,
}

impl Default for DrawMode {
    fn default() -> Self {
        Self::new(DrawModeShape::Circle, DrawModeRenderer::HalfBlocks, HollowThickness::None)
    }
}

impl DrawMode {
    pub(crate) fn new(shape: DrawModeShape, renderer: DrawModeRenderer, hollow_thickness: HollowThickness) -> Self {
        Self {
            shape,
            renderer,
            hollow_thickness,
        }
    }

    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice], paint_map: &mut PaintMap, area: Option<Rect>) {
        paint_map.reset();
        let Some(area) = area else {
            return;
        };
        surface.set_relative_clip(area.left(), area.top(), area.right(), area.bottom());
        surface.set_origin(area.left(), area.top());
        self.paint_shape(surface, theme, slices, paint_map, area);
        surface.reset_origin();
        surface.reset_clip();
    }

    fn paint_shape(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice], paint_map: &mut PaintMap, area: Rect) {
        match self.shape {
            DrawModeShape::Circle => self.paint_circle(surface, theme, slices, paint_map, area),
            DrawModeShape::Square => self.paint_square(surface, theme, slices, paint_map, area),
            DrawModeShape::RoundSquare => self.paint_round_square(surface, theme, slices, paint_map, area),
            DrawModeShape::Rectangle => self.paint_rectangle(surface, theme, slices, paint_map, area),
            DrawModeShape::RoundRectangle => self.paint_round_rectangle(surface, theme, slices, paint_map, area),
            DrawModeShape::Ellipse => self.paint_ellipse(surface, theme, slices, paint_map, area),
        }
    }

    fn paint_circle(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice], paint_map: &mut PaintMap, area: Rect) {
        let shape = Ellipse::circle(area, self.hollow_thickness, self.renderer);
        shape.paint(surface, theme, slices, paint_map);
    }
    fn paint_square(&self, _surface: &mut Surface, _theme: &Theme, _slices: &[Slice], _paint_map: &mut PaintMap, _area: Rect) {
        todo!()
    }
    fn paint_round_square(&self, _surface: &mut Surface, _theme: &Theme, _slices: &[Slice], _paint_map: &mut PaintMap, _area: Rect) {
        todo!()
    }
    fn paint_rectangle(&self, _surface: &mut Surface, _theme: &Theme, _slices: &[Slice], _paint_map: &mut PaintMap, _area: Rect) {
        todo!()
    }
    fn paint_round_rectangle(&self, _surface: &mut Surface, _theme: &Theme, _slices: &[Slice], _paint_map: &mut PaintMap, _area: Rect) {
        todo!()
    }
    fn paint_ellipse(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice], paint_map: &mut PaintMap, area: Rect) {
        let shape = Ellipse::fill(area, self.hollow_thickness, self.renderer);
        shape.paint(surface, theme, slices, paint_map);
    }
}

pub(super) fn plot_area(size: Size) -> Option<Rect> {
    let p = PADDING as i32;
    let w = size.width.saturating_sub(2 * PADDING);
    let h = size.height.saturating_sub(2 * PADDING);
    if (w == 0) || (h == 0) {
        return None;
    }
    Some(Rect::new(p, p, p + w as i32 - 1, p + h as i32 - 1))
}
