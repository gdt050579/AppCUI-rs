const INVALID_CURSOR_COORDONATE: u32 = 0xFFFFFFFF;

pub struct Cursor {
    pub x: u32,
    pub y: u32
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {x: INVALID_CURSOR_COORDONATE, y: INVALID_CURSOR_COORDONATE}
    }
    pub fn set(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
    pub fn hide(&mut self) {
        self.x = INVALID_CURSOR_COORDONATE;
        self.y = INVALID_CURSOR_COORDONATE;
    }
    pub fn is_visible(&self) -> bool {
        self.x != INVALID_CURSOR_COORDONATE
    }
}