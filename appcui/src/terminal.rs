mod windows_terminal;
mod debug_terminal;
mod null_terminal;
mod system_event;

use super::graphics::Surface;
use super::graphics::Color;
use super::graphics::CharFlags;
use super::graphics::Size;


pub (crate) use self::system_event::SystemEvent;
pub (crate) use self::system_event::MouseButtonUpEvent;
pub (crate) use self::system_event::MouseButtonDownEvent;
pub (crate) use self::system_event::MouseDoubleClickEvent;
pub (crate) use self::system_event::MouseMoveEvent;
pub (crate) use self::system_event::MouseWheelEvent;
pub (crate) use self::system_event::KeyPressedEvent;


use self::windows_terminal::WindowsTerminal;
use self::debug_terminal::DebugTerminal;

pub (crate) trait Terminal {
    fn update_screen(&mut self, surface: &Surface);
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_system_event(&mut self)-> SystemEvent;
}

#[repr(u8)]
pub enum TerminalType {
    Debug,
    WindowsConsole,
}
impl TerminalType {
    pub (crate) fn new(terminal_type: TerminalType) -> Option<Box<dyn Terminal>>
    {
        match terminal_type {
            TerminalType::WindowsConsole => {
                if let Some(term) = WindowsTerminal::create() {
                    return Some(term);
                }
                return None;
            }
            TerminalType::Debug => {
                if let Some(term) = DebugTerminal::create() {
                    return Some(term);
                }
                return None;               
            }
        }
    }   
}





