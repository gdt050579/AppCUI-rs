use crate::{
    ui::common::{ControlManager, UIElement},
    utils::HandleManager,
};

use super::Handle;

pub(crate) struct ControlHandleManager {
    manager: HandleManager<ControlManager>,
}
impl ControlHandleManager {
    pub(crate) fn new() -> Self {
        Self {
            manager: HandleManager::new(64),
        }
    }
    #[inline(always)]
    pub(crate) fn get_mut(&mut self, menu_handle: Handle<UIElement>) -> Option<&mut ControlManager> {
        self.manager.get_mut(menu_handle.cast())
    }
    #[inline(always)]
    pub(crate) fn get(&self, menu_handle: Handle<UIElement>) -> Option<&ControlManager> {
        self.manager.get(menu_handle.cast())
    }
    #[inline(always)]
    pub(crate) fn get_desktop(&mut self) -> &mut ControlManager {
        return self.manager.get_element_mut(0).unwrap();
    }
    pub(crate) fn clean_marked_for_focus(&mut self) {
        let max_count = self.manager.allocated_objects();
        for i in 0..max_count {
            if let Some(c) = self.manager.get_element_mut(i) {
                c.get_base_mut().clear_mark_to_receive_focus();
            }
        }
    }
    #[inline(always)]
    pub(crate) fn add(&mut self, manager: ControlManager) -> Handle<UIElement> {
        self.manager.add(manager).cast()
    }
}

/*
pub(crate) struct ControlHandleManager {
    controls: Vec<Option<ControlManager>>,
}
impl ControlHandleManager {
    pub(crate) fn new() -> ControlHandleManager {
        Self {
            controls: Vec::with_capacity(64),
        }
    }
    pub(crate) fn get_mut(&mut self, handle: Handle<UIElement>) -> Option<&mut ControlManager> {
        let idx = handle.get_index();
        if idx < self.controls.len() {
            let c = self.controls[idx].as_mut();
            if c.is_some() {
                if c.as_ref().unwrap().get_base().handle == handle {
                    return c;
                }
            }
        }
        None
    }
    pub(crate) fn get(&self, handle: Handle<UIElement>) -> Option<&ControlManager> {
        let idx = handle.get_index();
        if idx < self.controls.len() {
            let c = self.controls[idx].as_ref();
            if c.is_some() {
                if c.as_ref().unwrap().get_base().handle == handle {
                    return c;
                }
            }
        }
        None
    }



    #[inline(always)]
    pub(crate) fn get_desktop(&mut self) -> &mut ControlManager {
        return self.controls[0].as_mut().unwrap();
    }
    pub(crate) fn add(&mut self, mut manager: ControlManager) -> Handle<UIElement> {
        let idx = self.controls.len() as u32;
        let handle = Handle::new(idx);
        manager.get_base_mut().handle = handle;
        // set the handle for all children
        for child in manager.get_base().children.iter() {
            if let Some(control) = self.get_mut(*child) {
                control.get_base_mut().parent = handle;
            }
        }
        self.controls.push(Some(manager));
        handle
    }
    pub(crate) fn clean_marked_for_focus(&mut self) {
        for c in self.controls.iter_mut() {
            if let Some(control) = c {
                control.get_base_mut().clear_mark_to_receive_focus();
            }
        }
    }
}
*/
