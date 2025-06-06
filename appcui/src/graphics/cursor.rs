const INVALID_CURSOR_COORDONATE: u32 = 0xFFFFFFFF;

pub(crate) struct Cursor {
    pub x: u32,
    pub y: u32,
}

impl Cursor {
    pub(crate) fn new() -> Cursor {
        Cursor {
            x: INVALID_CURSOR_COORDONATE,
            y: INVALID_CURSOR_COORDONATE,
        }
    }
    pub(crate) fn set(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
    pub(crate) fn hide(&mut self) {
        self.x = INVALID_CURSOR_COORDONATE;
        self.y = INVALID_CURSOR_COORDONATE;
    }
    pub(crate) fn is_visible(&self) -> bool {
        self.x != INVALID_CURSOR_COORDONATE
    }
}
