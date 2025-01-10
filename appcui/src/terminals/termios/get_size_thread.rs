use std::sync::Arc;

use crate::{prelude::Size, terminals::{SystemEvent, SystemEventReader}};

use super::api::sizing::ResizeNotification;


pub(super) struct GetSizeThread {
    a: Arc<ResizeNotification>
}

impl GetSizeThread {
    pub(super) fn new(a: Arc<ResizeNotification>) -> Self {
        Self{a}
    }
}

impl SystemEventReader for GetSizeThread {
    fn read(&mut self) -> Option<crate::terminals::SystemEvent> {
        let mut guard = self.a.mutex.lock().unwrap();
        
        if (*guard).width > 0 {
            return Some(SystemEvent::Resize(*guard));
        }
        *guard = Size::default();

        guard = self.a.cond_var.wait(guard).unwrap();

        if (*guard).width > 0 {
            return Some(SystemEvent::Resize(*guard));
        }
        *guard = Size::default();

        return None;
    }
}
