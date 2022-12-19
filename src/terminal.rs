mod windows;

use super::graphics::Surface;
use super::graphics::Color;
use super::graphics::Attribute;
use self::windows::WindowsTerminal;

pub trait Terminal {
    fn update_screen(&mut self, surface: &Surface);
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
}

#[repr(u8)]
pub enum TerminalType {
    WindowsConsole,
}
impl TerminalType {
    pub fn new(terminal_type: TerminalType) -> Option<Box<dyn Terminal>>
    {
        match terminal_type {
            TerminalType::WindowsConsole => {
                if let Some(term) = WindowsTerminal::create() {
                    return Some(term);
                }
                return None;
            }
        }
    }   
}





