use super::super::piechart::slice_at;
use super::super::{HollowThickness, Slice};
use super::{hole_size, rasterize, Rotation};
use crate::graphics::{PaletteTile, Size};
use std::f64::consts::TAU;

pub(crate) struct Ellipse {
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    hole_rx: f64,
    hole_ry: f64,
}

impl Ellipse {
    pub(crate) fn circle(size: Size, hollow: HollowThickness) -> Self {
        // physical units: a cell is 1 unit wide and 2 units tall
        let width = size.width as f64;
        let height = size.height as f64 * 2.0;
        let radius = width.min(height) / 2.0;
        Self::new(width / 2.0, height / 2.0, radius, radius, hollow)
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
        if (self.rx <= 0.0) || (self.ry <= 0.0) {
            return None;
        }
        let dx = x - self.cx;
        let dy = y - self.cy;
        // normalized coordinates: the outer ellipse is the unit circle
        let nx = dx / self.rx;
        let ny = dy / self.ry;
        if nx * nx + ny * ny > 1.0 {
            return None;
        }
        if (self.hole_rx > 0.0) && (self.hole_ry > 0.0) {
            let hx = dx / self.hole_rx;
            let hy = dy / self.hole_ry;
            if hx * hx + hy * hy < 1.0 {
                return None;
            }
        }
        Some(self.direction_fraction(dx, dy))
    }

    // the fraction (from 12 o'clock, clockwise) of the direction (dx, dy) from the center
    pub(crate) fn direction_fraction(&self, dx: f64, dy: f64) -> f64 {
        if (self.rx <= 0.0) || (self.ry <= 0.0) {
            return 0.0;
        }
        // normalized coordinates: atan2(nx, -ny) is 0 at 12 o'clock and grows clockwise (y grows downwards)
        let f = ((dx / self.rx).atan2(-dy / self.ry) / TAU).rem_euclid(1.0);
        // rem_euclid can round up to exactly 1.0 for tiny negative angles
        if f >= 1.0 {
            0.0
        } else {
            f
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
