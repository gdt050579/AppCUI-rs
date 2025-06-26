use crate::system::SystemEvent;
use crate::backend::SystemEventReader;


pub(crate) struct Input {}
    

impl Input {
    pub(super) fn new() -> Self {
        todo!()
    }
}

impl SystemEventReader for Input {
    fn read(&mut self) -> Option<SystemEvent> {
        todo!()
    }
}
