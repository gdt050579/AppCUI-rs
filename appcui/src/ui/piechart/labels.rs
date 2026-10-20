use super::draw_mode::{plot_area, DrawMode};
use super::piechart::FULL;
use super::Slice;
use crate::graphics::{CharAttribute, Character, PaletteTile, Point, Rect, Size, Surface};
use crate::system::Theme;
use crate::utils::FormatNumber;

/// Where the labels of a [`PieChart`](super::PieChart) are drawn.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum LabelLayout {
    /// No labels. This is the default.
    #[default]
    None,
    /// Each label is centered on its slice, where the tooltip of the slice is shown.
    Inside,
    /// Labels are drawn around the chart (above, below, left or right of their slice)
    /// and linked to it with a short line. The chart shrinks to make room for them.
    Outside,
}

/// What the label of a slice shows.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum LabelContent {
    /// The value of the slice.
    Value,
    /// The label of the slice. This is the default.
    #[default]
    Label,
    /// The label and the value of the slice, as `label: value` (just the value if the label is empty).
    LabelAndValue,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(super) enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Copy, Clone, Debug)]
pub(super) struct Place {
    pub(super) index: usize,
    pub(super) pos: i32,
    pub(super) len: i32,
    pub(super) anchor: Point,
}

pub(super) fn update_texts(slices: &mut [Slice], content: LabelContent, format: &FormatNumber) {
    let mut buffer = [0u8; 96];
    for s in slices.iter_mut() {
        let Slice { text, tooltip, label, value, .. } = s;
        text.clear();
        tooltip.clear();
        // exact for every u64; `decimals(n)` adds n zero decimals
        let formatted = format.write_fraction(*value, 1u64, &mut buffer).unwrap_or("");
        if !label.is_empty() {
            tooltip.push_str(label);
            tooltip.push_str(": ");
        }
        tooltip.push_str(formatted);
        match content {
            LabelContent::Label => text.push_str(label),
            LabelContent::Value => text.push_str(formatted),
            LabelContent::LabelAndValue => text.push_str(tooltip),
        }
    }
}

fn has_label(s: &Slice) -> bool {
    (s.start != s.end) && !s.text.is_empty()
}

pub(super) fn side_of(s: &Slice, mode: &DrawMode) -> Side {
    let middle = (s.start / 2 + s.end / 2) as f64 / FULL as f64;
    let degrees = mode.angle_of(middle);
    if !(45.0..315.0).contains(&degrees) {
        Side::Top
    } else if degrees < 135.0 {
        Side::Right
    } else if degrees < 225.0 {
        Side::Bottom
    } else {
        Side::Left
    }
}

fn text_len(s: &Slice) -> u32 {
    s.text.chars().count() as u32
}

fn side_width(inner: Rect) -> u32 {
    (inner.width() / 4).max(1)
}

// the plot area for outside labels and whether there is room for them
pub(super) fn outside_area(size: Size, slices: &[Slice], mode: &DrawMode) -> (Option<Rect>, bool) {
    let Some(inner) = plot_area(size) else {
        return (None, false);
    };
    let cap = side_width(inner);
    let (mut left, mut right, mut top, mut bottom) = (0, 0, false, false);
    for s in slices.iter().filter(|s| has_label(s)) {
        match side_of(s, mode) {
            Side::Left => left = left.max(text_len(s).min(cap)),
            Side::Right => right = right.max(text_len(s).min(cap)),
            Side::Top => top = true,
            Side::Bottom => bottom = true,
        }
    }
    // the same room on opposite sides, so that the chart stays centered
    let side = left.max(right);
    let horizontal = if side > 0 { side as i32 + 2 } else { 0 };
    let vertical = if top || bottom { 2 } else { 0 };
    let l = inner.left() + horizontal;
    let r = inner.right() - horizontal;
    let t = inner.top() + vertical;
    let b = inner.bottom() - vertical;
    if (r - l + 1 < 3) || (b - t + 1 < 2) {
        return (Some(inner), false);
    }
    (Some(Rect::new(l, t, r, b)), true)
}

// first cell outside the chart, going from the center of the slice away from the center of the plot area
fn anchor(s: &Slice, side: Side, area: Rect, map: &PaletteTile) -> Point {
    let center = s.center.unwrap_or(Point::new(area.center_x(), area.center_y()));
    let sx = center.x as f64 + 0.5;
    let sy = center.y as f64 + 0.5;
    let mut dx = sx - (area.left() as f64 + area.width() as f64 / 2.0);
    let mut dy = (sy - (area.top() as f64 + area.height() as f64 / 2.0)) * 2.0;
    if dx * dx + dy * dy < 0.25 {
        (dx, dy) = match side {
            Side::Top => (0.0, -1.0),
            Side::Right => (1.0, 0.0),
            Side::Bottom => (0.0, 1.0),
            Side::Left => (-1.0, 0.0),
        };
    }
    let len = (dx * dx + dy * dy).sqrt();
    let (dx, dy) = (dx / len, dy / len);
    let size = map.size();
    let mut last = center;
    let mut t = 0.0;
    loop {
        t += 0.25;
        let x = (sx + dx * t).floor() as i32;
        let y = (sy + dy * t / 2.0).floor() as i32;
        if (x < 0) || (y < 0) || (x >= size.width as i32) || (y >= size.height as i32) {
            return last;
        }
        last = Point::new(x, y);
        if map.get(x as u32, y as u32).is_none() {
            return last;
        }
    }
}

