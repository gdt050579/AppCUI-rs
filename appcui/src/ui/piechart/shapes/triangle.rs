use super::super::piechart::slice_at;
use super::super::{HollowThickness, Slice};
use super::{hole_size, rasterize, Rotation};
use crate::graphics::{PaletteTile, Size};
use std::f64::consts::TAU;

// corner radius of a rounded triangle, as a part of the radius of its inscribed circle
const ROUND_CORNERS: f64 = 0.5;

// An equilateral-looking triangle pointing up, with the slices going around its centroid
// (cx, cy) (physical units). A slice covers the area swept by a ray from the centroid, from
// 12 o'clock clockwise, so the area of every slice is proportional to its value, for any hole.
pub(crate) struct Triangle {
    cx: f64,
    cy: f64,
    outline: Outline,
    hole: Option<Outline>,
}

impl Triangle {
    pub(crate) fn fit(size: Size, hollow: HollowThickness, rounded: bool) -> Self {
        // physical units: a cell is 1 unit wide and 2 units tall
        let width = size.width as f64;
        let height = size.height as f64 * 2.0;
        let side = width.min(height * 2.0 / 3f64.sqrt());
        let h = side * 3f64.sqrt() / 2.0;
        // the triangle is centered in the area; its centroid is 2/3 of the way down
        let top = (height - h) / 2.0;
        Self::new(width / 2.0, top + h * 2.0 / 3.0, side / 2.0, h, hollow, rounded)
    }

    // a triangle with its centroid in (cx, cy), half of its base `half_base` and height `height`
    pub(crate) fn new(cx: f64, cy: f64, half_base: f64, height: f64, hollow: HollowThickness, rounded: bool) -> Self {
        let (hole_half_base, hole_half_height) = hole_size(hollow, half_base, height / 2.0);
        let hole_half_base = hole_half_base.clamp(0.0, half_base.max(0.0));
        let hole_height = (hole_half_height * 2.0).clamp(0.0, height.max(0.0));
        let hole = if (hole_half_base > 0.0) && (hole_height > 0.0) {
            Outline::new(hole_half_base, hole_height, rounded)
        } else {
            None
        };
        Self {
            cx,
            cy,
            outline: Outline::new(half_base, height, rounded).unwrap_or_default(),
            hole,
        }
    }

    pub(crate) fn fraction(&self, x: f64, y: f64) -> Option<f64> {
        // relative to the centroid, y grows upwards
        let p = (x - self.cx, self.cy - y);
        if !self.outline.contains(p) {
            return None;
        }
        if self.hole.as_ref().is_some_and(|hole| hole.contains(p)) || (self.total() <= 0.0) {
            return None;
        }
        Some(self.direction_fraction(p.0, -p.1))
    }

    fn total(&self) -> f64 {
        self.outline.total - self.hole.as_ref().map_or(0.0, |hole| hole.total)
    }

