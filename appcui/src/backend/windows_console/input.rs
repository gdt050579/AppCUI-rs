use crate::system::SystemEvent;
use crate::backend::utils::win32;
use crate::backend::SystemEventReader;


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
