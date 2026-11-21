use super::super::piechart::slice_at;
use super::super::{HollowThickness, Slice};
use super::{hole_size, rasterize, Rotation};
use crate::graphics::{PaletteTile, Size};
use std::f64::consts::FRAC_PI_2;

// corner radius of a rounded rectangle, as a part of its smaller half size
const ROUND_CORNERS: f64 = 0.5;

// A rectangle centered in (cx, cy), with half width `rx`, half height `ry` and corners rounded
// with `radius` (physical units; a radius of 0 is a plain rectangle). A slice covers the area
// swept by a ray from the center, from 12 o'clock clockwise, so the area of every slice is
// proportional to its value, for any hole.
pub(crate) struct Rectangle {
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    radius: f64,
    hole_rx: f64,
    hole_ry: f64,
    hole_radius: f64,
}

impl Rectangle {
    pub(crate) fn square(size: Size, hollow: HollowThickness, rounded: bool) -> Self {
        // physical units: a cell is 1 unit wide and 2 units tall
        let width = size.width as f64;
        let height = size.height as f64 * 2.0;
        let half = width.min(height) / 2.0;
        Self::new(width / 2.0, height / 2.0, half, half, hollow, rounded)
    }

    pub(crate) fn fill(size: Size, hollow: HollowThickness, rounded: bool) -> Self {
        // physical units: a cell is 1 unit wide and 2 units tall
        let rx = size.width as f64 / 2.0;
        let ry = size.height as f64;
        Self::new(rx, ry, rx, ry, hollow, rounded)
    }

    pub(crate) fn new(cx: f64, cy: f64, rx: f64, ry: f64, hollow: HollowThickness, rounded: bool) -> Self {
        let (hole_rx, hole_ry) = hole_size(hollow, rx, ry);
        let hole_rx = hole_rx.clamp(0.0, rx.max(0.0));
        let hole_ry = hole_ry.clamp(0.0, ry.max(0.0));
        let round = if rounded { ROUND_CORNERS } else { 0.0 };
        Self {
            cx,
            cy,
            rx,
            ry,
            radius: rx.min(ry).max(0.0) * round,
            hole_rx,
            hole_ry,
            hole_radius: hole_rx.min(hole_ry) * round,
        }
    }

    pub(crate) fn fraction(&self, x: f64, y: f64) -> Option<f64> {
        let dx = x - self.cx;
        let dy = y - self.cy;
        if !inside(self.rx, self.ry, self.radius, dx, dy) || inside(self.hole_rx, self.hole_ry, self.hole_radius, dx, dy) {
            return None;
        }
        if self.total() <= 0.0 {
            return None;
        }
        Some(self.direction_fraction(dx, dy))
    }

    fn total(&self) -> f64 {
        4.0 * (quarter_area(self.rx, self.ry, self.radius) - quarter_area(self.hole_rx, self.hole_ry, self.hole_radius))
    }

    // the fraction (from 12 o'clock, clockwise) of the direction (dx, dy) from the center
    pub(crate) fn direction_fraction(&self, dx: f64, dy: f64) -> f64 {
        let total = self.total();
        if total <= 0.0 {
            return 0.0;
        }
        let swept = swept_area(self.rx, self.ry, self.radius, dx, dy) - swept_area(self.hole_rx, self.hole_ry, self.hole_radius, dx, dy);
        let f = swept / total;
        // rounding can push a point just left of 12 o'clock to exactly 1.0
        if f >= 1.0 {
            0.0
        } else {
            f.max(0.0)
        }
    }

    // fills the tile with slice indexes; a pixel is `scale` x `scale` physical units
    pub(crate) fn paint(&self, tile: &mut PaletteTile, scale: f64, shift: f64, rotation: Rotation, slices: &[Slice]) {
        let start = self.direction_fraction(rotation.start.0, rotation.start.1);
        rasterize(tile, scale, shift, |x, y| self.fraction(x, y).map(|f| slice_at(slices, rotation.apply(f, start))));
    }

    #[cfg(test)]
    pub(crate) fn slice_at(&self, x: f64, y: f64, slices: &[Slice]) -> Option<usize> {
        self.fraction(x, y).map(|f| slice_at(slices, f))
    }
}

