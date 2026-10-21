mod ellipse;
mod rectangle;
mod triangle;

pub(super) use self::ellipse::Ellipse;
pub(super) use self::rectangle::Rectangle;
pub(super) use self::triangle::Triangle;

use super::{Direction, HollowThickness};
use crate::graphics::PaletteTile;

// half width and half height of the hole, in physical units, for a shape with
// half width `rx` and half height `ry`
pub(crate) fn hole_size(hollow: HollowThickness, rx: f64, ry: f64) -> (f64, f64) {
    match hollow {
        HollowThickness::None => (0.0, 0.0),
        HollowThickness::Percentage(p) => {
            let f = p.min(100) as f64 / 100.0;
            (rx * f, ry * f)
        }
        HollowThickness::Absolute(chars) => {
            // `chars` wide, same proportions as the shape
            let hole_rx = chars.min(100) as f64 / 2.0;
            let hole_ry = if rx > 0.0 { hole_rx * ry / rx } else { 0.0 };
            (hole_rx, hole_ry)
        }
    }
}

// where the slices start and the direction they follow each other in
#[derive(Copy, Clone)]
pub(crate) struct Rotation {
    // the direction of the start ray, in physical units (y grows downwards)
    start: (f64, f64),
    clockwise: bool,
}

impl Rotation {
    pub(crate) fn new(degrees: u32, direction: Direction) -> Self {
        let angle = (degrees.min(360) as f64).to_radians();
        Self {
            start: (angle.sin(), -angle.cos()),
            clockwise: direction == Direction::Clockwise,
        }
    }

    // a fraction measured from 12 o'clock clockwise, measured instead from `start` (a fraction too)
    // in the direction of the rotation
    pub(crate) fn apply(&self, fraction: f64, start: f64) -> f64 {
        let f = (fraction - start).rem_euclid(1.0);
        let f = if self.clockwise { f } else { (1.0 - f).rem_euclid(1.0) };
        if f >= 1.0 {
            0.0
        } else {
            f
        }
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self::new(0, Direction::Clockwise)
    }
}

// fills every pixel of the tile with the slice at its center; a pixel is `scale` x `scale`
// physical units (a character is 1 unit wide and 2 units tall) and the tile starts `shift`
// units to the left of the shape's area
fn rasterize<F>(tile: &mut PaletteTile, scale: f64, shift: f64, slice_at: F)
where
    F: Fn(f64, f64) -> Option<usize>,
{
    let size = tile.size();
    for y in 0..size.height {
        let py = (y as f64 + 0.5) * scale;
        for x in 0..size.width {
            if let Some(id) = slice_at((x as f64 + 0.5) * scale - shift, py) {
                tile.set(x, y, id as u16);
            }
        }
    }
}
