use super::super::SystemEvent;
use crate::terminals::utils::win32;
use crate::terminals::SystemEventReader;


pub(crate) struct Input {
    console: win32::Console,
}

impl Input {
    pub(super) fn new(console: win32::Console) -> Self {
        Self { console }
    }
}

impl SystemEventReader for Input {
    fn read(&mut self) -> Option<SystemEvent> {
        self.console.read_event()
    }
}
