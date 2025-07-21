use crate::ui::Dock;

use super::Alignment;
use super::Coordinate16;
use super::Dimension16;
use super::Layout;
use super::Pivot;

pub struct LayoutBuilder {
    pub(super) inner_layout: Layout,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        Self {
            inner_layout: Layout {
                x: None,
                y: None,
                width: None,
                height: None,
                a_left: None,
                a_right: None,
                a_top: None,
                a_bottom: None,
                align: None,
                pivot: None,
                dock: None,
            },
        }
    }

    pub fn x<T>(mut self, x: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.x = Some(x.into());
        self
    }

    pub fn y<T>(mut self, y: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.y = Some(y.into());
        self
    }

    pub fn width<T>(mut self, width: T) -> Self
    where
        Dimension16: From<T>,
    {
        self.inner_layout.width = Some(width.into());
        self
    }

    pub fn height<T>(mut self, height: T) -> Self
    where
        Dimension16: From<T>,
    {
        self.inner_layout.height = Some(height.into());
        self
    }

    pub fn alignment(mut self, align: Alignment) -> Self {
        self.inner_layout.align = Some(align);
        self
    }

    pub fn pivot(mut self, pivot: Pivot) -> Self {
        self.inner_layout.pivot = Some(pivot);
        self
    }

    pub fn dock(mut self, dock: Dock) -> Self {
        self.inner_layout.dock = Some(dock);
        self
    }

    pub fn left_anchor<T>(mut self, left_anchor: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.a_left = Some(left_anchor.into());
        self
    }

    pub fn right_anchor<T>(mut self, right_anchor: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.a_right = Some(right_anchor.into());
        self
    }

    pub fn top_anchor<T>(mut self, top_anchor: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.a_top = Some(top_anchor.into());
        self
    }

    pub fn bottom_anchor<T>(mut self, bottom_anchor: T) -> Self
    where
        Coordinate16: From<T>,
    {
        self.inner_layout.a_bottom = Some(bottom_anchor.into());
        self
    }

    pub fn build(self) -> Layout {
        self.inner_layout
    }
}
