use std::ptr::NonNull;

use crate::{
    graphics::{Size, Surface},
    system::{Handle, Theme},
    ui::common::UIElement,
    utils::{HandleManager, VectorIndex},
    input::Key
};

use super::{
    Button, CheckBox, CloseButton, GroupPosition, HotKey, Label, MaximizeRestoreButton, PaintData, PositionHelper, ResizeCorner, SingleChoice, Tag,
    ToolBarItem, group::Group,
};

pub struct ToolbarElementHandle {
    group: Group,
    handle: Handle<UIElement>
}

pub struct ToolBar {
    pub(super) items: HandleManager<ToolBarItem>,
    current_handle: Handle<UIElement>,
    order: Vec<ToolbarElementHandle>,
    pressed: bool,
    last_group_index: u8
}

pub trait AddToToolbar<T> {
    fn add(self, toolbar: &mut ToolBar, group: Group) -> Handle<T>;
}

impl ToolBar {
    pub(crate) fn new() -> Self {
        ToolBar {
            items: HandleManager::new(4),
            pressed: false,
            order: Vec::with_capacity(4),
            current_handle: Handle::None,
            last_group_index: 0
        }
    }
    pub fn create_group(&mut self, pos: GroupPosition) -> Group {
        if self.last_group_index == 255 {
            Group { pos, id: 255u8 }
        } else {
            let g = Group{pos, id: self.last_group_index};
            self.last_group_index+=1;
            g
        }
    }
    pub fn add<T>(&mut self, group: Group, item: T) -> Handle<T>
    where
        T: AddToToolbar<T>,
    {        
        let h = AddToToolbar::add(item, self, group);
        self.order.push(ToolbarElementHandle { group, handle: h.cast() });
        self.order.sort_by_key(|k| k.group.id);
        h
    }
    pub fn get<T>(&self, handle: Handle<T>) -> Option<&T> {
        if let Some(obj) = self.items.get(handle.cast()) {
            match obj {
                ToolBarItem::Label(obj) => return Some(unsafe { &(*((obj as *const Label) as *const T)) }),
                ToolBarItem::HotKey(obj) => return Some(unsafe { &(*((obj as *const HotKey) as *const T)) }),
                ToolBarItem::Tag(obj) => return Some(unsafe { &(*((obj as *const Tag) as *const T)) }),
                ToolBarItem::CloseButton(obj) => return Some(unsafe { &(*((obj as *const CloseButton) as *const T)) }),
                ToolBarItem::MaximizeRestoreButton(obj) => return Some(unsafe { &(*((obj as *const MaximizeRestoreButton) as *const T)) }),
                ToolBarItem::ResizeCorner(obj) => return Some(unsafe { &(*((obj as *const ResizeCorner) as *const T)) }),
                ToolBarItem::Button(obj) => return Some(unsafe { &(*((obj as *const Button) as *const T)) }),
                ToolBarItem::CheckBox(obj) => return Some(unsafe { &(*((obj as *const CheckBox) as *const T)) }),
                ToolBarItem::SingleChoice(obj) => return Some(unsafe { &(*((obj as *const SingleChoice) as *const T)) }),
            }
        }
        None
    }
    pub fn get_mut<T>(&mut self, handle: Handle<T>) -> Option<&mut T> {
        let toolbar_ptr = unsafe { NonNull::new_unchecked(self as *mut ToolBar) } ; 
        if let Some(obj) = self.items.get_mut(handle.cast()) {
            match obj {
                ToolBarItem::Label(obj) => return Some(unsafe { &mut (*((obj as *mut Label) as *mut T)) }),
                ToolBarItem::HotKey(obj) => return Some(unsafe { &mut (*((obj as *mut HotKey) as *mut T)) }),
                ToolBarItem::Tag(obj) => return Some(unsafe { &mut (*((obj as *mut Tag) as *mut T)) }),
                ToolBarItem::CloseButton(obj) => return Some(unsafe { &mut (*((obj as *mut CloseButton) as *mut T)) }),
                ToolBarItem::MaximizeRestoreButton(obj) => return Some(unsafe { &mut (*((obj as *mut MaximizeRestoreButton) as *mut T)) }),
                ToolBarItem::ResizeCorner(obj) => return Some(unsafe { &mut (*((obj as *mut ResizeCorner) as *mut T)) }),
                ToolBarItem::Button(obj) => return Some(unsafe { &mut (*((obj as *mut Button) as *mut T)) }),
                ToolBarItem::CheckBox(obj) => return Some(unsafe { &mut (*((obj as *mut CheckBox) as *mut T)) }),
                ToolBarItem::SingleChoice(obj) => {
                    obj.tooldbar = Some(toolbar_ptr);
                    return Some(unsafe { &mut (*((obj as *mut SingleChoice) as *mut T)) });
                }
            }
        }
        None
    }
    #[inline(always)]
    pub(crate) fn set_current_item_handle(&mut self, handle: Handle<UIElement>) {
        self.current_handle = handle;
    }
    #[inline(always)]
    pub(crate) fn clear_current_item_handle(&mut self) {
        self.current_handle = Handle::None;
    }
    #[inline(always)]
    pub(crate) fn get_current_item_handle(&self) -> Handle<UIElement> {
        self.current_handle
    }
    #[inline(always)]
    pub(crate) fn get_item(&self, handle: Handle<UIElement>) -> Option<&ToolBarItem> {
        self.items.get(handle.cast())
    }
    #[inline(always)]
    pub(crate) fn get_item_mut(&mut self, handle: Handle<UIElement>) -> Option<&mut ToolBarItem> {
        self.items.get_mut(handle.cast())
    }

