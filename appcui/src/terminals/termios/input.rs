use crate::input::Key;
use crate::input::KeyCode;
use crate::input::MouseButton;
use crate::terminals::KeyPressedEvent;
use crate::terminals::MouseButtonDownEvent;
use crate::terminals::MouseButtonUpEvent;
use crate::terminals::MouseMoveEvent;
use crate::terminals::SystemEvent;
use crate::terminals::SystemEventReader;

use super::api::io::AnsiKeyCode;
use super::api::io::TermiosReader;

pub(super) struct Input {}

impl Input {
    pub(super) fn new() -> Self {
        Self {}
    }
}

impl SystemEventReader for Input {
    fn read(&mut self) -> Option<crate::terminals::SystemEvent> {        
        #[cfg(target_family = "unix")]
        match TermiosReader::read_key() {
            Ok(ansi_key) => {
                if let AnsiKeyCode::MouseButton(ev) = ansi_key.code() {
                    match ev.button {
                        MouseButton::None => {
                            return Some(SystemEvent::MouseButtonUp(MouseButtonUpEvent {
                                x: ev.x.into(),
                                y: ev.y.into(),
                                button: MouseButton::None,
                            }))
                        }
                        other => {
                            return Some(SystemEvent::MouseButtonDown(MouseButtonDownEvent {
                                button: other,
                                x: ev.x.into(),
                                y: ev.y.into(),
                            }))
                        }
                    }
                }
                if let AnsiKeyCode::MouseMove(ev) = ansi_key.code() {
                    return Some(SystemEvent::MouseMove(MouseMoveEvent {
                        x: ev.x.into(),
                        y: ev.y.into(),
                        button: ev.button,
                    }));
                }

                // We take the initial 4 bytes an we try to convert them into an `u32`
                let Some(bytes) = ansi_key.bytes().get(0..4) else {
                    return None;
                };
                let value = u32::from_le_bytes(bytes.try_into().unwrap_or([0; 4]));

                let mut character = char::from_u32(value).unwrap_or('\0');

                // We convert our ANSI key to the system's `Key` known key type
                let key: Key = ansi_key.into();

                match key.code {
                    KeyCode::F1
                    | KeyCode::F2
                    | KeyCode::F3
                    | KeyCode::F4
                    | KeyCode::F5
                    | KeyCode::F6
                    | KeyCode::F7
                    | KeyCode::F8
                    | KeyCode::F9
                    | KeyCode::F10
                    | KeyCode::F11
                    | KeyCode::F12
                    | KeyCode::Enter
                    | KeyCode::Escape
                    | KeyCode::Insert
                    | KeyCode::Delete
                    | KeyCode::Backspace
                    | KeyCode::Tab
                    | KeyCode::Left
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Right
                    | KeyCode::PageUp
                    | KeyCode::PageDown
                    | KeyCode::Home
                    | KeyCode::End => {
                        character = 0 as char;
                    }
                    _ => {}
                }

                Some(SystemEvent::KeyPressed(KeyPressedEvent { key, character }))
            }
            Err(_) => None,
        }

        // Currently the way we handle raw terminal input is not available on windows through
        // termios
        #[cfg(target_family = "windows")]
        SystemEvent::None
    }
}
