use super::component_toolbar_item::ComponentToolbarItem;
use super::Component;
use super::ProcessEventResult;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::ui::ControlBase;
use crate::utils::HandleManager;

pub struct ComponentsToolbar {
    items: HandleManager<ComponentToolbarItem>,
}
impl ComponentsToolbar {
    pub fn add<T>(&mut self, item: T) -> Handle<T>
    where
        T: Component,
    {
        // self.items.
        // let h = AddToToolbar::add(item, self, group);
        // self.order.push(ToolbarElementHandle { group, handle: h.cast() });
        // self.order.sort_by_key(|k| k.group.id);
        // h
        Handle::None
    }
    pub fn get<T>(&self, handle: Handle<T>) -> Option<&T> {
        if let Some(obj) = self.items.get(handle.cast()) {
            match obj {
                ComponentToolbarItem::ScrollBar(obj) => return Some(unsafe { &(*((obj as *const super::ScrollBar) as *const T)) }),
            }
        }
        None
    }
    pub fn get_mut<T>(&mut self, handle: Handle<T>) -> Option<&mut T> {
        if let Some(obj) = self.items.get_mut(handle.cast()) {
            match obj {
                ComponentToolbarItem::ScrollBar(obj) => return Some(unsafe { &mut (*((obj as *mut super::ScrollBar) as *mut T)) }),
            }
        }
        None
    }

    pub fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        let count = self.items.allocated_objects();
        for index in 0..count {
            if let Some(item) = self.items.get_element(index) {
                item.paint(surface, theme, control);
            }
        }
    }
    pub fn on_mouse_event(&mut self, event: &MouseEvent) -> ProcessEventResult {
        let mut res = ProcessEventResult::PassToControl;
        let count = self.items.allocated_objects();
        for index in 0..count {
            if let Some(item) = self.items.get_element_mut(index) {
                res |= item.on_mouse_event(event);
            }
        }
        res
    }
    pub fn resize(&mut self, new_size: Size, left_margin: i32, top_margin: i32) {
        let mut w = (new_size.width as i32) - (left_margin + 2); // 2 = space from right
        let mut h = (new_size.height as i32) - (top_margin + 1); // 1 = space from bottom
        let mut x = left_margin;
        let mut y = top_margin;
        let count = self.items.allocated_objects();
        for index in 0..count {
            if let Some(item) = self.items.get_element_mut(index) {
                let vertical = item.is_vertical();
                // logic
            }
        }
    }
}
