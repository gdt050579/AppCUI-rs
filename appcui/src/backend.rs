//! # Backends
//!
//! This module contains the different terminal implementations.
//! The terminal is responsible for rendering the UI and handling user input.
//!
//! The terminal is not created by the user, it is created by the system based on some parameters that are provided when an Application is being initialized.
//! ## Terminal Support and Capabilities
//!
//! AppCUI supports multiple terminal implementations with varying capabilities across different operating systems:
//!
//! - **Windows Console**: Native Windows terminal with full keyboard/mouse support
//! - **NCurses**: Unix-based terminal with good display and input capabilities
//! - **Termios**: Basic Unix terminal with limited features
//!
//! ### Default Backends by OS:
//! - Windows: Windows Console
//! - Linux: NCurses
//! - Mac/OSX: Termios
//!
//! ### Key Capabilities:
//!
//! #### Display
//! - 16 foreground and background colors across all terminals
//! - Unicode support (Windows Console: UTF-16, NCurses: UTF-8)
//! - Cursor control and underline support (Windows/NCurses)
//! - Bold text support (NCurses only)
//!
//! #### Input Handling
//! - Keyboard: Full modifier support in Windows (Alt/Ctrl/Shift combinations)
//! - Mouse: Click, move, drag and wheel support (Windows/NCurses)
//! - System: Console resize events (Windows/NCurses)
//!
//! #### Additional Features
//! - Clipboard support via native APIs (Windows) or copypasta crate (NCurses)
//! - Window title control (Windows only)
//! - Console dimension control (Windows/NCurses)
//!
//! Each terminal implementation provides these capabilities through the Terminal trait,
//! allowing AppCUI to work consistently across different platforms while leveraging
//! platform-specific features when available.

mod debug;
#[cfg(target_os = "linux")]
mod ncurses;
mod system_event_thread;
#[cfg(target_family = "unix")]
mod termios;
#[cfg(target_arch = "wasm32")]
mod web_terminal;
#[cfg(target_os = "windows")]
mod windows_console;
#[cfg(target_os = "windows")]
mod windows_vt;


pub(crate) mod utils;


#[cfg(test)]
mod tests;

use std::sync::mpsc::Sender;

use super::graphics::Size;
use super::graphics::Surface;
use super::system::Error;
use super::system::ErrorKind;
use super::system::SystemEvent;

#[cfg(not(target_arch = "wasm32"))]
pub(super) use self::system_event_thread::SystemEventReader;

use self::debug::DebugTerminal;

#[cfg(target_os = "linux")]
use self::ncurses::NcursesTerminal;
#[cfg(target_family = "unix")]
use self::termios::TermiosTerminal;
#[cfg(target_arch = "wasm32")]
use self::web_terminal::WebTerminal;
#[cfg(target_os = "windows")]
use self::windows_console::WindowsConsoleTerminal;
#[cfg(target_os = "windows")]
use self::windows_vt::WindowsVTTerminal;

pub(crate) trait Backend {
    fn update_screen(&mut self, surface: &Surface);
    fn on_resize(&mut self, new_size: Size);
    fn size(&self) -> Size;
    fn clipboard_text(&self) -> Option<String>;
    fn set_clipboard_text(&mut self, text: &str);
    fn has_clipboard_text(&self) -> bool;
    fn query_system_event(&mut self) -> Option<SystemEvent> {
        None
    }
    fn is_single_threaded(&self) -> bool;
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Type {
    #[cfg(target_os = "windows")]
    WindowsConsole,
    #[cfg(target_os = "windows")]
    WindowsVT,
    #[cfg(target_family = "unix")]
    Termios,
    #[cfg(target_os = "linux")]
    NcursesTerminal,
    #[cfg(target_arch = "wasm32")]
    WebTerminal,
}

pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
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
        let term = DebugTerminal::new(builder)?;
        return Ok(Box::new(term));
    }
    // if no terminal is provided --> consider the default terminal (best approach)
    // this depends on the OS
    if builder.terminal.is_none() {
        // based on OS we should choose a terminal
        return build_default_terminal(builder, sender);
    }
    // finaly, based on the type, return a terminal
    let terminal = *builder.terminal.as_ref().unwrap();
    match terminal {
        #[cfg(target_os = "windows")]
        Type::WindowsConsole => {
            let term = WindowsConsoleTerminal::new(builder, sender)?;
            Ok(Box::new(term))
        }
        #[cfg(target_os = "windows")]
        Type::WindowsVT => {
            let term = WindowsVTTerminal::new(builder, sender)?;
            Ok(Box::new(term))
        }
        #[cfg(target_family = "unix")]
        Type::Termios => TermiosTerminal::new(builder, sender),

        #[cfg(target_os = "linux")]
        Type::NcursesTerminal => {
            let term = NcursesTerminal::new(builder, sender)?;
            Ok(Box::new(term))
        }

        #[cfg(target_arch = "wasm32")]
        Type::WebTerminal => {
            let term = WebTerminal::new(builder, sender)?;
            return Ok(Box::new(term));
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn build_default_terminal(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    let term = WebTerminal::new(builder, sender)?;
    Ok(Box::new(term))
}

#[cfg(target_os = "windows")]
fn build_default_terminal(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    let term = WindowsConsoleTerminal::new(builder, sender)?;
    Ok(Box::new(term))
}
#[cfg(target_os = "linux")]
fn build_default_terminal(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    // TermiosTerminal::new(builder)
    let term = NcursesTerminal::new(builder, sender)?;
    Ok(Box::new(term))
}
#[cfg(target_os = "macos")]
fn build_default_terminal(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    TermiosTerminal::new(builder, sender)
}
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos", target_arch = "wasm32")))]
fn build_default_terminal(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    // anything else
    TermiosTerminal::new(builder, sender)
}
