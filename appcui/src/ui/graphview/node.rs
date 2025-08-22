use super::GraphNode;
use crate::{graphics::Rect, prelude::Point};

pub struct Node<T: GraphNode> {
    obj: T,
    rect: Rect,
}
impl<T> Node<T>
where
    T: GraphNode,
{
    pub(super) fn new(x: i32, y: i32, w: u32, h: u32, obj: T) -> Self {
        Self {
            obj,
            rect: Rect::new(x, y, x + w as i32, y + h as i32),
        }
    }
    #[inline]
    pub(super) fn contains(&self, x: i32, y: i32) -> bool {
        self.rect.contains(Point::new(x, y))
    }
}
