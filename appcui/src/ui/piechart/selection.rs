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

fn is_selected(cells: &PaletteTile, x: isize, y: isize, idx: u16) -> bool {
    if is_slice(cells, x, y, idx) {
        return true;
    }
    let sides = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().filter(|(dx, dy)| is_slice(cells, x + dx, y + dy, idx)).count();
    sides >= 3
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
