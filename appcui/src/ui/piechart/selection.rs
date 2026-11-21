use crate::graphics::{CharAttribute, Character, PaletteTile, Rect, Surface};

pub(super) fn dim(cells: &PaletteTile, index: usize, surface: &mut Surface, attr: CharAttribute) {
    if index >= PaletteTile::TRANSPARENT_U16 as usize {
        return;
    }
    let idx = Some(index as u16);
    let size = cells.size();
    let r = Rect::with_size(0, 0, size.width as u16, size.height as u16);
    surface.transform_rect(r, |ch, p| {
        if cells.get(p.x as u32, p.y as u32) == idx {
            None
        } else {
            Some(Character::with_attributes(ch.code, attr))
        }
    });
}

pub(super) fn paint_selection(cells: &PaletteTile, index: usize, surface: &mut Surface, attr: CharAttribute) {
    if index >= PaletteTile::TRANSPARENT_U16 as usize {
        return;
    }
    let idx = index as u16;
    let size = cells.size();
    for y in 0..size.height as isize {
        for x in 0..size.width as isize {
            if is_selected(cells, x, y, idx) {
                continue;
            }
            let mask = neighbours(cells, x, y, idx);
            if mask != 0 {
                surface.write_char(x as i32, y as i32, Character::with_attributes(selection_char(mask), attr));
            }
        }
    }
}

#[inline(always)]
fn is_slice(cells: &PaletteTile, x: isize, y: isize, idx: u16) -> bool {
    (x >= 0) && (y >= 0) && (cells.get(x as u32, y as u32) == Some(idx))
}

// a cell of the slice, or of a notch in it: a corridor one cell thick, between two parts of
// the slice, that the slice also closes at one end (any length; a cell with the slice on 3 or 4
// sides is the shortest one)
fn is_selected(cells: &PaletteTile, x: isize, y: isize, idx: u16) -> bool {
    if is_slice(cells, x, y, idx) {
        return true;
    }
    let up = is_slice(cells, x, y - 1, idx);
    let down = is_slice(cells, x, y + 1, idx);
    let left = is_slice(cells, x - 1, y, idx);
    let right = is_slice(cells, x + 1, y, idx);
    (up && down && (closed(cells, x, y, 1, 0, idx) || closed(cells, x, y, -1, 0, idx)))
        || (left && right && (closed(cells, x, y, 0, 1, idx) || closed(cells, x, y, 0, -1, idx)))
}

// walking from (x, y) in direction (dx, dy) along a corridor between two parts of the slice,
// the slice closes the corridor
fn closed(cells: &PaletteTile, x: isize, y: isize, dx: isize, dy: isize, idx: u16) -> bool {
    let size = cells.size();
    let (mut x, mut y) = (x + dx, y + dy);
    while (x >= 0) && (y >= 0) && (x < size.width as isize) && (y < size.height as isize) {
        if is_slice(cells, x, y, idx) {
            return true;
        }
        // the walls of the corridor
        let walls = if dx != 0 {
            is_slice(cells, x, y - 1, idx) && is_slice(cells, x, y + 1, idx)
        } else {
            is_slice(cells, x - 1, y, idx) && is_slice(cells, x + 1, y, idx)
        };
        if !walls {
            return false;
        }
        x += dx;
        y += dy;
    }
    false
}

fn neighbours(cells: &PaletteTile, x: isize, y: isize, idx: u16) -> u8 {
    const OFFSETS: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
    let mut mask = 0u8;
    for (bit, (dx, dy)) in OFFSETS.iter().enumerate() {
        if is_selected(cells, x + dx, y + dy, idx) {
            mask |= 1 << bit;
        }
    }
    mask
}

fn selection_char(mask: u8) -> char {
    const N: u8 = 0x01;
    const NE: u8 = 0x02;
    const E: u8 = 0x04;
    const SE: u8 = 0x08;
    const S: u8 = 0x10;
    const SW: u8 = 0x20;
    const W: u8 = 0x40;
    const NW: u8 = 0x80;
    let has = |b: u8| (mask & b) != 0;
    let up = !has(N) && (has(NW) || has(NE) || has(W) || has(E));
    let right = !has(E) && (has(NE) || has(SE) || has(N) || has(S));
    let down = !has(S) && (has(SW) || has(SE) || has(W) || has(E));
    let left = !has(W) && (has(NW) || has(SW) || has(N) || has(S));
    match (up, right, down, left) {
        (false, true, true, false) => '┌',
        (false, false, true, true) => '┐',
        (true, true, false, false) => '└',
        (true, false, false, true) => '┘',
        (true, true, true, false) => '├',
        (true, false, true, true) => '┤',
        (false, true, true, true) => '┬',
        (true, true, false, true) => '┴',
        (true, true, true, true) => '┼',
        (true, false, true, false) | (true, false, false, false) | (false, false, true, false) => '│',
        _ => '─',
    }
}
