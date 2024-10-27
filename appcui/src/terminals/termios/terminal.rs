//! Module representing an `TermiosTerminal` abstraction over the ANSI protocol using the termios
//! API to set it into raw mode. Targeted for UNIX systems, including `linux` and `mac`

use super::super::{ SystemEvent, Terminal };
use crate::{ graphics::*, input::MouseButton, prelude::{Key, KeyModifier}, system::Error, terminals::{KeyPressedEvent, MouseButtonDownEvent, MouseButtonUpEvent, MouseMoveEvent} };

#[cfg(target_family = "unix")]
use super::api::{io::{TermiosReader, AnsiKeyCode, Letter}, Termios};

/// Represents a terminal interface that has support for termios API terminals, supported by unix
/// family and outputs ANSI escape codes and receives input from
/// the standard input descriptor
pub struct TermiosTerminal {
    // Size of the window created
    size: Size,
    // We keep the original `Termios` structure, such that before the application exits, we return
    // the terminal as the user had it initially.
    _orig_termios: Termios,
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

        let mut t = TermiosTerminal {
            size: Size::new(80, 30),
            _orig_termios,
        };
        if let Some(sz) = builder.size {
            t.size = sz;

            // If the terminal size is invalid, we will return an error
            // However, we are returning an `Err` without checking that :-?
            return Err(Error::new(
                crate::prelude::ErrorKind::InvalidFeature,
                "TermiosTerminal is not yet implemented to support custom sizes".to_owned(),
            ));
        }

        print!("\x1b[?1003h"); // capture mouse events
        Ok(Box::new(t))
    }

    fn clear(&mut self) {
        print!("\x1b[2J");
    }
}

impl Terminal for TermiosTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        self.clear();
        
        let mut s = String::new();
        let sz = surface.size();
        for y in 0..sz.height {
            for x in 0..sz.width {
                if let Some(c) = surface.get(x as i32, y as i32) {
                    // Set the foreground using ANSI escape codes
                    match c.foreground {
                        Color::Black => s.push_str("\x1b[30m"),
                        Color::DarkBlue => s.push_str("\x1b[34m"),
                        Color::DarkGreen => s.push_str("\x1b[32m"),
                        Color::Teal => s.push_str("\x1b[36m"),
                        Color::DarkRed => s.push_str("\x1b[31m"),
                        Color::Magenta => s.push_str("\x1b[35m"),
                        Color::Olive => s.push_str("\x1b[33m"),
                        Color::Silver => s.push_str("\x1b[37m"),
                        Color::Gray => s.push_str("\x1b[90m"),
                        Color::Blue => s.push_str("\x1b[94m"),
                        Color::Green => s.push_str("\x1b[92m"),
                        Color::Aqua => s.push_str("\x1b[96m"),
                        Color::Red => s.push_str("\x1b[91m"),
                        Color::Pink => s.push_str("\x1b[95m"),
                        Color::Yellow => s.push_str("\x1b[93m"),
                        Color::White => s.push_str("\x1b[97m"),
                        Color::Transparent => {}
                    }
                    s.push(c.code);
                    s.push_str("\x1b[0m");
                }
            }
            s.push('\n');
        }
        print!("{}", s);
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_system_event(&mut self) -> SystemEvent {
        #[cfg(target_family = "unix")]
        match TermiosReader::read_key() {
            Ok(ansi_key) => {
                // If the key pressed was `Ctrl-Q` we quit the application
                if ansi_key.modifier() == KeyModifier::Ctrl
                    && ansi_key.code() == AnsiKeyCode::Letter(Letter::Q) {
                    return SystemEvent::AppClose;
                }

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

    fn set_clipboard_text(&mut self, text: &str) {
        todo!()
    }

    fn has_clipboard_text(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UnsupportedCode([u8; 5]);
