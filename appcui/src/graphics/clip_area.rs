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
    pub fn with_size(x: i32, y: i32, width: u16, height: u16) -> ClipArea {
        ClipArea::new(x, y, x + (width as i32) - 1, y + (height as i32) - 1)
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
    pub fn set_with_size(&mut self, x: i32, y: i32, width: u16, height: u16) {
        self.set(x, y, x + (width as i32) - 1, y + (height as i32) - 1);
    }
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    #[inline(always)]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        return self.visible && (x >= self.left) && (x <= self.right) && (y >= self.top) && (y <= self.bottom);
    }
    #[inline(always)]
    pub(crate) fn contains_with_margins(&self, x: i32, y: i32, right_margin: i32, bottom_margin: i32) -> bool {
        return self.visible && (x >= self.left) && (x <= self.right + right_margin) && (y >= self.top) && (y <= self.bottom + bottom_margin);
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
    #[inline]
    pub fn reduce_margins(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.left += left;
        self.right -= right;
        self.top += top;
        self.bottom -= bottom;
        self.visible = (self.left <= self.right) && (self.top <= self.bottom);
    }
}

impl Default for ClipArea {
    fn default() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            visible: false,
        }
    }
}
