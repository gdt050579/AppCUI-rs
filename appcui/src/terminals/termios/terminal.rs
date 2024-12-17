//! Module representing an `TermiosTerminal` abstraction over the ANSI protocol using the termios
//! API to set it into raw mode. Targeted for UNIX systems, including `linux` and `mac`

use std::{fs::File, io::Write, os::unix::io::FromRawFd};


use libc::STDOUT_FILENO;

use super::super::{ SystemEvent, Terminal };
use crate::{ graphics::*, input::MouseButton, prelude::Key, system::Error, terminals::{termios::api::sizing::listen_for_resizes, KeyPressedEvent, MouseButtonDownEvent, MouseButtonUpEvent, MouseMoveEvent} };

use self::sizing::{get_terminal_size, set_terminal_size, RESIZE_EVENT};
#[cfg(target_family = "unix")]
use super::api::{io::{TermiosReader, AnsiKeyCode}, Termios, sizing};

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
    screen_buffer: String
}

impl TermiosTerminal {
    pub(crate) fn new(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        let Ok(_orig_termios) = Termios::enable_raw_mode() else {
            return Err(Error::new(
                crate::prelude::ErrorKind::InitializationFailure,
                "Cannot enable raw mode in Termios Terminal to get input from stdin"
                    .to_string(),
            ));
        };

        let stdout = unsafe {
            File::from_raw_fd(STDOUT_FILENO)
        };

        let mut t = TermiosTerminal {
            size: Size::new(80, 30),
            _orig_termios,
            stdout,
            screen_buffer: String::with_capacity(4096)
        };

        if let Err(err) = listen_for_resizes() {
            return Err(Error::new(
                crate::system::ErrorKind::InitializationFailure,
                format!("Failed to setup terminal resize listener: {:?}", err)));
        }
        
        if let Some(sz) = builder.size {
            t.size = sz;
        }
        
        match get_terminal_size() {
            Err(err) => { return Err(Error::new(crate::system::ErrorKind::InitializationFailure, err.to_string())); }
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
        Ok(Box::new(t))
    }

    fn clear(&mut self) {
        let _ = self.stdout.write("\x1b[2J".as_bytes());
    }

    fn update_size(&mut self) -> Result<(), std::io::Error> {
        self.size = get_terminal_size()?;
        Ok(())
    }

    fn move_cursor(&mut self, to: &Cursor) -> Result<(), std::io::Error> {
        self.stdout.write_all(format!("\x1b[{};{}H", to.y + 1, to.x + 1).as_bytes())?;

        Ok(())
    }
}

impl Terminal for TermiosTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        self.clear();
        
        let chars = &surface.chars;
        
