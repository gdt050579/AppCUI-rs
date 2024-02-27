//! Module representing an `AnsiTerminal` abstraction over the ANSI protocol
use super::super::{ SystemEvent, Terminal };
use crate::{ graphics::*, system::Error, prelude::{Key, KeyModifier}, terminals::KeyPressedEvent };
use super::termios::{io::{TermiosReader, AnsiKeyCode, Letter}, Termios};

/// Represents a terminal interface that has support for ANSI escape codes and receives input from
/// the standard input descriptor
pub struct AnsiTerminal {
    // Size of the window created
    size: Size,
    // We keep the original `Termios` structure, such that before the application exits, we return
    // the terminal as the user had it initially.
    _orig_termios: Termios,
}

impl AnsiTerminal {
    pub(crate) fn new(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        let _orig_termios = Termios::enable_raw_mode()?;

        let mut t = AnsiTerminal { size: Size::new(80, 30), _orig_termios };
        if let Some(sz) = builder.size {
            t.size = sz;

            // If the terminal size is invalid, we will return an error
            // However, we are returning an `Err` without checking that :-?
            return Err(Error::new(
                crate::prelude::ErrorKind::InvalidFeature,
                "AnsiTerminal is not yet implemented to support custom sizes".to_owned(),
            ));
        }
        Ok(Box::new(t))
    }
}

impl Terminal for AnsiTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        let mut s = String::new();
        let sz = surface.get_size();
        for y in 0..sz.height {
            for x in 0..sz.width {
                if let Some(c) = surface.get(x as i32, y as i32) {
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
        let maybe_ansi_key = TermiosReader::read_key();

        match maybe_ansi_key {
            Ok(ansi_key) => {
                // If the key pressed was `Ctrl-Q` we quit the application
                if ansi_key.modifier() == KeyModifier::Ctrl
                    && ansi_key.code() == AnsiKeyCode::Letter(Letter::Q) {
                    return SystemEvent::AppClose;
                }

                // We take the initial 4 bytes an we try to convert them into an `u32`
                let Some(bytes) = ansi_key.bytes().get(0..4) else {
                    return SystemEvent::None;
                };
                let value = u32::from_le_bytes(bytes.try_into().unwrap_or([0; 4]));

                let Some(character) = char::from_u32(value) else {
                    return SystemEvent::None;
                };

                // We convert our ANSI key to the system's `Key` known key type
                let key: Key = ansi_key.into();
                SystemEvent::KeyPressed(KeyPressedEvent {
                    key,
                    character,
                })
            }
            Err(_) => SystemEvent::None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UnsupportedCode([u8; 5]);
