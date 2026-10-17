use crate::graphics::Size;

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
}