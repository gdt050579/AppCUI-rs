use super::Terminal;
use super::Surface;

struct DebugTerminal {

}
impl Terminal for DebugTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        todo!()
    }

    fn get_width(&self) -> u32 {
        todo!()
    }

    fn get_height(&self) -> u32 {
        todo!()
    }

    fn get_system_event(&mut self)-> super::SystemEvent {
        todo!()
    }
}