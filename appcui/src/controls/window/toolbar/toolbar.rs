use crate::{
    graphics::{Size, Surface},
    system::{Handle, Theme},
    utils::{HandleManager, VectorIndex},
};

use super::{
    CloseButton, Gravity, HotKey, Label, MaximizeRestoreButton, PaintData, PositionHelper,
    ResizeCorner, Tag, ToolBarItem, ToolBarItemHandle,
};

pub struct ToolBar {
    pub(super) items: HandleManager<ToolBarItem>,
    current: VectorIndex,
    pressed: bool,
}

pub trait AddToToolbar {
    fn add(self, toolbar: &mut ToolBar) -> Handle;
}

impl ToolBar {
    pub(crate) fn new() -> Self {
        ToolBar {
            items: HandleManager::new(4),
            pressed: false,
            current: VectorIndex::default(),
        }
    }
    pub fn add<T>(&mut self, item: T) -> ToolBarItemHandle<T>
    where
        T: AddToToolbar,
    {
        ToolBarItemHandle::new(AddToToolbar::add(item, self))
    }
    pub fn get<T>(&self, handle: ToolBarItemHandle<T>) -> Option<&T> {
        if let Some(obj) = self.items.get(handle.handle) {
            match obj {
                ToolBarItem::Label(obj) => {
                    return Some(unsafe { &(*((obj as *const Label) as *const T)) })
                }
                ToolBarItem::HotKey(obj) => {
                    return Some(unsafe { &(*((obj as *const HotKey) as *const T)) })
                }
                ToolBarItem::Tag(obj) => {
                    return Some(unsafe { &(*((obj as *const Tag) as *const T)) })
                }
                ToolBarItem::CloseButton(obj) => {
                    return Some(unsafe { &(*((obj as *const CloseButton) as *const T)) })
                }
                ToolBarItem::MaximizeRestoreButton(obj) => {
                    return Some(unsafe { &(*((obj as *const MaximizeRestoreButton) as *const T)) })
                }
                ToolBarItem::ResizeCorner(obj) => {
                    return Some(unsafe { &(*((obj as *const ResizeCorner) as *const T)) })
                }
            }
        }
        None
    }
    pub fn get_mut<T>(&self, handle: ToolBarItemHandle<T>) -> Option<&mut T> {
        if let Some(obj) = self.items.get_mut(handle.handle) {
            match obj {
                ToolBarItem::Label(obj) => {
                    return Some(unsafe { &mut (*((obj as *mut Label) as *mut T)) })
                }
                ToolBarItem::HotKey(obj) => {
                    return Some(unsafe { &mut (*((obj as *mut HotKey) as *mut T)) })
                }
                ToolBarItem::Tag(obj) => {
                    return Some(unsafe { &mut (*((obj as *mut Tag) as *mut T)) })
                }
                ToolBarItem::CloseButton(obj) => {
                    return Some(unsafe { &mut (*((obj as *mut CloseButton) as *mut T)) })
                }
                ToolBarItem::MaximizeRestoreButton(obj) => {
                    return Some(unsafe { &mut (*((obj as *mut MaximizeRestoreButton) as *mut T)) })
                }
                ToolBarItem::ResizeCorner(obj) => {
                    return Some(unsafe { &mut (*((obj as *mut ResizeCorner) as *mut T)) })
                }
            }
        }
        None
    }
    fn update_position_from_left(&mut self, index: usize, helper: &mut PositionHelper, right: i32) {
        if let Some(d) = self.items.get_element_mut(index) {
            let pos = d.get_base_mut();
            let my_variant = Some(std::mem::discriminant(d));
            let (next, add_flags) =
                pos.update_position_from_left(helper.x, helper.y, my_variant, helper.variant);
            let last_index = helper.index;
            if next < right {
                pos.set_visible();
                helper.index = VectorIndex::with_value(index);
                helper.x = next;
                helper.variant = my_variant;
            }
            if add_flags && last_index.is_valid() {
                if let Some(last) = self.items.get_element_mut(last_index.index()) {
                    last.get_base_mut().set_right_marker();
                }
            }
        }
    }
    fn update_position_from_right(&mut self, index: usize, helper: &mut PositionHelper, left: i32) {
        if let Some(d) = self.items.get_element_mut(index) {
            let pos = d.get_base_mut();
            let my_variant = Some(std::mem::discriminant(d));
            let (next, add_flags) =
                pos.update_position_from_right(helper.x, helper.y, my_variant, helper.variant);
            let last_index = helper.index;
            if next > left {
                pos.set_visible();
                helper.index = VectorIndex::with_value(index);
                helper.x = next;
                helper.variant = my_variant;
            }
            if add_flags && last_index.is_valid() {
                if let Some(last) = self.items.get_element_mut(last_index.index()) {
                    last.get_base_mut().set_left_marker();
                }
            }
        }
    }
    pub(crate) fn update_positions(&mut self, size: Size) -> (i32, i32) {
        // clear all flags (visible & left|right marker)
        let count = self.items.allocated_objects();
        for index in 0..count {
            if let Some(d) = self.items.get_element_mut(index) {
                d.get_base_mut().clear();
            }
        }
        let mut top_left = PositionHelper::new(1, 0);
        let mut top_right = PositionHelper::new((size.width as i32) - 2, 0);
        let mut bottom_left = PositionHelper::new(1, (size.height as i32) - 1);
        let mut bottom_right =
            PositionHelper::new((size.width as i32) - 1, (size.height as i32) - 1);

        for index in 0..count {
            if let Some(d) = self.items.get_element_mut(index) {
                let pos = d.get_base();
                if pos.is_hidden() {
                    continue;
                }
                let gravity = pos.get_gravity();
                match gravity {
                    Gravity::TopLeft => {
                        self.update_position_from_left(index, &mut top_left, top_right.x);
                    }
                    Gravity::BottomLeft => {
                        self.update_position_from_left(index, &mut bottom_left, bottom_right.x)
                    }
                    Gravity::TopRight => {
                        self.update_position_from_right(index, &mut top_right, top_left.x);
                    }
                    Gravity::BottomRight => {
                        self.update_position_from_right(index, &mut bottom_right, bottom_left.x);
                    }
                }
            }
        }

        // last elements
        if top_left.index.is_valid() {
            if let Some(item) = self.items.get_element_mut(top_left.index.index()) {
                item.get_base_mut().set_right_marker();
            }
        }
        if bottom_left.index.is_valid() {
            if let Some(item) = self.items.get_element_mut(bottom_left.index.index()) {
                item.get_base_mut().set_right_marker();
            }
        }
        if top_right.index.is_valid() {
            if let Some(item) = self.items.get_element_mut(top_right.index.index()) {
                item.get_base_mut().set_left_marker();
            }
        }
        if bottom_right.index.is_valid() {
            if let Some(item) = self.items.get_element_mut(bottom_right.index.index()) {
                item.get_base_mut().set_left_marker();
            }
        }
        //let title_x_pos = top_left.x + 1;
        //let title_space = (top_right.x - title_x_pos).max(0);
        //(title_x_pos, title_space as u16)
        (top_left.x + 1, top_right.x)
    }
    pub(crate) fn paint(
        &self,
        surface: &mut Surface,
        theme: &Theme,
        focused: bool,
        maximized: bool,
    ) {
        let mut paint_data = PaintData {
            focused,
            current: false,
            maximized,
            is_current_item_pressed: self.pressed,
            sep_attr: if focused {
                theme.lines.normal
            } else {
                theme.lines.inactive
            },
        };
        let current_bar_index = self.current.index();
        let count = self.items.allocated_objects();
        // paint bar items
        for index in 0..count {
            if let Some(item) = self.items.get_element_mut(index) {
                paint_data.current = index == current_bar_index;
                item.paint(surface, theme, &paint_data);
            }
        }
    }
}
