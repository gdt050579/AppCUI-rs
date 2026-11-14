//! # Backends
//!
//! This module contains the different backends implementations.
//! The backend is responsible for rendering the UI and handling user input.
//!
//! The backend is not created by the user, it is created by the system based on some parameters that are provided when an Application is being initialized.
//!
//! ## Backends
//!
//! - **Windows Console**: native Windows console API. Default on Windows.
//! - **Windows VT**: Windows console driven by VT / ANSI sequences.
//! - **Termios**: termios raw mode and ANSI sequences. Default on Linux and macOS.
//! - **NCurses**: ncurses. Unix only, and only when the `NCURSES` feature is enabled.
//! - **CrossTerm**: the `crossterm` crate. Only when the `CROSSTERM` feature is enabled.
//! - **Web Terminal**: browser canvas. Default on WebAssembly.
//!
//! NCurses and CrossTerm are never selected automatically. What each backend can display and which input it can read depends on the terminal.

mod debug;
#[cfg(all(target_family = "unix", feature = "NCURSES"))]
mod ncurses;
mod system_event_thread;
// #[cfg(target_family = "unix")]
// mod ncurses;
#[cfg(target_family = "unix")]
mod termios;
#[cfg(target_arch = "wasm32")]
mod web_terminal;
#[cfg(target_os = "windows")]
mod windows_console;
#[cfg(target_os = "windows")]
mod windows_vt;
//#[cfg(any(target_family = "unix", feature = "CROSSTERM"))]
#[cfg(feature = "CROSSTERM")]
mod crossterm;

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

#[cfg(all(target_family = "unix", feature = "NCURSES"))]
use self::ncurses::NcursesTerminal;
#[cfg(target_family = "unix")]
use self::termios::TermiosTerminal;
#[cfg(target_arch = "wasm32")]
use self::web_terminal::WebTerminal;
#[cfg(target_os = "windows")]
use self::windows_console::WindowsConsoleTerminal;
#[cfg(target_os = "windows")]
use self::windows_vt::WindowsVTTerminal;
//#[cfg(any(target_family = "unix", feature = "CROSSTERM"))]
#[cfg(feature = "CROSSTERM")]
use self::crossterm::CrossTerm;

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
    fn on_close(&mut self) {}
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
/// Terminal backend used to render the UI and read input.
///
/// Pass a variant to [`crate::system::MultiWindowAppBuilder::backend`]. If omitted,
/// AppCUI picks a default for the current OS:
///
/// | OS | Default |
/// |----|---------|
/// | Windows | Windows Console |
/// | Linux | Termios |
/// | macOS | Termios |
/// | WASM | Web Terminal |
///
/// Variants are gated by target OS and crate features, so only backends compiled
/// for the current platform appear in this enum.
///
/// # Examples
///
/// ```rust, no_run
/// use appcui::prelude::*;
///
/// #[cfg(target_os = "windows")]
/// let _builder = App::new().backend(appcui::backend::Type::WindowsVT);
/// #[cfg(not(target_os = "windows"))]
/// let _builder = App::new();
/// ```
pub enum Type {
    /// Native Windows Console API (default on Windows). Fast; typically 16 colors.
    #[cfg(target_os = "windows")]
    WindowsConsole,
    /// Windows console driven by VT100 / ANSI sequences. Supports true color; slower than the Console API.
    #[cfg(target_os = "windows")]
    WindowsVT,
    /// Low-level Unix termios / raw mode and ANSI sequences (default on Linux and macOS).
    #[cfg(target_family = "unix")]
    Termios,
    /// Unix ncurses terminal. Enable the `NCURSES` crate feature. Not selected by default.
    #[cfg(all(target_family = "unix", feature = "NCURSES"))]
    NcursesTerminal,
    /// Browser canvas terminal (default on WASM).
    #[cfg(target_arch = "wasm32")]
    WebTerminal,
    /// [`crossterm`](https://docs.rs/crossterm) backend. Enable the `CROSSTERM` crate feature. Not selected by default.
    //#[cfg(any(target_family = "unix", feature = "CROSSTERM"))]
    #[cfg(feature = "CROSSTERM")]
    CrossTerm,
}

pub(crate) fn new(builder: &crate::system::InternalBuilder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    // check if backend size if valid (if present)
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
    // check if we have a debug script present --> if so ... we will create a Debug backend
    if builder.debug_script.is_some() {
        let term = DebugTerminal::new(builder)?;
        return Ok(Box::new(term));
    }
    // if no backend is provided --> consider the default backend (best approach)
    // this depends on the OS
    if builder.backend.is_none() {
        // based on OS we should choose a backend
        return build_default_backend(builder, sender);
    }
    // finaly, based on the type, return a terminal
    let backend = *builder.backend.as_ref().unwrap();
    match backend {
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
        Type::Termios => {
            let term = TermiosTerminal::new(builder, sender)?;
            Ok(Box::new(term))
        }

        #[cfg(all(target_family = "unix", feature = "NCURSES"))]
        Type::NcursesTerminal => {
            let term = NcursesTerminal::new(builder, sender)?;
            Ok(Box::new(term))
        }

        #[cfg(target_arch = "wasm32")]
        Type::WebTerminal => {
            let term = WebTerminal::new(builder, sender)?;
            Ok(Box::new(term))
        }

        // #[cfg(any(target_family = "unix", feature = "CROSSTERM"))]
        #[cfg(feature = "CROSSTERM")]
        Type::CrossTerm => {
            let term = CrossTerm::new(builder, sender)?;
            Ok(Box::new(term))
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn build_default_backend(builder: &crate::system::InternalBuilder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    let term = WebTerminal::new(builder, sender)?;
    Ok(Box::new(term))
}

#[cfg(target_os = "windows")]
fn build_default_backend(builder: &crate::system::InternalBuilder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    let term = WindowsConsoleTerminal::new(builder, sender)?;
    Ok(Box::new(term))
}
#[cfg(target_os = "linux")]
fn build_default_backend(builder: &crate::system::InternalBuilder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    let term = TermiosTerminal::new(builder, sender)?;
    Ok(Box::new(term))
}
#[cfg(target_os = "macos")]
fn build_default_backend(builder: &crate::system::InternalBuilder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    let term = TermiosTerminal::new(builder, sender)?;
    Ok(Box::new(term))
}
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos", target_arch = "wasm32")))]
fn build_default_backend(builder: &crate::system::InternalBuilder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
    // anything else
    let term = TermiosTerminal::new(builder, sender)?;
    Ok(Box::new(term))
}
