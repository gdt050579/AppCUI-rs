use crate::utils::VectorIndex;

use super::ToolBarItem;

pub (super) struct PositionHelper {
    pub (super) x: i32,
    pub (super) y: i32,
    pub (super) index: VectorIndex,
    pub (super) variant: Option<std::mem::Discriminant<ToolBarItem>>,
    pub (super) add_space_before_next: bool
}
impl PositionHelper {
    pub (super) fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            index: VectorIndex::default(),
            variant: None,
            add_space_before_next: false
        }
    }
}