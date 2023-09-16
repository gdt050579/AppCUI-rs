use std::ptr::NonNull;

use crate::{
    graphics::{Size, Surface},
    input::Key,
    prelude::{CharFlags, Character, Color},
    system::{Handle, Theme},
    ui::common::UIElement,
    utils::HandleManager,
};

use super::{
    group::Group, Button, CheckBox, CloseButton, GroupPosition, HotKey, Label, MaximizeRestoreButton, PaintData, PositionHelper, ResizeCorner,
    SingleChoice, Tag, ToolBarItem,
};

pub struct ToolbarElementHandle {
    group: Group,
    handle: Handle<UIElement>,
}

pub struct ToolBar {
    pub(super) items: HandleManager<ToolBarItem>,
    current_handle: Handle<UIElement>,
    order: Vec<ToolbarElementHandle>,
    pressed: bool,
    last_group_index: u8,
    // for debug purposes
    temp_top_left: i32,
    temp_top_right: i32,
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
            last_group_index: 0,
            temp_top_left: 0,
            temp_top_right: 0,
        }
    }
    pub fn create_group(&mut self, pos: GroupPosition) -> Group {
        if self.last_group_index == 255 {
            Group { pos, id: 255u8 }
        } else {
            self.last_group_index += 1;
            Group {
                pos,
                id: self.last_group_index,
            }
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
        let toolbar_ptr = unsafe { NonNull::new_unchecked(self as *mut ToolBar) };
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

    pub(crate) fn update_positions(&mut self, size: Size) -> (i32, i32) {
        // clear all flags (visible & left|right marker)
        let count = self.items.allocated_objects();
        for index in 0..count {
            if let Some(d) = self.items.get_element_mut(index) {
                d.get_base_mut().clear();
            }
        }
        let mut top_left = PositionHelper::new(1, 0);
        let mut top_right = PositionHelper::new((size.width as i32) - 1, 0);
        let mut bottom_left = PositionHelper::new(1, (size.height as i32) - 1);
        let mut bottom_right = PositionHelper::new((size.width as i32) - 1, (size.height as i32) - 1);

        for elem in &self.order {
            if let Some(d) = self.items.get_mut(elem.handle.cast()) {
                let is_resize_corner = d.is_resize_corner();
                let base = d.get_base_mut();
                if !base.is_visible() {
                    continue;
                }
                let pos = base.get_position();
                let (h, on_left) = match pos {
                    GroupPosition::TopLeft => (base.update_position_from_left(&mut top_left, top_right.x), true),
                    GroupPosition::BottomLeft => (base.update_position_from_left(&mut bottom_left, bottom_right.x), true),
                    GroupPosition::TopRight => (base.update_position_from_right(&mut top_right, top_left.x), false),
                    GroupPosition::BottomRight => {
                        if is_resize_corner {
                            bottom_right.x += 1;
                        }
                        (base.update_position_from_right(&mut bottom_right, bottom_left.x), false)
                    }
                };
                if !h.is_none() {
                    if let Some(prev) = self.items.get_mut(h.cast()) {
                        if on_left {
                            prev.get_base_mut().set_right_marker();
                        } else {
                            prev.get_base_mut().set_left_marker();
                        }
                    }
                }
            }
        }

        // last elements
        if !top_left.last_handle.is_none() {
            if let Some(item) = self.items.get_mut(top_left.last_handle.cast()) {
                item.get_base_mut().set_right_marker();
            }
        }
        if !bottom_left.last_handle.is_none() {
            if let Some(item) = self.items.get_mut(bottom_left.last_handle.cast()) {
                item.get_base_mut().set_right_marker();
            }
        }
        if !top_right.last_handle.is_none() {
            if let Some(item) = self.items.get_mut(top_right.last_handle.cast()) {
                item.get_base_mut().set_left_marker();
                if item.get_base().supports_markers() {
                    top_right.x -= 1;
                }
            }
        }
        if !bottom_right.last_handle.is_none() {
            if let Some(item) = self.items.get_mut(bottom_right.last_handle.cast()) {
                item.get_base_mut().set_left_marker();
                if item.get_base().supports_markers() {
                    bottom_right.x -= 1;
                }
            }
        }
        let left_pos = top_left.x + if top_left.last_handle.is_none() { 0 } else { 1 };
        let right_pos = top_right.x - 1;
        // for debug purposes
        self.temp_top_left = left_pos;
        self.temp_top_right = right_pos;
        (left_pos, right_pos)
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
        surface.write_char(self.temp_top_left, 0, Character::new('X', Color::Red, Color::Yellow, CharFlags::None));
        surface.write_char(self.temp_top_right, 0, Character::new('X', Color::Red, Color::Yellow, CharFlags::None));
    }

    pub(crate) fn update_singlechoice_group_id(&mut self, group_id: u32, handle: Handle<UIElement>) {
        let count = self.items.allocated_objects();
        // paint bar items
        for index in 0..count {
            if let Some(item) = self.items.get_element_mut(index) {
                if let ToolBarItem::SingleChoice(sc) = item {
                    if sc.get_group_id() == group_id {
                        sc.update_select_status(handle == sc.base.get_handle());
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
                    return Some(item.get_base().get_handle());
                }
            }
        }
        None
    }
}
