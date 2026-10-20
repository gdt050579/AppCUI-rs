use super::shapes::{Ellipse, Rectangle, Triangle};
use super::Slice;
use crate::graphics::{CharAttribute, CharFlags, PaletteTile, PaletteTileRenderMethod, Rect, Size, Surface};
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
    /// The largest triangle that fits the plot area, pointing up, with three equal sides,
    /// centered. The slices go around its centroid (one third of the way up from the base).
    Triangle,
    /// Like [`Triangle`](Self::Triangle), with rounded corners.
    RoundTriangle,
}

/// How the pixels of a [`PieChart`](super::PieChart) are turned into characters.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum DrawModeRenderer {
    /// Half blocks (`▀`): two pixels per character, each with its own color. This is the default.
    #[default]
    HalfBlocks,
    /// U+2588 — full block.
    Solid,
    /// U+2593 ▓ — dense shade.
    Shade75,
    /// U+2592 ▒ — medium shade.
    Shade50,
    /// U+2591 ░ — light shade.
    Shade25,
    /// 2x4 checkerboard pattern.
    Checkerboard,
    /// U+253C ┼ — light grid/lattice.
    Grid,
    /// U+256C ╬ — double-line grid, heavier than [`Grid`](Self::Grid).
    GridDouble,
    /// U+2573 ╳ — diagonal cross-hatch.
    CrossHatch,
    /// ┇ — dashed line.
    Dashed,
    /// U+2571 ╱ — forward diagonal (rising left-to-right).
    DiagonalUp,
    /// U+2572 ╲ — backward diagonal (falling left-to-right).
    DiagonalDown,
    /// Braille dots: 2x4 pixels per character. A character takes the color of the slice that covers most of its dots.
    Braille,
    /// The given character in the slice color: every pixel is 2 characters wide.
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

    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice], pixels: &mut PaletteTile, cells: &mut PaletteTile, area: Option<Rect>) {
        cells.reset();
        let Some(area) = area else {
            return;
        };
        if slices.iter().all(|s| s.start == s.end) {
            return;
        }
        let method = self.renderer.render_method();
        // pixels needed to cover the area, and the size of a pixel in physical units
        // (a character is 1 unit wide and 2 units tall)
        let size = area.size();
        // with pixels 2 characters wide, the tile starts one character to the left if that puts
        // the left edge of the shape on the edge between two pixels (so the shape stays centered)
        let (width, height, scale, shift) = match method {
            PaletteTileRenderMethod::SmallBlocks => (size.width, size.height * 2, 1.0, 0),
            PaletteTileRenderMethod::Braille => (size.width * 2, size.height * 4, 0.5, 0),
            PaletteTileRenderMethod::LargeBlocks | PaletteTileRenderMethod::Custom(_) => {
                let shift = (self.left_edge(size).round() as u32) % 2;
                ((size.width + shift).div_ceil(2), size.height, 2.0, shift)
            }
        };
        pixels.resize(width, height);
        self.paint_shape(pixels, scale, shift as f64, slices, size);
        let background = theme.chart.background.background;
        surface.set_relative_clip(area.left(), area.top(), area.right(), area.bottom());
        surface.draw_palette_tile(area.left() - shift as i32, area.top(), pixels, method, |id| {
            CharAttribute::new(slices[id as usize].color(id as usize, theme), background, CharFlags::None)
        });
        surface.reset_clip();
        for y in 0..size.height {
            for x in 0..size.width {
                if let Some(id) = pixels.cell_index(x + shift, y, method) {
                    cells.set(area.left() as u32 + x, area.top() as u32 + y, id);
                }
            }
        }
    }

    // left edge of the shape, in characters from the left of the area
    fn left_edge(&self, size: Size) -> f64 {
        let width = size.width as f64;
        match self.shape {
            DrawModeShape::Circle | DrawModeShape::Square | DrawModeShape::RoundSquare => (width - width.min(size.height as f64 * 2.0)) / 2.0,
            DrawModeShape::Rectangle | DrawModeShape::RoundRectangle | DrawModeShape::Ellipse => 0.0,
            DrawModeShape::Triangle | DrawModeShape::RoundTriangle => (width - width.min(size.height as f64 * 4.0 / 3f64.sqrt())) / 2.0,
        }
    }

    fn paint_shape(&self, pixels: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice], size: Size) {
        match self.shape {
            DrawModeShape::Circle => self.paint_circle(pixels, scale, shift, slices, size),
            DrawModeShape::Square => self.paint_square(pixels, scale, shift, slices, size),
            DrawModeShape::RoundSquare => self.paint_round_square(pixels, scale, shift, slices, size),
            DrawModeShape::Rectangle => self.paint_rectangle(pixels, scale, shift, slices, size),
            DrawModeShape::RoundRectangle => self.paint_round_rectangle(pixels, scale, shift, slices, size),
            DrawModeShape::Ellipse => self.paint_ellipse(pixels, scale, shift, slices, size),
            DrawModeShape::Triangle => self.paint_triangle(pixels, scale, shift, slices, size),
            DrawModeShape::RoundTriangle => self.paint_round_triangle(pixels, scale, shift, slices, size),
        }
    }

    fn paint_circle(&self, pixels: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice], size: Size) {
        Ellipse::circle(size, self.hollow_thickness).paint(pixels, scale, shift, slices);
    }
    fn paint_square(&self, pixels: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice], size: Size) {
        Rectangle::square(size, self.hollow_thickness, false).paint(pixels, scale, shift, slices);
    }
    fn paint_round_square(&self, pixels: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice], size: Size) {
        Rectangle::square(size, self.hollow_thickness, true).paint(pixels, scale, shift, slices);
    }
    fn paint_rectangle(&self, pixels: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice], size: Size) {
        Rectangle::fill(size, self.hollow_thickness, false).paint(pixels, scale, shift, slices);
    }
    fn paint_round_rectangle(&self, pixels: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice], size: Size) {
        Rectangle::fill(size, self.hollow_thickness, true).paint(pixels, scale, shift, slices);
    }
    fn paint_ellipse(&self, pixels: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice], size: Size) {
        Ellipse::fill(size, self.hollow_thickness).paint(pixels, scale, shift, slices);
    }
    fn paint_triangle(&self, pixels: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice], size: Size) {
        Triangle::fit(size, self.hollow_thickness, false).paint(pixels, scale, shift, slices);
    }
    fn paint_round_triangle(&self, pixels: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice], size: Size) {
        Triangle::fit(size, self.hollow_thickness, true).paint(pixels, scale, shift, slices);
    }
}

