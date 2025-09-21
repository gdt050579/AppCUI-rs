//! Module representing an `TermiosTerminal` abstraction over the ANSI protocol using the termios
//! API to set it into raw mode. Targeted for UNIX systems, including `linux` and `mac`

use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use std::{io::Write, sync::mpsc::Sender};

use super::{
    super::SystemEvent,
    api::sizing::{get_resize_notification, get_terminal_size, set_terminal_size},
    input::Input,
    size_reader::SizeReader,
};
use crate::backend::utils::{AnsiFlags, AnsiFormatter};
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
    size: Size,
    orig_termios: Termios,
    ansi_buffer: AnsiFormatter,
}

impl TermiosTerminal {
    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Box<dyn Backend>, Error> {
        let Ok(orig_termios) = Termios::enable_raw_mode() else {
            return Err(Error::new(
                crate::prelude::ErrorKind::InitializationFailure,
                "Cannot enable raw mode in Termios backend to get input from stdin".to_string(),
            ));
        };

        let mut t = TermiosTerminal {
            size: Size::new(80, 30),
            orig_termios,
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

        t.ansi_buffer.clear();
        t.ansi_buffer.enable_mouse_events();
        let _ = std::io::stdout().write_all(t.ansi_buffer.text().as_bytes());
        let _ = std::io::stdout().flush();

        Input::new().start(sender.clone());
        SizeReader::new(get_resize_notification().clone()).start(sender);
        Ok(Box::new(t))
    }
}

impl Backend for TermiosTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        //self.clear();
        self.ansi_buffer.render(surface, Point::new(0, 0));
        let _ = std::io::stdout().write_all(self.ansi_buffer.text().as_bytes());
        let _ = std::io::stdout().flush();
    }

    fn on_resize(&mut self, new_size: Size) {
        self.size = new_size;
    }

    fn size(&self) -> Size {
        self.size
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

    fn query_system_event(&mut self) -> Option<SystemEvent> {
        None
    }

    fn is_single_threaded(&self) -> bool {
        false
    }

    fn on_close(&mut self) {
        self.ansi_buffer.clear();
        self.ansi_buffer.disable_mouse_events();
        let _ = std::io::stdout().write_all(self.ansi_buffer.text().as_bytes());
        let _ = std::io::stdout().flush();
        self.orig_termios.restore();
    }
}