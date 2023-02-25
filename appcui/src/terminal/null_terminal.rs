use super::SystemEvent;
use super::Terminal;

pub(crate) struct NullTerminal {}
impl Terminal for NullTerminal {
    fn update_screen(&mut self, surface: &crate::graphics::Surface) {}

    fn get_width(&self) -> u32 {
        0
    }

    fn get_height(&self) -> u32 {
        0
    }

    fn get_system_event(&mut self) -> SystemEvent {
        SystemEvent::None
    }
}
