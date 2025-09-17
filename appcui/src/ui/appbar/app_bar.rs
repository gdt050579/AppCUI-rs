use super::AppBarItem;
use crate::graphics::*;
use crate::input::*;
use crate::system::{Handle, RuntimeManager, Theme};
use crate::ui::appbar;
use crate::ui::appbar::ItemStatus;
use crate::ui::common::traits::EventProcessStatus;
use crate::utils::HandleManager;

macro_rules! const_cast {
    ($obj:expr, $from:ty, $to:ty) => {
        unsafe { &*($obj as *const $from as *const $to) }
    };
}
macro_rules! mut_cast {
    ($obj:expr, $from:ty, $to:ty) => {
        unsafe { &mut (*(($obj as *mut $from) as *mut $to)) }
    };
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct VisibleIndex {
    value: u32,
}
impl VisibleIndex {
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        Self { value: v as u32 }
    }
    #[inline(always)]
    fn index(&self) -> usize {
        self.value as usize
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum MousePos {
    None,
    NonClickable(VisibleIndex),
    Clickable(VisibleIndex),
}

#[derive(Copy, Clone)]
struct AppBarItemPos {
    idx: u32,
    x: i16,
    width: u8,
    order: u8,
}
pub struct AppBar {
    manager: HandleManager<AppBarItem>,
    shown_items: Vec<AppBarItemPos>,
    receiver_control_handle: Handle<()>,
    width: u32,
    mouse_pos: MousePos,
    current_item_index: Option<usize>,
    last_mouse_pos: Point,
}
impl AppBar {
    const LEFT_RIGHT_MIN_SPACE: i32 = 5;
    pub(crate) fn new(width: u32) -> Self {
        Self {
            manager: HandleManager::with_capacity(16),
            shown_items: Vec::with_capacity(64),
            receiver_control_handle: Handle::None,
            width,
            mouse_pos: MousePos::None,
            current_item_index: None,
            last_mouse_pos: Point::new(i32::MAX, i32::MAX),
        }
    }
    #[allow(private_bounds)]
    pub fn add<T>(&mut self, item: T) -> Handle<T>
    where
        T: Into<AppBarItem>,
    {
        self.manager.add(item.into()).cast()
    }
    #[allow(private_bounds)]
    pub fn get<T>(&self, menubaritem_hamdle: Handle<T>) -> Option<&T>
    where
        T: Into<AppBarItem>,
    {
        let ref_item = self.manager.get(menubaritem_hamdle.cast())?;
        Some(match ref_item {
            AppBarItem::Separator(_) => todo!(),
            AppBarItem::MenuButton(obj) => const_cast!(obj, appbar::MenuButton, T),
            AppBarItem::Label(_) => todo!(),
            AppBarItem::Button(_) => todo!(),
            AppBarItem::CheckBox(_) => todo!(),
        })
    }
    #[allow(private_bounds)]
    pub fn get_mut<T>(&mut self, menubaritem_hamdle: Handle<T>) -> Option<&mut T>
    where
        T: Into<AppBarItem>,
    {
        let ref_item = self.manager.get_mut(menubaritem_hamdle.cast())?;
        Some(match ref_item {
            AppBarItem::Separator(_) => todo!(),
            AppBarItem::MenuButton(obj) => mut_cast!(obj, appbar::MenuButton, T),
            AppBarItem::Label(_) => todo!(),
            AppBarItem::Button(_) => todo!(),
            AppBarItem::CheckBox(_) => todo!(),
        })
    }
    #[inline(always)]
    pub(crate) fn set_receiver_control_handle(&mut self, handle: Handle<()>) {
        self.receiver_control_handle = handle;
    }
    pub(crate) fn clear(&mut self) {
        self.shown_items.clear();
        self.receiver_control_handle = Handle::None;
    }
    pub(crate) fn close(&mut self) {
        self.current_item_index = None;
        self.mouse_pos = MousePos::None;
    }
    fn mouse_coord_to_mouse_pos(&self, x: i32, y: i32) -> MousePos {
        if y != 0 {
            return MousePos::None;
        }
        for (index, item) in self.shown_items.iter().enumerate() {
            let start = item.x as i32;
            let end = start + item.width as i32;
            if (x >= start) && (x < end) {
                if let Some(elem) = self.manager.element(item.idx as usize) {
                    let base = elem.base();
                    return if base.is_enabled() {
                        if base.accepts_input() {
                            MousePos::Clickable(VisibleIndex { value: index as u32 })
                        } else {
                            MousePos::NonClickable(VisibleIndex { value: index as u32 })
                        }
                    } else {
                        // disabled
                        MousePos::None
                    };
                } else {
                    return MousePos::None;
                }
            }
        }
        MousePos::None
    }
    pub(crate) fn update_positions(&mut self) {
        // sort the data first
        self.shown_items.sort_by_key(|i| i.order);
        let mut left = 0i32;
        let mut right = self.width as i32;
        let mut index = 0;
        for item in &mut self.shown_items {
            let idx = item.idx as usize;
            if let Some(obj) = self.manager.element_mut(idx) {
                // refresh the width
                let w = obj.base().width();
                item.width = w;

                let onleft = obj.base().is_on_left();
                if onleft {
                    let next = left + w as i32;
                    if next + AppBar::LEFT_RIGHT_MIN_SPACE >= right {
                        break;
                    }
                    obj.base_mut().set_x(left);
                    item.x = left as i16;
                    left += w as i32;
                } else {
                    right -= w as i32;
                    if right - AppBar::LEFT_RIGHT_MIN_SPACE <= left {
                        break;
                    }
                    item.x = right as i16;
                    obj.base_mut().set_x(right);
                }
            }
            index += 1;
        }
        self.shown_items.truncate(index);
        self.update_mouse_pos();
    }
    #[inline(always)]
    pub(crate) fn is_opened(&self) -> bool {
        self.current_item_index.is_some()
    }
    fn change_current_item(&mut self, goto_next: bool) {
        if self.shown_items.is_empty() {
            self.current_item_index = None;
            return;
        }
        if let Some(value) = self.current_item_index {
            let len = self.shown_items.len();
            let new_index = if goto_next { (value + 1) % len } else { (value + len - 1) % len };
            self.open(VisibleIndex::from_usize(new_index))
        }
    }
    fn open(&mut self, shown_index: VisibleIndex) {
        if let Some(elem) = self.item_mut(shown_index) {
            let base = elem.base();
            if base.is_enabled() && base.accepts_input() {
                elem.activate();
                self.current_item_index = Some(shown_index.index());
                return;
            }
        }
        self.current_item_index = None;
    }
    #[inline(always)]
    fn item(&self, shown_index: VisibleIndex) -> Option<&AppBarItem> {
        self.shown_items
            .get(shown_index.index())
            .map(|item_pos| self.manager.element(item_pos.idx as usize))?
    }
    #[inline(always)]
    fn item_mut(&mut self, shown_index: VisibleIndex) -> Option<&mut AppBarItem> {
        self.shown_items
            .get_mut(shown_index.index())
            .map(|item_pos| self.manager.element_mut(item_pos.idx as usize))?
    }
    fn show_tooltip(&mut self, shown_index: VisibleIndex) {
        if let Some(elem) = self.item(shown_index) {
        } else {
        }
    }
    fn update_mouse_pos(&mut self) -> bool {
        let new_mouse_pos = self.mouse_coord_to_mouse_pos(self.last_mouse_pos.x, self.last_mouse_pos.y);
        if new_mouse_pos == self.mouse_pos {
            // nothing to update
            return false;
        }
        self.mouse_pos = new_mouse_pos;
        match new_mouse_pos {
            MousePos::None => {}
            MousePos::NonClickable(shown_index) => {
                self.show_tooltip(shown_index);
            }
            MousePos::Clickable(shown_index) => {
                // show tooltip only if not opened
                if self.current_item_index.is_some() {
                    self.open(shown_index);
                } else {
                    self.show_tooltip(shown_index);
                }
            }
        }
        true
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.fill_horizontal_line_with_size(0, 0, self.width, Character::with_attributes(' ', theme.menu.text.normal));
        let hover_index = match self.mouse_pos {
            MousePos::Clickable(vi) => vi.index(),
            _ => usize::MAX,
        };
        let current_index = self.current_item_index.unwrap_or(usize::MAX);
        for (index, item) in self.shown_items.iter().enumerate() {
            if let Some(elem) = self.manager.element(item.idx as usize) {
                let status = if !elem.is_enabled() {
                    ItemStatus::Inactive
                } else if index == current_index {
                    ItemStatus::Current
                } else if index == hover_index {
                    ItemStatus::Hovered
                } else {
                    ItemStatus::Normal
                };
                elem.paint(surface, theme, status);
            }
        }
    }
    pub(crate) fn on_mouse_pressed(&mut self, x: i32, y: i32) -> EventProcessStatus {
        self.last_mouse_pos.x = x;
        self.last_mouse_pos.y = y;
        let mut res = self.update_mouse_pos();
        match self.mouse_pos {
            MousePos::Clickable(index) => {
                self.open(index);
                res = true;
            }
            _ => {}
        }
        if res {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
    pub(crate) fn on_mouse_move(&mut self, x: i32, y: i32) -> EventProcessStatus {
        self.last_mouse_pos.x = x;
        self.last_mouse_pos.y = y;
        if self.update_mouse_pos() {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
    fn process_shortcut(&mut self, key: Key) -> bool {
        for (index, pos) in self.shown_items.iter().enumerate() {
            if let Some(elem) = self.manager.element(pos.idx as usize) {
                if elem.is_enabled() && (key == elem.hotkey()) {
                    self.open(VisibleIndex::from_usize(index));
                    return true;
                }
            }
        }
        false
    }
    fn process_key(&mut self, key: Key) -> EventProcessStatus {
        match key.code {
            KeyCode::Left => {
                self.change_current_item(false);
                EventProcessStatus::Processed
            }
            KeyCode::Right => {
                self.change_current_item(true);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }

    pub(crate) fn on_key_event(&mut self, key: Key, menu_is_opened: bool) -> EventProcessStatus {
        if menu_is_opened {
            if key.modifier.is_empty() {
                return self.process_key(key);
            }
            if key.modifier == KeyModifier::Alt {
                // check if a shortcut was pressed
                if self.process_shortcut(key) {
                    return EventProcessStatus::Processed;
                }
            }
            EventProcessStatus::Ignored
        } else {
            if key.modifier == KeyModifier::Alt {
                // check if a shortcut was pressed
                if self.process_shortcut(key) {
                    return EventProcessStatus::Processed;
                }
            }
            // else check all shortcuts
            let menus = RuntimeManager::get().get_menus();
            for pos in &self.shown_items {
                if let Some(elem) = self.manager.element(pos.idx as usize) {
                    if elem.process_shortcut(key, menus) {
                        return EventProcessStatus::Processed;
                    }
                }
            }
            // for (index, item) in self.items.iter().enumerate() {
            //     if index >= self.count {
            //         break;
            //     }
            //     if let Some(menu) = menus.get_mut(item.handle) {
            //         if menu.process_shortcut(key, item.receiver_control_handle) {
            //             return EventProcessStatus::Processed;
            //         }
            //     }
            // }

            // nothing to process
            EventProcessStatus::Ignored
        }
    }
    pub(crate) fn update_width(&mut self, width: u32) {
        if width != self.width {
            self.width = width;
            self.update_positions();
        }
    }
    #[allow(private_bounds)]
    pub fn show<T>(&mut self, handle: Handle<T>)
    where
        T: Into<AppBarItem>,
    {
        if self.receiver_control_handle.is_none() {
            return;
        }
        if let Some(item) = self.manager.get_mut(handle.cast()) {
            item.set_receiver_control_handle(self.receiver_control_handle);
            let base = item.base();
            self.shown_items.push(AppBarItemPos {
                idx: handle.index() as u32,
                x: base.x() as i16,
                width: base.width(),
                order: base.order(),
            });
        }
    }
}