// places items along [lo, hi] (keeping `gap` cells between them) as close as possible to
// their desired position; the smallest slices are dropped if they do not all fit
pub(super) fn fit(items: &mut Vec<Place>, slices: &[Slice], lo: i32, hi: i32, gap: i32) {
    while !items.is_empty() {
        let total: i32 = items.iter().map(|p| p.len).sum::<i32>() + gap * (items.len() as i32 - 1);
        if total <= hi - lo + 1 {
            break;
        }
        let smallest = (0..items.len()).min_by_key(|&i| (slices[items[i].index].value, std::cmp::Reverse(items[i].index))).unwrap();
        items.remove(smallest);
    }
    items.sort_by_key(|p| (p.pos, p.index));
    let mut next = lo;
    for p in items.iter_mut() {
        p.pos = p.pos.max(next);
        next = p.pos + p.len + gap;
    }
    let mut limit = hi;
    for p in items.iter_mut().rev() {
        p.pos = p.pos.min(limit - p.len + 1);
        limit = p.pos - gap - 1;
    }
}

fn write_text(surface: &mut Surface, x: i32, y: i32, text: &str, max: usize, attr: CharAttribute) {
    let count = text.chars().count();
    for (i, ch) in text.chars().take(max).enumerate() {
        let ch = if (count > max) && (i + 1 == max) { '…' } else { ch };
        surface.write_char(x + i as i32, y, Character::with_attributes(ch, attr));
    }
}

fn write_leader(surface: &mut Surface, map: &PaletteTile, x: i32, y: i32, ch: char, attr: CharAttribute) {
    if (x >= 0) && (y >= 0) && map.get(x as u32, y as u32).is_none() {
        surface.write_char(x, y, Character::with_attributes(ch, attr));
    }
}

pub(super) fn paint_inside(slices: &[Slice], surface: &mut Surface, size: Size, theme: &Theme) {
    let Some(inner) = plot_area(size) else {
        return;
    };
    let attr = theme.chart.label;
    for s in slices.iter().filter(|s| has_label(s)) {
        let Some(center) = s.center else {
            continue;
        };
        let len = (text_len(s).min(inner.width())) as i32;
        let x = (center.x - len / 2).clamp(inner.left(), inner.right() - len + 1);
        write_text(surface, x, center.y, &s.text, len as usize, attr);
    }
}

// bounds of the drawn chart, from the cell map
fn drawn_bounds(map: &PaletteTile) -> Option<Rect> {
    let size = map.size();
    let (mut l, mut t, mut r, mut b) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for y in 0..size.height {
        for x in 0..size.width {
            if map.get(x, y).is_some() {
                l = l.min(x as i32);
                r = r.max(x as i32);
                t = t.min(y as i32);
                b = b.max(y as i32);
            }
        }
    }
    (l <= r).then(|| Rect::new(l, t, r, b))
}

pub(super) fn paint_outside(slices: &[Slice], surface: &mut Surface, map: &PaletteTile, area: Rect, mode: &DrawMode, theme: &Theme, places: &mut Vec<Place>) {
    let Some(inner) = plot_area(surface.size()) else {
        return;
    };
    let Some(drawn) = drawn_bounds(map) else {
        return;
    };
    let attr = theme.chart.label;
    let cap = side_width(inner) as i32;
    for side in [Side::Right, Side::Left, Side::Top, Side::Bottom] {
        places.clear();
        for (index, s) in slices.iter().enumerate() {
            if !has_label(s) || side_of(s, mode) != side {
                continue;
            }
            let anchor = anchor(s, side, area, map);
            let vertical = matches!(side, Side::Left | Side::Right);
            let len = if vertical { 1 } else { (text_len(s) as i32).min(inner.width() as i32) };
            let pos = if vertical { anchor.y } else { anchor.x - len / 2 };
            places.push(Place { index, pos, len, anchor });
        }
        match side {
            Side::Left | Side::Right => fit(places, slices, area.top(), area.bottom(), 0),
            Side::Top | Side::Bottom => fit(places, slices, inner.left(), inner.right(), 1),
        }
        for p in places.iter() {
            let s = &slices[p.index];
            // labels are placed next to the drawn chart: one line cell, one space, the text
            match side {
                Side::Right => {
                    let x = drawn.right() + 3;
                    let len = (text_len(s) as i32).min(cap).min(inner.right() - x + 1);
                    if len <= 0 {
                        continue;
                    }
                    write_text(surface, x, p.pos, &s.text, len as usize, attr);
                    if p.pos == p.anchor.y {
                        for lx in p.anchor.x..=x - 2 {
                            write_leader(surface, map, lx, p.pos, '─', attr);
                        }
                    }
                }
                Side::Left => {
                    let end = drawn.left() - 3;
                    let len = (text_len(s) as i32).min(cap).min(end - inner.left() + 1);
                    if len <= 0 {
                        continue;
                    }
                    write_text(surface, end - len + 1, p.pos, &s.text, len as usize, attr);
                    if p.pos == p.anchor.y {
                        for lx in end + 2..=p.anchor.x {
                            write_leader(surface, map, lx, p.pos, '─', attr);
                        }
                    }
                }
                Side::Top | Side::Bottom => {
                    let y = if side == Side::Top { drawn.top() - 2 } else { drawn.bottom() + 2 };
                    if (y < inner.top()) || (y > inner.bottom()) {
                        continue;
                    }
                    write_text(surface, p.pos, y, &s.text, p.len as usize, attr);
                    if (p.anchor.x >= p.pos) && (p.anchor.x < p.pos + p.len) {
                        let (from, to) = if side == Side::Top { (y + 1, p.anchor.y) } else { (p.anchor.y, y - 1) };
                        for ly in from..=to {
                            write_leader(surface, map, p.anchor.x, ly, '│', attr);
                        }
                    }
                }
            }
        }
    }
}
