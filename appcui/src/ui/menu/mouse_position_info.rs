use crate::utils::VectorIndex;

use super::Menu;

pub(super) struct MousePositionInfo {
    pub(super) item_index: VectorIndex,
    pub(super) is_on_menu: bool,
    pub(super) is_on_up_button: bool,
    pub(super) is_on_down_button: bool,
}
impl MousePositionInfo {
    pub(super) fn new(x: i32, y: i32, menu: &Menu) -> Self {
        let mut mpi = MousePositionInfo {
            item_index: VectorIndex::Invalid,
            is_on_menu: false,
            is_on_up_button: false,
            is_on_down_button: false,
        };
        if (x >= 1) && (y >= 1) && (x <= (menu.width as i32)) && (y <= (menu.visible_items_count as i32)) {
            let item_index = ((y - 1) as usize) + (menu.first_visible_item as usize);
            let idx = item_index;
            if idx < menu.items.len() {
                let item = &menu.items[idx];
                mpi.item_index = if (item.is_enabled()) && !item.is_line() {
                    VectorIndex::with_value(item_index)
                } else {
                    VectorIndex::Invalid
                };
            }
        }
        let center_x = (menu.width >> 1) as i32;
        mpi.is_on_menu = (x >= 0) && (y >= 0) && (x < (menu.width as i32) + 2) && (y < (menu.visible_items_count as i32) + 2);
        if (x >= center_x) && (x <= center_x + 2) {
            mpi.is_on_up_button = y == 0;
            mpi.is_on_down_button = y == (menu.clip.bottom - menu.clip.top);
        }
        mpi
    }
}