// (dx, dy) relative to the center is inside the rectangle with half sizes (rx, ry) and corners
// rounded with `radius`; half open: the left and top edges are inside, the right and bottom ones are not
fn inside(rx: f64, ry: f64, radius: f64, dx: f64, dy: f64) -> bool {
    if !(-rx..rx).contains(&dx) || !(-ry..ry).contains(&dy) {
        return false;
    }
    // distance from the center of the corner's circle
    let qx = dx.abs() - (rx - radius);
    let qy = dy.abs() - (ry - radius);
    (qx <= 0.0) || (qy <= 0.0) || (qx * qx + qy * qy <= radius * radius)
}

// area of a quarter of the rectangle with half sizes (a, b) and corners rounded with `radius`
fn quarter_area(a: f64, b: f64, radius: f64) -> f64 {
    if (a <= 0.0) || (b <= 0.0) {
        return 0.0;
    }
    a * b - (1.0 - FRAC_PI_2 / 2.0) * radius * radius
}

// Area swept by a ray from the center inside the top-right quarter of the rectangle with half
// sizes (a, b) and corners rounded with `radius`, from 12 o'clock clockwise up to the
// direction (ux, uy) (ux >= 0 to the right, uy >= 0 upwards). The boundary of the quarter is the
// top edge, the arc of the corner and the right edge; the swept area grows linearly along the
// edges (triangles with the center) and has a closed form along the arc.
fn quarter_swept_area(a: f64, b: f64, radius: f64, ux: f64, uy: f64) -> f64 {
    // the corner's circle is centered in (cx, cy)
    let (cx, cy) = (a - radius, b - radius);
    // area swept along the arc, from its top end (angle PI / 2) down to the angle t
    let arc = |t: f64| 0.5 * (radius * (cx * (FRAC_PI_2.sin() - t.sin()) - cy * (FRAC_PI_2.cos() - t.cos())) + radius * radius * (FRAC_PI_2 - t));
    let top = b * cx / 2.0;
    // the ray leaves through the top edge
    if (uy > 0.0) && (ux * b <= cx * uy) {
        return b * (ux * b / uy) / 2.0;
    }
    // the ray leaves through the right edge
    if (ux > 0.0) && (uy * a <= cy * ux) {
        return top + arc(0.0) + a * (cy - uy * a / ux) / 2.0;
    }
    // the ray leaves through the arc: the farthest point of the ray on the corner's circle
    let uu = ux * ux + uy * uy;
    let uc = ux * cx + uy * cy;
    let d = (uc * uc - uu * (cx * cx + cy * cy - radius * radius)).max(0.0);
    let s = (uc + d.sqrt()) / uu;
    let t = (s * uy - cy).atan2(s * ux - cx).clamp(0.0, FRAC_PI_2);
    top + arc(t)
}

// Area of the centered rectangle with half sizes (rx, ry) and corners rounded with `radius`,
// swept by a ray from the center, from 12 o'clock clockwise up to the direction (dx, dy)
// (y grows downwards). The other quarters are mirror images of the top-right one.
fn swept_area(rx: f64, ry: f64, radius: f64, dx: f64, dy: f64) -> f64 {
    if (rx <= 0.0) || (ry <= 0.0) || ((dx == 0.0) && (dy == 0.0)) {
        return 0.0;
    }
    let q = quarter_area(rx, ry, radius);
    let quarter = |ux: f64, uy: f64| quarter_swept_area(rx, ry, radius, ux, uy);
    match (dx >= 0.0, dy <= 0.0) {
        // top-right: from 12 o'clock
        (true, true) => quarter(dx, -dy),
        // bottom-right: from 3 o'clock (mirror image of the top-right quarter)
        (true, false) => 2.0 * q - quarter(dx, dy),
        // bottom-left: from 6 o'clock (the top-right quarter, rotated)
        (false, false) => 2.0 * q + quarter(-dx, dy),
        // top-left: from 9 o'clock (mirror image of the top-right quarter)
        (false, true) => 4.0 * q - quarter(-dx, -dy),
    }
}
