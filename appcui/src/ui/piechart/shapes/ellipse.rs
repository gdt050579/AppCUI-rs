use super::super::draw_mode::PADDING;
use super::super::piechart::slice_at;
use super::super::{DrawModeRenderer, HollowThickness, Slice, PaintMap};
use crate::graphics::{CharFlags, Character, Color, Rect, Size, SpecialChar, Surface};
use crate::system::Theme;
use std::f64::consts::TAU;

pub(crate) struct Ellipse {
    size: Size,
    origin_x: usize,
    origin_y: usize,
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    hole_rx: f64,
    hole_ry: f64,
    renderer: DrawModeRenderer,
}

impl Ellipse {
    pub(crate) fn circle(area: Rect, hollow: HollowThickness, renderer: DrawModeRenderer) -> Self {
        let size = area.size();
        // physical units: a cell is 1 unit wide and 2 units tall
        let width = size.width as f64;
        let height = size.height as f64 * 2.0;
        let radius = width.min(height) / 2.0;
        Self::new(size, width / 2.0, height / 2.0, radius, radius, hollow, renderer).at(area)
    }

    pub(crate) fn fill(area: Rect, hollow: HollowThickness, renderer: DrawModeRenderer) -> Self {
        let size = area.size();
        // physical units: a cell is 1 unit wide and 2 units tall
        let rx = size.width as f64 / 2.0;
        let ry = size.height as f64;
        Self::new(size, rx, ry, rx, ry, hollow, renderer).at(area)
    }

    fn at(mut self, area: Rect) -> Self {
        self.origin_x = area.left().max(0) as usize;
        self.origin_y = area.top().max(0) as usize;
        self
    }

    pub(crate) fn new(size: Size, cx: f64, cy: f64, rx: f64, ry: f64, hollow: HollowThickness, renderer: DrawModeRenderer) -> Self {
        let (hole_rx, hole_ry) = Self::hole_radii(hollow, rx, ry);
        Self {
            size,
            origin_x: PADDING as usize,
            origin_y: PADDING as usize,
            cx,
            cy,
            rx,
            ry,
            hole_rx: hole_rx.clamp(0.0, rx.max(0.0)),
            hole_ry: hole_ry.clamp(0.0, ry.max(0.0)),
            renderer,
        }
    }

    pub(crate) fn hole_radii(hollow: HollowThickness, rx: f64, ry: f64) -> (f64, f64) {
        match hollow {
            HollowThickness::None => (0.0, 0.0),
            HollowThickness::Percentage(p) => {
                let f = p.min(100) as f64 / 100.0;
                (rx * f, ry * f)
            }
            HollowThickness::Absolute(chars) => {
                // `chars` wide, same proportions as the ellipse
                let hole_rx = chars.min(100) as f64 / 2.0;
                let hole_ry = if rx > 0.0 { hole_rx * ry / rx } else { 0.0 };
                (hole_rx, hole_ry)
            }
            HollowThickness::Rectangle(w, h) => {
                // a character is 1 unit wide and 2 units tall
                (w as f64 / 2.0, h as f64)
            }
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
        // atan2(nx, -ny) is 0 at 12 o'clock and grows clockwise (y grows downwards)
        let f = (nx.atan2(-ny) / TAU).rem_euclid(1.0);
        // rem_euclid can round up to exactly 1.0 for tiny negative angles
        Some(if f >= 1.0 { 0.0 } else { f })
    }

    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme, slices: &[Slice], paint_map: &mut PaintMap) {
        if slices.iter().all(|s| s.start == s.end) {
            return;
        }
        let background = theme.chart.background.background;
        match self.renderer {
            DrawModeRenderer::HalfBlocks => self.paint_half_blocks(surface, slices, background, paint_map),
            DrawModeRenderer::Braille => self.paint_braille(surface, slices, background, paint_map),
            DrawModeRenderer::Custom(ch) => self.paint_custom(surface, slices, ch, background, paint_map),
        }
    }

