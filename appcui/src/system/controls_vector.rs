use crate::controls::{ControlManager, Handle};

pub(crate) struct ControlsVector {
    controls: Vec<Option<ControlManager>>,
}
impl ControlsVector {
    pub(crate) fn new() -> ControlsVector {
        Self {
            controls: Vec::with_capacity(64),
        }
    }
    pub(crate) fn get(&mut self, handle: Handle) -> Option<&mut ControlManager> {
        let idx = handle.get_index();
        if idx < self.controls.len() {
            let c = self.controls[idx].as_mut();
            if c.is_some() {
                if c.as_ref().unwrap().get_base().handle.unwrap() == handle {
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
    pub(crate) fn add(&mut self, mut manager: ControlManager) -> Handle {
        let idx = self.controls.len() as u32;
        let handle = Handle::new(idx);
        manager.get_base_mut().handle = Some(handle);
        // set the handle for all children
        for child in manager.get_base().children.iter() {
            if let Some(control) = self.get(*child) {
                control.get_base_mut().parent = Some(handle);
            }
        }
        self.controls.push(Some(manager));
        handle
    }
}
