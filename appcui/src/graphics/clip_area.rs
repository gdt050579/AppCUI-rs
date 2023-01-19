#[derive(Copy, Clone, Debug)]
pub(crate) struct ClipArea {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    visible: bool,
}

impl ClipArea {
    #[inline]
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> ClipArea {
        ClipArea {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
            visible: (left <= right) && (top < bottom),
        }
    }
    #[inline]
    pub fn set(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
        self.visible = (left <= right) && (top <= bottom);
    }
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    #[inline]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        return self.visible
            && (x >= self.left)
            && (x <= self.right)
            && (y >= self.top)
            && (y <= self.bottom);
    }
    #[inline]
    pub fn contains_y(&self, y: i32) -> bool {
        return self.visible && (y >= self.top) && (y <= self.bottom);
    }
    #[inline]
    pub fn intersect_with(&mut self, clip: &ClipArea) {
        self.set(
            i32::max(self.left, clip.left),
            i32::max(self.top, clip.top),
            i32::min(self.right, clip.right),
            i32::min(self.bottom, clip.bottom),
        )
    }
}

impl Default for ClipArea {
    fn default() -> Self {
        Self { left: 0, top: 0, right: 0, bottom: 0, visible: false }
    }
}