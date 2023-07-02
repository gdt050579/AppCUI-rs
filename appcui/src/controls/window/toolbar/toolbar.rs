use crate::{
    graphics::Size,
    system::Handle,
    utils::{HandleManager, VectorIndex},
};

use super::{PositionHelper, ToolBarItem, ToolbarItemLayout};

pub struct ToolBar {
    pub(super) items: HandleManager<ToolBarItem>,
}

pub trait AddToToolbar {
    fn add(self, toolbar: &mut ToolBar) -> Handle;
}

impl ToolBar {
    pub(super) fn new() -> Self {
        ToolBar {
            items: HandleManager::new(4),
        }
    }
    pub fn add<T>(&mut self, item: T)
    where
        T: AddToToolbar,
    {
        let _h = AddToToolbar::add(item, self);
    }
    fn update_position_from_left(&mut self, index: usize, helper: &mut PositionHelper, right: i32) {
        if let Some(d) = self.items.get_element_mut(index) {
            let pos = d.get_position_mut();
            let (next, add_flags) =
                pos.update_position_from_left(helper.x, helper.y, helper.decoraror_type);
            let last_index = helper.index;
            if next < right {
                pos.set_visible();
                helper.index = VectorIndex::with_value(index);
                helper.x = next;
                helper.decoraror_type = d.get_type();
            }
            if add_flags && last_index.is_valid() {
                if let Some(last) = self.items.get_element_mut(last_index.index()) {
                    last.get_position_mut().set_right_marker();
                }
            }
        }
    }
    fn update_position_from_right(&mut self, index: usize, helper: &mut PositionHelper, left: i32) {
        if let Some(d) = self.items.get_element_mut(index) {
            let pos = d.get_position_mut();
            let (next, add_flags) =
                pos.update_position_from_right(helper.x, helper.y, helper.decoraror_type);
            let last_index = helper.index;
            if next > left {
                pos.set_visible();
                helper.index = VectorIndex::with_value(index);
                helper.x = next;
                helper.decoraror_type = d.get_type();
            }
            if add_flags && last_index.is_valid() {
                if let Some(last) = self.items.get_element_mut(last_index.index()) {
                    last.get_position_mut().set_left_marker();
                }
            }
        }
    }
    pub(super) fn update_positions(&mut self, size: Size) -> (i32, i32) {
        // clear all flags (visible & left|right marker)
        let count = self.items.allocated_objects();
        for index in 0..count {
            if let Some(d) = self.items.get_element_mut(index) {
                d.get_position_mut().clear();
            }
        }
        let mut top_left = PositionHelper::new(1, 0);
        let mut top_right = PositionHelper::new((size.width as i32) - 2, 0);
        let mut bottom_left = PositionHelper::new(1, (size.height as i32) - 1);
        let mut bottom_right =
            PositionHelper::new((size.width as i32) - 1, (size.height as i32) - 1);

        for index in 0..count {
            if let Some(d) = self.items.get_element_mut(index) {
                let pos = d.get_position();
                if pos.is_hidden() {
                    continue;
                }
                let layout = pos.get_layout();
                match layout {
                    ToolbarItemLayout::TopLeft => {
                        self.update_position_from_left(index, &mut top_left, top_right.x);
                    }
                    ToolbarItemLayout::BottomLeft => {
                        self.update_position_from_left(index, &mut bottom_left, bottom_right.x)
                    }
                    ToolbarItemLayout::TopRight => {
                        self.update_position_from_right(index, &mut top_right, top_left.x);
                    }
                    ToolbarItemLayout::BottomRight => {
                        self.update_position_from_right(index, &mut bottom_right, bottom_left.x);
                    }
                }
            }
        }

        // last elements
        if top_left.index.is_valid() {
            if let Some(item) = self.items.get_element_mut(top_left.index.index()) {
                item.get_position_mut().set_right_marker();
            }
        }
        if bottom_left.index.is_valid() {
            if let Some(item) = self.items.get_element_mut(bottom_left.index.index()) {
                item.get_position_mut().set_right_marker();
            }
        }
        if top_right.index.is_valid() {
            if let Some(item) = self.items.get_element_mut(top_right.index.index()) {
                item.get_position_mut().set_left_marker();
            }
        }
        if bottom_right.index.is_valid() {
            if let Some(item) = self.items.get_element_mut(bottom_right.index.index()) {
                item.get_position_mut().set_left_marker();
            }
        }
        //let title_x_pos = top_left.x + 1;
        //let title_space = (top_right.x - title_x_pos).max(0);
        //(title_x_pos, title_space as u16)
        (top_left.x + 1, top_right.x)
    }
}
