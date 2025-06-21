//! Module representing an `TermiosTerminal` abstraction over the ANSI protocol using the termios
//! API to set it into raw mode. Targeted for UNIX systems, including `linux` and `mac`

use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use libc::STDOUT_FILENO;
use std::{fs::File, io::Write, os::unix::io::FromRawFd, sync::mpsc::Sender};

use super::{
    super::SystemEvent,
    api::sizing::{get_resize_notification, get_terminal_size, set_terminal_size},
    input::Input,
    size_reader::SizeReader,
};
use crate::backend::utils::AnsiFormatter;
use crate::{
    backend::{termios::api::sizing::listen_for_resizes, Backend, SystemEventReader},
    graphics::*,
    system::Error,
};

#[cfg(target_family = "unix")]
use super::api::Termios;

/// Represents a terminal interface that has support for termios API terminals, supported by unix
/// family and outputs ANSI escape codes and receives input from
/// the standard input descriptor
pub struct TermiosTerminal {
    // Size of the window created
    size: Size,
    // We keep the original `Termios` structure, such that before the application exits, we return
    // the terminal as the user had it initially.
    _orig_termios: Termios,

    stdout: File,
    ansi_buffer: AnsiFormatter,
}

impl TermiosTerminal {
    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
        let Ok(_orig_termios) = Termios::enable_raw_mode() else {
            return Err(Error::new(
                crate::prelude::ErrorKind::InitializationFailure,
                "Cannot enable raw mode in Termios backend to get input from stdin".to_string(),
            ));
        };

        let stdout = unsafe { File::from_raw_fd(STDOUT_FILENO) };

        let mut t = TermiosTerminal {
            size: Size::new(80, 30),
            _orig_termios,
            stdout,
            ansi_buffer: AnsiFormatter::new(
                16384,
                if builder.use_color_schema {
                    AnsiFlags::Use16ColorSchema
                } else {
                    AnsiFlags::None
                },
            ),
        };

        if let Err(err) = listen_for_resizes() {
            return Err(Error::new(
                crate::system::ErrorKind::InitializationFailure,
                format!("Failed to setup terminal resize listener: {:?}", err),
            ));
        }

        if let Some(sz) = builder.size {
            t.size = sz;
        }

        match get_terminal_size() {
            Err(err) => {
                return Err(Error::new(crate::system::ErrorKind::InitializationFailure, err.to_string()));
            }
            Ok(size) => {
                if size != t.size {
                    if let Err(err) = set_terminal_size(&size) {
                        return Err(Error::new(crate::system::ErrorKind::InitializationFailure, err.to_string()));
                    }
                }
                t.size = size;
            }
        }

        let _ = t.stdout.write("\x1b[?1003h".as_bytes()); // capture mouse events

        Input::new().start(sender.clone());
        SizeReader::new(get_resize_notification().clone()).start(sender);
        Ok(Box::new(t))
    }

    // fn clear(&mut self) {
    //     let _ = self.stdout.write("\x1b[2J".as_bytes());
    // }

    // fn move_cursor(&mut self, to: &Cursor) -> Result<(), std::io::Error> {
    //     if !to.is_visible() {
    //         return Ok(());
    //     };

    //     self.stdout
    //         .write_all(format!("\x1b[{};{}H", to.y.saturating_add(1), to.x.saturating_add(1)).as_bytes())?;

    //     Ok(())
    // }
}

impl Backend for TermiosTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        //self.clear();
        self.ansi_buffer.render(surface, Point::new(0, 0));
        let _ = std::io::stdout().write_all(self.ansi_buffer.text().as_bytes());
        let _ = std::io::stdout().flush();
    }

    fn size(&self) -> Size {
        self.size
    }

    fn query_system_event(&mut self) -> Option<SystemEvent> {
        None
    }
    fn clipboard_text(&self) -> Option<String> {
        let mut ctx: ClipboardContext = ClipboardContext::new().ok()?;
        ctx.get_contents().ok()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
        ctx.set_contents(text.to_owned()).unwrap();
    }

    fn has_clipboard_text(&self) -> bool {
        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
        ctx.get_contents().is_ok()
    }

    fn on_resize(&mut self, new_size: Size) {
        self.size = new_size;
    }

    fn is_single_threaded(&self) -> bool {
        false
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UnsupportedCode([u8; 5]);
