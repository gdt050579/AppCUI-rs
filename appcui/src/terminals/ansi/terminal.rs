use super::super::SystemEvent;
use super::super::Terminal;
use crate::system::Error;
use crate::graphics::*;


pub struct AnsiTerminal {

}
impl AnsiTerminal {
    pub(crate) fn new(_builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        todo!();
    }
}
impl Terminal for AnsiTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        todo!()
    }

    fn get_size(&self) -> Size {
        todo!()
    }

    fn get_system_event(&mut self) -> SystemEvent {
        todo!()
    }
}