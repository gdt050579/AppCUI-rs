#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Size {
        Size {
            width: width,
            height: height,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self { width: 0, height: 0 }
    }
}
