use super::{Character, Size};

pub struct Sprite {
    chars: Vec<Character>,
    size: Size
}
impl Sprite {
    pub fn new(width: u32, height: u32) -> Self {
        let w = width.clamp(1, super::surface::MAX_SURFACE_WIDTH);
        let h = height.clamp(1, super::surface::MAX_SURFACE_HEIGHT);
        let count = (w as usize) * (h as usize);
        Self { 
            chars: vec![Character::default(); count],
            size: Size::new(w, h)
        }
    }
    #[inline(always)]
    pub fn size(&self) -> Size {
        self.size
    }
    #[inline(always)]
    pub fn get(&self, x: u32, y: u32) -> Option<Character> {
        if x >= self.size.width || y >= self.size.height {
            return None;
        }
        Some(self.chars[y as usize * self.size.width as usize + x as usize])
    }
    #[inline(always)]
    pub fn set(&mut self, x: u32, y: u32, ch: Character) {
        if x >= self.size.width || y >= self.size.height {
            return;
        }   
        self.chars[y as usize * self.size.width as usize + x as usize] = ch;
    }
    #[inline(always)]
    pub fn clear(&mut self, ch: Character) {
        self.chars.fill(ch);
    }
    #[inline(always)]
    pub fn resize(&mut self, width: u32, height: u32, ch: Character) {
        let w = width.clamp(1, super::surface::MAX_SURFACE_WIDTH);
        let h = height.clamp(1, super::surface::MAX_SURFACE_HEIGHT);
        let count = (w as usize) * (h as usize);
        self.chars.resize(count, ch);
        self.chars.fill(ch);
        self.size = Size::new(w, h);
    }
}