    pub(crate) fn slice_at(&self, x: f64, y: f64, slices: &[Slice]) -> Option<usize> {
        self.fraction(x, y).map(|f| slice_at(slices, f))
    }

    // two pixels per cell (top and bottom), each pixel 1 x 1 physical units; in the
    // paint map, a cell belongs to the slice of its top pixel (or bottom, if the top is empty)
    fn paint_half_blocks(&self, surface: &mut Surface, slices: &[Slice], background: Color, paint_map: &mut PaintMap) {
        let color = |id: Option<usize>| id.map_or(background, |i| slices[i].color);
        for cell_y in 0..self.size.height {
            let top_y = cell_y as f64 * 2.0 + 0.5;
            for cell_x in 0..self.size.width {
                let x = cell_x as f64 + 0.5;
                let top = self.slice_at(x, top_y, slices);
                let bottom = self.slice_at(x, top_y + 1.0, slices);
                if top.is_none() && bottom.is_none() {
                    continue;
                }
                let ch = Character::new(SpecialChar::BlockUpperHalf, color(top), color(bottom), CharFlags::None);
                surface.write_char(cell_x as i32, cell_y as i32, ch);
                if let Some(id) = top.or(bottom) {
                    paint_map.set(cell_x as usize + self.origin_x, cell_y as usize + self.origin_y, id as u16);
                }
            }
        }
    }

    // 2 x 4 dots per cell, each dot 0.5 x 0.5 physical units; a cell takes the color
    // of the slice that covers most of its dots
    fn paint_braille(&self, surface: &mut Surface, slices: &[Slice], background: Color, paint_map: &mut PaintMap) {
        // bit of each dot in a braille character, by (column, row)
        const DOTS: [[u8; 4]; 2] = [[0x01, 0x02, 0x04, 0x40], [0x08, 0x10, 0x20, 0x80]];
        for cell_y in 0..self.size.height {
            for cell_x in 0..self.size.width {
                let mut pattern = 0u8;
                let mut ids = [0usize; 8];
                let mut counts = [0u8; 8];
                let mut distinct = 0usize;
                for (col, bits) in DOTS.iter().enumerate() {
                    let x = cell_x as f64 + (col as f64 + 0.5) * 0.5;
                    for (row, bit) in bits.iter().enumerate() {
                        let y = cell_y as f64 * 2.0 + (row as f64 + 0.5) * 0.5;
                        if let Some(id) = self.slice_at(x, y, slices) {
                            pattern |= bit;
                            match ids[..distinct].iter().position(|i| *i == id) {
                                Some(p) => counts[p] += 1,
                                None => {
                                    ids[distinct] = id;
                                    counts[distinct] = 1;
                                    distinct += 1;
                                }
                            }
                        }
                    }
                }
                if pattern == 0 {
                    continue;
                }
                // most dots wins; on a tie, the first slice found
                let mut best = 0;
                for p in 1..distinct {
                    if counts[p] > counts[best] {
                        best = p;
                    }
                }
                let code = char::from_u32(0x2800 + pattern as u32).unwrap_or(' ');
                let ch = Character::new(code, slices[ids[best]].color, background, CharFlags::None);
                surface.write_char(cell_x as i32, cell_y as i32, ch);
                paint_map.set(cell_x as usize + self.origin_x, cell_y as usize + self.origin_y, ids[best] as u16);
            }
        }
    }

    // one sample per cell, at its center
    fn paint_custom(&self, surface: &mut Surface, slices: &[Slice], code: char, background: Color, paint_map: &mut PaintMap) {
        for cell_y in 0..self.size.height {
            let y = cell_y as f64 * 2.0 + 1.0;
            for cell_x in 0..self.size.width {
                if let Some(id) = self.slice_at(cell_x as f64 + 0.5, y, slices) {
                    let ch = Character::new(code, slices[id].color, background, CharFlags::None);
                    surface.write_char(cell_x as i32, cell_y as i32, ch);
                    paint_map.set(cell_x as usize + self.origin_x, cell_y as usize + self.origin_y, id as u16);
                }
            }
        }
    }
}
