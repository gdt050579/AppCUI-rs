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
    pub fn reduce_by(&self, value: u32) -> Size {
        Size {
            width: if self.width > value { self.width - value } else { 0 },
            height: if self.height > value { self.height - value } else { 0 },
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self { width: 0, height: 0 }
    }
}
