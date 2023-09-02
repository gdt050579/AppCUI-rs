mod debug;
mod system_event;
mod windows_console;

use super::graphics::CharFlags;
use super::graphics::Color;
use super::graphics::Size;
use super::graphics::Surface;
use super::system::Error;

pub(crate) use self::system_event::KeyPressedEvent;
pub(crate) use self::system_event::MouseButtonDownEvent;
pub(crate) use self::system_event::MouseButtonUpEvent;
pub(crate) use self::system_event::MouseDoubleClickEvent;
pub(crate) use self::system_event::MouseMoveEvent;
pub(crate) use self::system_event::MouseWheelEvent;
pub(crate) use self::system_event::SystemEvent;

use self::debug::DebugTerminal;
use self::windows_console::WindowsTerminal;

pub(crate) trait Terminal {
    fn update_screen(&mut self, surface: &Surface);
    fn get_size(&self) -> Size;
    fn get_system_event(&mut self) -> SystemEvent;
}

#[repr(u8)]
#[derive(Copy,Clone)]
pub enum TerminalType {
    WindowsConsole,
}
impl TerminalType {
    pub(crate) fn new(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        // check if we have a debug script present --> if so ... we will create a Debug terminal
        if builder.debug_script.is_some() {
            return DebugTerminal::new(builder);
        }
        // if no terminal is provided --> consider the default terminal (best approach)
        // this depends on the OS
        if builder.terminal.is_none() {
            // based on OS we should choose a terminal
            return WindowsTerminal::new(builder);
        }
        // finaly, based on the type, return a terminal
        let terminal = *builder.terminal.as_ref().unwrap();
        match terminal {
            TerminalType::WindowsConsole => WindowsTerminal::new(builder),
        }
    }
}
