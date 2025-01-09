use crate::system::{Handle, RuntimeManager};
use super::Timer;

pub(crate) struct TimerManager {
    slots: Vec<Option<Timer>>,
    last_id: u32,
}

impl TimerManager {
    pub(crate) fn new(capacity: u8) -> Self {
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
            self.slots[handle.index()].as_mut().filter(|timer| timer.handle() == handle)
        } else {
            None
        }
    }
    pub(crate) fn update_control_handle(&mut self, handle: Handle<Timer>, control_handle: Handle<()>) {
        if let Some(timer) = self.get_mut(handle) {
            timer.update_control_handle(control_handle);
        }
    }
    pub(crate) fn update_threads(&mut self) {

        let rm = RuntimeManager::get();
        for slot in self.slots.iter_mut() {
            if let Some(timer) = slot {
                if timer.is_ready() {
                    // we need to start a new timer
                    timer.start_thread(rm.get_system_event_sender());
                } else if timer.is_closed() {
                    // timer is about to close (empty the slot)
                    *slot = None;
                }
            }
        }
    }
    pub(crate) fn control_handle(&self, index: u8) -> Handle<()> {
        if (index as usize) < self.slots.len() {
            if let Some(timer) = &self.slots[index as usize] {
                return timer.control_handle();
            }
        }
        Handle::None
    }
    pub(crate) fn index_mut(&mut self, index: u8) -> Option<&mut Timer> {
        if (index as usize) < self.slots.len() {
            if let Some(timer) = &mut self.slots[index as usize] {
                Some(timer)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub(crate) fn terminate_thread(&mut self, index: usize) {
        if index < self.slots.len() {
            if let Some(timer) = &mut self.slots[index] {
                timer.stop();
            }
            self.slots[index] = None;
        } 
    }
}
