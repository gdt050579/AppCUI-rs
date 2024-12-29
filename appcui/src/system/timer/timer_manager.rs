use crate::{system::Handle, ui::common::UIElement};

use super::Timer;

pub(crate) struct TimerManager {
    slots: Vec<Option<Timer>>,
    last_id: u32,
}

impl TimerManager {
    pub(crate) fn new(capacity: u32) -> Self {
        let mut r = Self {
            slots: Vec::with_capacity(capacity as usize),
            last_id: 1,
        };
        for _ in 0..capacity {
            r.slots.push(None);
        }
        r
    }
    pub(crate) fn allocate_for(&mut self, control_handle: Handle<()>) -> Handle<Timer> {
        // find first empty slot
        for (index, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                let handle: Handle<Timer> = Handle::with_id(self.last_id, index as u32);
                *slot = Some(Timer::new(control_handle, handle));
                self.last_id += 1;
                return handle;
            }
        }
        Handle::None
    }
    pub(crate) fn get_mut(&mut self, handle: Handle<Timer>) -> Option<&mut Timer> {
        if handle.index() < self.slots.len() {
            if let Some(timer) = &mut self.slots[handle.index()] {
                if timer.handle() == handle {
                    Some(timer)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    pub(crate) fn update_control_handle(&mut self, handle: Handle<Timer>, control_handle: Handle<()>) {
        if let Some(timer) = self.get_mut(handle) {
            timer.update_control_handle(control_handle);
        }
    }
}