        self.screen_buffer.clear();
        let mut x = 0;
        let width = surface.size().width;
        for c in chars {
            match c.foreground {
                Color::Black => self.screen_buffer.push_str("\x1b[30m"),
                Color::DarkBlue => self.screen_buffer.push_str("\x1b[34m"),
                Color::DarkGreen => self.screen_buffer.push_str("\x1b[32m"),
                Color::Teal => self.screen_buffer.push_str("\x1b[36m"),
                Color::DarkRed => self.screen_buffer.push_str("\x1b[31m"),
                Color::Magenta => self.screen_buffer.push_str("\x1b[35m"),
                Color::Olive => self.screen_buffer.push_str("\x1b[33m"),
                Color::Silver => self.screen_buffer.push_str("\x1b[37m"),
                Color::Gray => self.screen_buffer.push_str("\x1b[90m"),
                Color::Blue => self.screen_buffer.push_str("\x1b[94m"),
                Color::Green => self.screen_buffer.push_str("\x1b[92m"),
                Color::Aqua => self.screen_buffer.push_str("\x1b[96m"),
                Color::Red => self.screen_buffer.push_str("\x1b[91m"),
                Color::Pink => self.screen_buffer.push_str("\x1b[95m"),
                Color::Yellow => self.screen_buffer.push_str("\x1b[93m"),
                Color::White => self.screen_buffer.push_str("\x1b[97m"),
                Color::Transparent => {}
            }

            match c.background {
                Color::Black => self.screen_buffer.push_str("\x1b[40m"),
                Color::DarkBlue => self.screen_buffer.push_str("\x1b[44m"),
                Color::DarkGreen => self.screen_buffer.push_str("\x1b[42m"),
                Color::Teal => self.screen_buffer.push_str("\x1b[46m"),
                Color::DarkRed => self.screen_buffer.push_str("\x1b[41m"),
                Color::Magenta => self.screen_buffer.push_str("\x1b[45m"),
                Color::Olive => self.screen_buffer.push_str("\x1b[43m"),
                Color::Silver => self.screen_buffer.push_str("\x1b[47m"),
                Color::Gray => self.screen_buffer.push_str("\x1b[100m"),
                Color::Blue => self.screen_buffer.push_str("\x1b[104m"),
                Color::Green => self.screen_buffer.push_str("\x1b[102m"),
                Color::Aqua => self.screen_buffer.push_str("\x1b[106m"),
                Color::Red => self.screen_buffer.push_str("\x1b[101m"),
                Color::Pink => self.screen_buffer.push_str("\x1b[105m"),
                Color::Yellow => self.screen_buffer.push_str("\x1b[103m"),
                Color::White => self.screen_buffer.push_str("\x1b[107m"),
                Color::Transparent => {}
            }
            
            self.screen_buffer.push(c.code);
            self.screen_buffer.push_str("\x1b[0m");

            x += 1;
            if x >= width {
                self.screen_buffer.push('\n');
                x = 0;
            }
        }
        let buf = self.screen_buffer.as_bytes();
        let _ = self.stdout.write(&buf[..buf.len() - 1]);

        let _ = self.move_cursor(&surface.cursor);
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_system_event(&mut self) -> SystemEvent {
        if RESIZE_EVENT.load(std::sync::atomic::Ordering::SeqCst) {
            RESIZE_EVENT.store(false, std::sync::atomic::Ordering::SeqCst);
            if let Err(_) = self.update_size() {
                return SystemEvent::None;
            };

            return SystemEvent::Resize(self.size);
        }
        
        #[cfg(target_family = "unix")]
        match TermiosReader::read_key() {
            Ok(ansi_key) => {
                if let AnsiKeyCode::MouseButton(ev) = ansi_key.code() {
                    match ev.button {
                        MouseButton::None => return SystemEvent::MouseButtonUp(MouseButtonUpEvent {x: ev.x.into(), y: ev.y.into(), button: MouseButton::None}),
                        other => return SystemEvent::MouseButtonDown(MouseButtonDownEvent {button: other, x: ev.x.into(), y: ev.y.into()})
                    }
                }
                if let AnsiKeyCode::MouseMove(ev) = ansi_key.code() {
                    return SystemEvent::MouseMove(MouseMoveEvent { x: ev.x.into(), y: ev.y.into(), button: ev.button });
                }

                // We take the initial 4 bytes an we try to convert them into an `u32`
                let Some(bytes) = ansi_key.bytes().get(0..4) else {
                    return SystemEvent::None;
                };
                let value = u32::from_le_bytes(bytes.try_into().unwrap_or([0; 4]));

                let character = char::from_u32(value).unwrap_or('\0');

                // We convert our ANSI key to the system's `Key` known key type
                let key: Key = ansi_key.into();
                SystemEvent::KeyPressed(KeyPressedEvent {
                    key,
                    character,
                })
            }
            Err(_) => SystemEvent::None,
        }

        // Currently the way we handle raw terminal input is not available on windows through
        // termios
        #[cfg(target_family = "windows")]
        SystemEvent::None
    }
    fn get_clipboard_text(&self) -> Option<String> {
        todo!()
    }

    fn set_clipboard_text(&mut self, _text: &str) {
        todo!()
    }

    fn has_clipboard_text(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UnsupportedCode([u8; 5]);