    #[inline(always)]
    pub(crate) fn is_current_item_pressed(&self) -> bool {
        self.pressed
    }
    #[inline(always)]
    pub(crate) fn set_current_item_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }

    pub(crate) fn get_from_position(&self, x: i32, y: i32) -> Option<&ToolBarItem> {
        let count = self.items.allocated_objects();
        for index in 0..count {
            if let Some(item) = self.items.get_element(index) {
                if item.get_base().contains(x, y) {
                    return Some(item);
                }
            }
        }
        None
    }

    fn update_position_from_left(&mut self, index: usize, helper: &mut PositionHelper, right: i32) {
        if let Some(item) = self.items.get_element_mut(index) {
            let my_variant = Some(std::mem::discriminant(item));
            let pos = item.get_base_mut();
            let (next, add_flags, add_space) = pos.update_position_from_left(
                if helper.add_space_before_next { helper.x + 1 } else { helper.x },
                helper.y,
                my_variant,
                helper.variant,
            );
            let last_index = helper.index;
            if next < right {
                helper.index = VectorIndex::with_value(index);
                helper.x = next;
                helper.variant = my_variant;
                helper.add_space_before_next = add_space;
            } else {
                pos.set_outside_drawing_area();
            }
            if add_flags && last_index.is_valid() {
                if let Some(last) = self.items.get_element_mut(last_index.index()) {
                    last.get_base_mut().set_right_marker();
                }
            }
        }
    }
    fn update_position_from_right(&mut self, index: usize, helper: &mut PositionHelper, left: i32) {
        if let Some(item) = self.items.get_element_mut(index) {
            let my_variant = Some(std::mem::discriminant(item));
            let pos = item.get_base_mut();
            let (next, add_flags, add_space) = pos.update_position_from_right(
                if helper.add_space_before_next { helper.x - 1 } else { helper.x },
                helper.y,
                my_variant,
                helper.variant,
            );
            let last_index = helper.index;
            if next > left {
                helper.index = VectorIndex::with_value(index);
                helper.x = next;
                helper.variant = my_variant;
                helper.add_space_before_next = add_space;
            } else {
                pos.set_outside_drawing_area();
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
        let mut bottom_right = PositionHelper::new((size.width as i32) - 2, (size.height as i32) - 1);

        for index in 0..count {
            if let Some(d) = self.items.get_element_mut(index) {
                let base = d.get_base();
                if !base.is_visible() {
                    continue;
                }
                let gravity = base.get_gravity();
                match gravity {
                    GroupPosition::TopLeft => {
                        self.update_position_from_left(index, &mut top_left, top_right.x);
                    }
                    GroupPosition::BottomLeft => self.update_position_from_left(index, &mut bottom_left, bottom_right.x),
                    GroupPosition::TopRight => {
                        self.update_position_from_right(index, &mut top_right, top_left.x);
                    }
                    GroupPosition::BottomRight => {
                        match d {
                            ToolBarItem::ResizeCorner(_) => bottom_right.x += 1,
                            _ => {}
                        }
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
        (top_left.x + 1, top_right.x)
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme, focused: bool, maximized: bool) {
        let mut paint_data = PaintData {
            focused,
            current: false,
            maximized,
            is_current_item_pressed: self.pressed,
            sep_attr: if focused { theme.lines.normal } else { theme.lines.inactive },
        };
        let current_bar_index = self.current_handle.get_index();
        let count = self.items.allocated_objects();
        // paint bar items
        for index in 0..count {
            if let Some(item) = self.items.get_element(index) {
                paint_data.current = index == current_bar_index;
                item.paint(surface, theme, &paint_data);
            }
        }
    }

    pub(crate) fn update_singlechoice_group_id(&mut self, group_id: u32, handle: Handle<UIElement>) {
        let count = self.items.allocated_objects();
        // paint bar items
        for index in 0..count {
            if let Some(item) = self.items.get_element_mut(index) {
                if let ToolBarItem::SingleChoice(sc) = item {
                    if sc.get_group_id() == group_id {
                        sc.update_select_status(handle == sc.handle);
                    }
                }
            }
        }
    }

    pub(crate) fn hotkey_to_item(&self, hotkey: Key) -> Option<Handle<UIElement>> {
        let count = self.items.allocated_objects();
        // paint bar items
        for index in 0..count {
            if let Some(item) = self.items.get_element(index) {
                if item.get_hotkey() == hotkey {
                    return Some(item.get_handle());
                }
            }
        }
        None     
    }
}
