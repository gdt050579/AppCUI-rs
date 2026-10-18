use crate::graphics::{CharAttribute, Character, Rect, Size, Surface};

pub(super) struct PaintMap {
    map: Vec<u16>,
    size: Size,
}

impl PaintMap {
    const INVALID: u16 = 0xFFFF;
    pub(super) fn new(size: Size) -> Self {
        Self {
            map: vec![Self::INVALID; size.width as usize * size.height as usize],
            size,
        }
    }
    pub(super) fn resize(&mut self, size: Size) {
        self.map.resize(size.width as usize * size.height as usize, Self::INVALID);
        self.map.fill(Self::INVALID);
        self.size = size;
    }
    pub(super) fn size(&self) -> Size {
        self.size
    }
    pub(super) fn reset(&mut self) {
        self.map.fill(Self::INVALID);
    }
    pub(super) fn get(&self, x: usize, y: usize) -> Option<u16> {
        if x >= self.size.width as usize || y >= self.size.height as usize {
            return None;
        }
        let index = self.map[y * self.size.width as usize + x];
        if index == Self::INVALID {
            return None;
        }
        Some(index)
    }
    pub(super) fn set(&mut self, x: usize, y: usize, value: u16) {
        if x >= self.size.width as usize || y >= self.size.height as usize {
            return;
        }
        self.map[y * self.size.width as usize + x] = value;
    }
    #[allow(dead_code)]
    pub(super) fn clear(&mut self, x: usize, y: usize) {
        if x >= self.size.width as usize || y >= self.size.height as usize {
            return;
        }
        self.map[y * self.size.width as usize + x] = Self::INVALID;
    }
    pub(super) fn dim(&mut self, index: usize, surface: &mut Surface, attr: CharAttribute) {
        if index >= Self::INVALID as usize {
            return;
        }
        let idx = Some(index as u16);
        let r = Rect::with_size(0, 0, self.size.width as u16, self.size.height as u16);
        surface.transform_rect(r, |ch,p| {
            if self.get(p.x as usize, p.y as usize) == idx {
                None
            } else {
                Some(Character::with_attributes(ch.code, attr))
            }
        });
    }
    pub(super) fn paint_selection(&mut self, index: usize, surface: &mut Surface, attr: CharAttribute) {
        if index >= Self::INVALID as usize {
            return;
        }
        let idx = index as u16;
        let w = self.size.width as usize;
        let h = self.size.height as usize;
        for y in 0..h {
            for x in 0..w {
                if self.is_selected(x as isize, y as isize, idx) {
                    continue;
                }
                let mask = self.neighbours(x, y, idx);
                if mask != 0 {
                    surface.write_char(x as i32, y as i32, Character::with_attributes(Self::selection_char(mask), attr));
                }
            }
        }
    }

    #[inline(always)]
    fn is_slice(&self, x: isize, y: isize, idx: u16) -> bool {
        (x >= 0) && (y >= 0) && (x < self.size.width as isize) && (y < self.size.height as isize) && (self.map[y as usize * self.size.width as usize + x as usize] == idx)
    }
    fn is_selected(&self, x: isize, y: isize, idx: u16) -> bool {
        if self.is_slice(x, y, idx) {
            return true;
        }
        let sides = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().filter(|(dx, dy)| self.is_slice(x + dx, y + dy, idx)).count();
        sides >= 3
    }
    fn neighbours(&self, x: usize, y: usize, idx: u16) -> u8 {
        const OFFSETS: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
        let mut mask = 0u8;
        for (bit, (dx, dy)) in OFFSETS.iter().enumerate() {
            if self.is_selected(x as isize + dx, y as isize + dy, idx) {
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
}