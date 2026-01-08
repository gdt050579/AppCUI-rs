#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellSize {
    /// Width of a single character cell in pixels
    pub width: u16,
    /// Height of a single character cell in pixels
    pub height: u16,
}

impl CellSize {
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub const fn default_size() -> Self {
        Self { width: 8, height: 16 }
    }

    pub fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }
}

impl Default for CellSize {
    fn default() -> Self {
        Self::default_size()
    }
}
