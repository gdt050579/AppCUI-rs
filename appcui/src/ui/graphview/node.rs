use super::GraphNode;
use crate::{graphics::Rect, prelude::Point};

pub struct Node<T: GraphNode> {
    pub(super) obj: T,
    pub(super) rect: Rect,
}
impl<T> Node<T>
where
    T: GraphNode,
{
    pub(super) fn new(obj: T) -> Self {
        Self {
            obj,
            rect: Rect::new(0, 0, 1, 1),
        }
    }
    #[inline]
    pub(super) fn contains(&self, x: i32, y: i32) -> bool {
        self.rect.contains(Point::new(x, y))
    }
}
