use crate::terminals::{SystemEvent, Terminal};
use crate::graphics::*;

pub(super) struct EventRecorder {
    events: Vec<i32>,   
}
impl EventRecorder {
    pub(super) fn new()->Self {
        Self {
            events: Vec::with_capacity(512),
        }
    }
    pub(super) fn save() {

    }
    pub(super) fn add(&mut self, sys_event: &SystemEvent, terminal: &mut Box<dyn Terminal>, surface: &Surface) {
        
    }
}