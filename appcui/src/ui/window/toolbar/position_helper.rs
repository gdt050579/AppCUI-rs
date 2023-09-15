use crate::system::Handle;
use crate::ui::common::UIElement;

pub (super) struct PositionHelper {
    pub (super) x: i32,
    pub (super) y: i32,
    pub (super) last_handle: Handle<UIElement>,
    pub (super) last_group: u8,
}
impl PositionHelper {
    pub (super) fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            last_handle: Handle::None,
            last_group: 0u8,
        }
    }
}