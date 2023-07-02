use crate::utils::VectorIndex;

pub (super) struct PositionHelper {
    pub (super) x: i32,
    pub (super) y: i32,
    pub (super) index: VectorIndex,
    pub (super) decoraror_type: DecoratorType,
}
impl PositionHelper {
    pub (super) fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            index: VectorIndex::default(),
            decoraror_type: DecoratorType::None,
        }
    }
}