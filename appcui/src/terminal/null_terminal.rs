use super::SystemEvent;
use super::Terminal;
use crate::graphics::*;

pub(crate) struct NullTerminal {}
impl Terminal for NullTerminal {
    fn update_screen(&mut self, _surface: &Surface) {}

    fn get_width(&self) -> u32 {
        0
    }

    fn get_height(&self) -> u32 {
        0
    }
    fn on_resize(&mut self, _new_size: Size) {}
    fn get_system_event(&mut self) -> SystemEvent {
        SystemEvent::None
    }
}
