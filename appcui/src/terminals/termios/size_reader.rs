use std::sync::Arc;

use crate::{
    prelude::Size,
    terminals::{SystemEvent, SystemEventReader},
};

use super::api::sizing::ResizeNotification;

macro_rules! check_guard {
    ($guard: ident) => {
        if $guard.width > 0 {
            return Some(SystemEvent::Resize(*$guard));
        }
        *$guard = Size::default();
    };
}

pub(super) struct SizeReader {
    a: Arc<ResizeNotification>,
}

impl SizeReader {
    pub(super) fn new(a: Arc<ResizeNotification>) -> Self {
        Self { a }
    }
}

impl SystemEventReader for SizeReader {
    fn read(&mut self) -> Option<crate::terminals::SystemEvent> {
        let mut guard = self.a.mutex.lock().unwrap();

        check_guard!(guard);

        guard = self.a.cond_var.wait(guard).unwrap();

        check_guard!(guard);

        None
    }
}
