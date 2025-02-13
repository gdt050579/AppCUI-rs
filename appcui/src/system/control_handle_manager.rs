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
            manager: HandleManager::with_capacity(64),
        }
    }
    #[inline(always)]
    pub(crate) fn get_mut(&mut self, handle: Handle<UIElement>) -> Option<&mut ControlManager> {
        self.manager.get_mut(handle.cast())
    }
    #[inline(always)]
    pub(crate) fn get(&self, handle: Handle<UIElement>) -> Option<&ControlManager> {
        self.manager.get(handle.cast())
    }
    #[inline(always)]
    pub(crate) fn desktop_mut(&mut self) -> &mut ControlManager {
        self.manager.element_mut(0).unwrap()
    }
    #[inline(always)]
    pub(crate) fn remove(&mut self, handle: Handle<UIElement>) -> bool {
        self.manager.remove(handle.cast())
    }
    pub(crate) fn clean_marked_for_focus(&mut self) {
        let max_count = self.manager.allocated_objects();
        for i in 0..max_count {
            if let Some(c) = self.manager.element_mut(i) {
                c.base_mut().clear_mark_to_receive_focus();
            }
        }
    }
    #[inline(always)]
    pub(crate) fn add(&mut self, manager: ControlManager) -> Handle<UIElement> {
        self.manager.add(manager).cast()
    }
}
