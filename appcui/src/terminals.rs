pub(crate) mod ansi;
mod debug;
mod system_event;
#[cfg(target_os = "windows")]
mod windows_console;

use super::graphics::Size;
use super::graphics::Surface;
use super::system::Error;
use super::system::ErrorKind;

use self::ansi::AnsiTerminal;
pub(crate) use self::system_event::KeyPressedEvent;
pub(crate) use self::system_event::MouseButtonDownEvent;
pub(crate) use self::system_event::MouseButtonUpEvent;
pub(crate) use self::system_event::MouseDoubleClickEvent;
pub(crate) use self::system_event::MouseMoveEvent;
pub(crate) use self::system_event::MouseWheelEvent;
pub(crate) use self::system_event::SystemEvent;

use self::debug::DebugTerminal;

#[cfg(target_os = "windows")]
use self::windows_console::WindowsTerminal;

pub(crate) trait Terminal {
    fn update_screen(&mut self, surface: &Surface);
    fn get_size(&self) -> Size;
    fn get_system_event(&mut self) -> SystemEvent;
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TerminalType {
    #[cfg(target_os = "windows")]
    WindowsConsole,
    ANSI,
}
impl TerminalType {
    pub(crate) fn new(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        // check if terminal size if valid (if present)
        if let Some(sz) = builder.size.as_ref() {
            if (sz.width == 0) || (sz.height == 0) {
                return Err(Error::new(
                    ErrorKind::InvalidParameter,
                    format!(
                        "Invalid size for a terminal ({}x{}). Both width and height must be bigger than 0 !",
                        sz.width, sz.height
                    ),
                ));
            }
        }
        // check if we have a debug script present --> if so ... we will create a Debug terminal
        if builder.debug_script.is_some() {
            return DebugTerminal::new(builder);
        }
        // if no terminal is provided --> consider the default terminal (best approach)
        // this depends on the OS
        if builder.terminal.is_none() {
            // based on OS we should choose a terminal
            return TerminalType::build_default_terminal(builder);
        }
        // finaly, based on the type, return a terminal
        let terminal = *builder.terminal.as_ref().unwrap();
        match terminal {
            #[cfg(target_os = "windows")]
            TerminalType::WindowsConsole => WindowsTerminal::new(builder),
            TerminalType::ANSI => AnsiTerminal::new(builder),
        }
    }
    #[cfg(target_os = "windows")]
    fn build_default_terminal(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        WindowsTerminal::new(builder)
    }
    #[cfg(target_os = "linux")]
    fn build_default_terminal(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        AnsiTerminal::new(builder)
    }
    #[cfg(target_os = "macos")]
    fn build_default_terminal(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        AnsiTerminal::new(builder)
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    fn build_default_terminal(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        // anything else
        AnsiTerminal::new(builder)
    }
}
