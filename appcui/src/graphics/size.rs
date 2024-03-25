#[derive(Copy, Clone, PartialEq, Debug, Eq, Default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Size {
        Size { width, height }
    }
    pub fn reduce_by(&self, value: u32) -> Size {
        Size {
            width: if self.width > value { self.width - value } else { 0 },
            height: if self.height > value { self.height - value } else { 0 },
        }
    }
}