impl DrawModeRenderer {
    pub(crate) fn render_method(self) -> PaletteTileRenderMethod {
        match self {
            DrawModeRenderer::HalfBlocks => PaletteTileRenderMethod::SmallBlocks,
            DrawModeRenderer::Solid => PaletteTileRenderMethod::LargeBlocks,
            DrawModeRenderer::Shade75 => PaletteTileRenderMethod::Custom('\u{2593}'),
            DrawModeRenderer::Shade50 => PaletteTileRenderMethod::Custom('\u{2592}'),
            DrawModeRenderer::Shade25 => PaletteTileRenderMethod::Custom('\u{2591}'),
            DrawModeRenderer::Checkerboard => PaletteTileRenderMethod::Custom('\u{2592}'),
            DrawModeRenderer::Grid => PaletteTileRenderMethod::Custom('\u{253C}'),
            DrawModeRenderer::GridDouble => PaletteTileRenderMethod::Custom('\u{256C}'),
            DrawModeRenderer::CrossHatch => PaletteTileRenderMethod::Custom('\u{2573}'),
            DrawModeRenderer::Dashed => PaletteTileRenderMethod::Custom('\u{2571}'),
            DrawModeRenderer::DiagonalUp => PaletteTileRenderMethod::Custom('\u{2571}'),
            DrawModeRenderer::DiagonalDown => PaletteTileRenderMethod::Custom('\u{2572}'),
            DrawModeRenderer::Braille => PaletteTileRenderMethod::Braille,
            DrawModeRenderer::Custom(ch) => PaletteTileRenderMethod::Custom(ch),
        }
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
