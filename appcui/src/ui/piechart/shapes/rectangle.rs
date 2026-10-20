use super::super::piechart::slice_at;
use super::super::{HollowThickness, Slice};
use super::{hole_size, rasterize};
use crate::graphics::{PaletteTile, Size};

// A rectangle centered in (cx, cy), with half width `rx` and half height `ry` (physical units).
// A slice covers the area swept by a ray from the center, from 12 o'clock clockwise, so the
// area of every slice is proportional to its value, for any hole.
pub(crate) struct Rectangle {
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    hole_rx: f64,
    hole_ry: f64,
}

impl Rectangle {
    pub(crate) fn square(size: Size, hollow: HollowThickness) -> Self {
        // physical units: a cell is 1 unit wide and 2 units tall
        let width = size.width as f64;
        let height = size.height as f64 * 2.0;
        let half = width.min(height) / 2.0;
        Self::new(width / 2.0, height / 2.0, half, half, hollow)
    }

    pub(crate) fn fill(size: Size, hollow: HollowThickness) -> Self {
        // physical units: a cell is 1 unit wide and 2 units tall
        let rx = size.width as f64 / 2.0;
        let ry = size.height as f64;
        Self::new(rx, ry, rx, ry, hollow)
    }

    pub(crate) fn new(cx: f64, cy: f64, rx: f64, ry: f64, hollow: HollowThickness) -> Self {
        let (hole_rx, hole_ry) = hole_size(hollow, rx, ry);
        Self {
            cx,
            cy,
            rx,
            ry,
            hole_rx: hole_rx.clamp(0.0, rx.max(0.0)),
            hole_ry: hole_ry.clamp(0.0, ry.max(0.0)),
        }
    }

    pub(crate) fn fraction(&self, x: f64, y: f64) -> Option<f64> {
        let dx = x - self.cx;
        let dy = y - self.cy;
        // half open: the left and top edges are inside, the right and bottom ones are not
        if !(-self.rx..self.rx).contains(&dx) || !(-self.ry..self.ry).contains(&dy) {
            return None;
        }
        if (-self.hole_rx..self.hole_rx).contains(&dx) && (-self.hole_ry..self.hole_ry).contains(&dy) {
            return None;
        }
        let total = 4.0 * (self.rx * self.ry - self.hole_rx * self.hole_ry);
        if total <= 0.0 {
            return None;
        }
        let swept = swept_area(self.rx, self.ry, dx, dy) - swept_area(self.hole_rx, self.hole_ry, dx, dy);
        let f = swept / total;
        // rounding can push a point just left of 12 o'clock to exactly 1.0
        Some(if f >= 1.0 { 0.0 } else { f.max(0.0) })
    }

    // fills the tile with slice indexes; a pixel is `scale` x `scale` physical units
    pub(crate) fn paint(&self, tile: &mut PaletteTile, scale: f64, shift: f64, slices: &[Slice]) {
        rasterize(tile, scale, shift, |x, y| self.slice_at(x, y, slices));
    }

    pub(crate) fn slice_at(&self, x: f64, y: f64, slices: &[Slice]) -> Option<usize> {
        self.fraction(x, y).map(|f| slice_at(slices, f))
    }
}

// Area of the centered rectangle with half sizes (rx, ry) swept by a ray from the
// center, from 12 o'clock clockwise up to the direction (dx, dy) (y grows downwards).
// The rectangle is split into triangles from the center to each edge, so the area
// grows linearly along every edge: rx * ry / 2 for every half edge, 4 * rx * ry in total.
fn swept_area(rx: f64, ry: f64, dx: f64, dy: f64) -> f64 {
    if (rx <= 0.0) || (ry <= 0.0) || ((dx == 0.0) && (dy == 0.0)) {
        return 0.0;
    }
    let half = rx * ry / 2.0;
    if dx.abs() * ry >= dy.abs() * rx {
        // the ray leaves through the left or the right edge, at height y
        let y = dy * rx / dx.abs();
        if dx > 0.0 {
            half + rx * (y + ry) / 2.0
        } else {
            5.0 * half + rx * (ry - y) / 2.0
        }
    } else {
        // the ray leaves through the top or the bottom edge, at x
        let x = dx * ry / dy.abs();
        if dy > 0.0 {
            3.0 * half + ry * (rx - x) / 2.0
        } else if x >= 0.0 {
            ry * x / 2.0
        } else {
            7.0 * half + ry * (x + rx) / 2.0
        }
    }
}