    // the fraction (from 12 o'clock, clockwise) of the direction (dx, dy) from the centroid (y grows downwards)
    pub(crate) fn direction_fraction(&self, dx: f64, dy: f64) -> f64 {
        let total = self.total();
        if total <= 0.0 {
            return 0.0;
        }
        let p = (dx, -dy);
        let hole_swept = self.hole.as_ref().map_or(0.0, |hole| hole.swept(p));
        let f = (self.outline.swept(p) - hole_swept) / total;
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

type Vector = (f64, f64);

#[inline(always)]
fn cross(a: Vector, b: Vector) -> f64 {
    a.0 * b.1 - a.1 * b.0
}

// A piece of the outline, going clockwise: a straight edge from `from` to `to`, or an arc of
// the circle centered in `center` (with the corner radius), from angle `from_angle` down to `to_angle`.
#[derive(Copy, Clone, Default)]
enum Piece {
    #[default]
    None,
    Edge { from: Vector, to: Vector },
    Arc { center: Vector, from_angle: f64, to_angle: f64 },
}

// The outline of a triangle (pointing up, centroid at the origin, y grows upwards) with its corners
// rounded with `radius`, as edges and arcs in clockwise order, with the area swept from the
// origin up to the start of every piece.
#[derive(Copy, Clone, Default)]
struct Outline {
    pieces: [Piece; 6],
    // area swept from the origin up to the start of every piece
    start: [f64; 6],
    total: f64,
    // area swept up to the point at 12 o'clock
    top: f64,
    // the centers of the corner circles (the corners themselves if the radius is 0)
    inner: [Vector; 3],
    radius: f64,
}

impl Outline {
    fn new(half_base: f64, height: f64, rounded: bool) -> Option<Self> {
        if (half_base <= 0.0) || (height <= 0.0) {
            return None;
        }
        // clockwise: apex, bottom-right, bottom-left
        let corners = [(0.0, height * 2.0 / 3.0), (half_base, -height / 3.0), (-half_base, -height / 3.0)];
        let side = (half_base * half_base + height * height).sqrt();
        let inradius = half_base * height / (half_base + side);
        let radius = if rounded { inradius * ROUND_CORNERS } else { 0.0 };
        let mut outline = Outline { radius, ..Default::default() };
        // where the outline leaves every corner (and enters the next one)
        let mut enter = [(0.0, 0.0); 3];
        let mut leave = [(0.0, 0.0); 3];
        for i in 0..3 {
            let v = corners[i];
            let prev = corners[(i + 2) % 3];
            let next = corners[(i + 1) % 3];
            let to_prev = unit((prev.0 - v.0, prev.1 - v.1));
            let to_next = unit((next.0 - v.0, next.1 - v.1));
            let half_angle = (to_prev.0 * to_next.0 + to_prev.1 * to_next.1).clamp(-1.0, 1.0).acos() / 2.0;
            let bisector = unit((to_prev.0 + to_next.0, to_prev.1 + to_next.1));
            let along = radius / half_angle.tan();
            let to_center = radius / half_angle.sin();
            enter[i] = (v.0 + to_prev.0 * along, v.1 + to_prev.1 * along);
            leave[i] = (v.0 + to_next.0 * along, v.1 + to_next.1 * along);
            outline.inner[i] = (v.0 + bisector.0 * to_center, v.1 + bisector.1 * to_center);
        }
        let mut count = 0;
        for i in 0..3 {
            if radius > 0.0 {
                let c = outline.inner[i];
                let from_angle = (enter[i].1 - c.1).atan2(enter[i].0 - c.0);
                let mut to_angle = (leave[i].1 - c.1).atan2(leave[i].0 - c.0);
                // clockwise: the angle goes down
                if to_angle > from_angle {
                    to_angle -= TAU;
                }
                outline.pieces[count] = Piece::Arc { center: c, from_angle, to_angle };
                count += 1;
            }
            outline.pieces[count] = Piece::Edge { from: leave[i], to: enter[(i + 1) % 3] };
            count += 1;
        }
        let mut area = 0.0;
        for i in 0..count {
            outline.start[i] = area;
            area += outline.piece_area(outline.pieces[i], None);
        }
        outline.total = area;
        outline.top = outline.cumulative((0.0, 1.0));
        Some(outline)
    }

    // area swept from the origin over `piece`, from its start up to `point` (or the whole piece)
    fn piece_area(&self, piece: Piece, point: Option<Vector>) -> f64 {
        match piece {
            Piece::None => 0.0,
            Piece::Edge { from, to } => 0.5 * cross(point.unwrap_or(to), from),
            Piece::Arc { center, from_angle, to_angle } => {
                let end = match point {
                    Some(p) => {
                        let mut t = (p.1 - center.1).atan2(p.0 - center.0);
                        while t > from_angle {
                            t -= TAU;
                        }
                        t.max(to_angle)
                    }
                    None => to_angle,
                };
                // integral of (P x P') / 2 along the circle P(t) = center + r (cos t, sin t), from `end` to `from_angle`
                let r = self.radius;
                let f = |t: f64| 0.5 * (r * (center.0 * t.sin() - center.1 * t.cos()) + r * r * t);
                f(from_angle) - f(end)
            }
        }
    }

    // where the ray from the origin in direction `u` leaves the outline: the piece and the point
    fn hit(&self, u: Vector) -> Option<(usize, Vector)> {
        for (i, piece) in self.pieces.iter().enumerate() {
            match *piece {
                Piece::None => break,
                Piece::Edge { from, to } => {
                    let d = (to.0 - from.0, to.1 - from.1);
                    let den = cross(u, d);
                    if den.abs() < 1e-12 {
                        continue;
                    }
                    let s = cross(from, d) / den;
                    let k = cross(from, u) / den;
                    if (s > 0.0) && (-1e-9..=1.0 + 1e-9).contains(&k) {
                        return Some((i, (u.0 * s, u.1 * s)));
                    }
                }
                Piece::Arc { center, from_angle, to_angle } => {
                    // the farthest point of the ray on the circle
                    let uu = u.0 * u.0 + u.1 * u.1;
                    let uc = u.0 * center.0 + u.1 * center.1;
                    let disc = uc * uc - uu * (center.0 * center.0 + center.1 * center.1 - self.radius * self.radius);
                    if disc < 0.0 {
                        continue;
                    }
                    let s = (uc + disc.sqrt()) / uu;
                    let p = (u.0 * s, u.1 * s);
                    let mut t = (p.1 - center.1).atan2(p.0 - center.0);
                    while t > from_angle + 1e-9 {
                        t -= TAU;
                    }
                    if (s > 0.0) && (t >= to_angle - 1e-9) {
                        return Some((i, p));
                    }
                }
            }
        }
        None
    }

    // area swept from the start of the outline up to where the ray in direction `u` leaves it
    fn cumulative(&self, u: Vector) -> f64 {
        match self.hit(u) {
            Some((i, p)) => self.start[i] + self.piece_area(self.pieces[i], Some(p)),
            None => 0.0,
        }
    }

    // area swept from 12 o'clock clockwise up to the direction of `p`
    fn swept(&self, p: Vector) -> f64 {
        if (p.0 == 0.0) && (p.1 == 0.0) {
            return 0.0;
        }
        (self.cumulative(p) - self.top).rem_euclid(self.total)
    }

    // `p` is inside the outline: within `radius` of the triangle that joins the corner circles' centers
    fn contains(&self, p: Vector) -> bool {
        if self.total <= 0.0 {
            return false;
        }
        let inside_inner = (0..3).all(|i| {
            let a = self.inner[i];
            let b = self.inner[(i + 1) % 3];
            // clockwise corners: the inside is on the right of every edge
            cross((b.0 - a.0, b.1 - a.1), (p.0 - a.0, p.1 - a.1)) <= 0.0
        });
        if inside_inner {
            return true;
        }
        let r2 = self.radius * self.radius;
        (0..3).any(|i| distance_to_segment2(p, self.inner[i], self.inner[(i + 1) % 3]) <= r2)
    }
}

fn unit(v: Vector) -> Vector {
    let len = (v.0 * v.0 + v.1 * v.1).sqrt();
    if len > 0.0 {
        (v.0 / len, v.1 / len)
    } else {
        (0.0, 0.0)
    }
}

// squared distance from `p` to the segment from `a` to `b`
fn distance_to_segment2(p: Vector, a: Vector, b: Vector) -> f64 {
    let d = (b.0 - a.0, b.1 - a.1);
    let len2 = d.0 * d.0 + d.1 * d.1;
    let k = if len2 > 0.0 { (((p.0 - a.0) * d.0 + (p.1 - a.1) * d.1) / len2).clamp(0.0, 1.0) } else { 0.0 };
    let q = (a.0 + d.0 * k - p.0, a.1 + d.1 * k - p.1);
    q.0 * q.0 + q.1 * q.1
}
