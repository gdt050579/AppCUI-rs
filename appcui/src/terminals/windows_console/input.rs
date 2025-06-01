use std::sync::Arc;
use std::sync::Mutex;

use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::input::MouseWheelDirection;
use crate::terminals::SystemEventReader;

use super::super::system_event::KeyModifierChangedEvent;
use super::super::KeyPressedEvent;
use super::super::MouseButtonDownEvent;
use super::super::MouseButtonUpEvent;
use super::super::MouseDoubleClickEvent;
use super::super::MouseMoveEvent;
use super::super::MouseWheelEvent;
use super::super::SystemEvent;
use super::constants::*;
use super::structs::*;
use super::utils;
use super::winapi;
use crate::graphics::*;

pub(crate) struct Input {
    stdin: HANDLE,
    stdout: HANDLE,
    shift_state: KeyModifier,
    last_mouse_pos: Point,
    visible_region: SMALL_RECT,
    shared_visible_region: Arc<Mutex<SMALL_RECT>>,
}

const TRANSLATION_MATRIX: [KeyCode; 256] = [
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::Backspace,
    KeyCode::Tab,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::Enter,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::Escape,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::Space,
    KeyCode::PageUp,
    KeyCode::PageDown,
    KeyCode::End,
    KeyCode::Home,
    KeyCode::Left,
    KeyCode::Up,
    KeyCode::Right,
    KeyCode::Down,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::Insert,
    KeyCode::Delete,
    KeyCode::None,
    KeyCode::N0,
    KeyCode::N1,
    KeyCode::N2,
    KeyCode::N3,
    KeyCode::N4,
    KeyCode::N5,
    KeyCode::N6,
    KeyCode::N7,
    KeyCode::N8,
    KeyCode::N9,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::A,
    KeyCode::B,
    KeyCode::C,
    KeyCode::D,
    KeyCode::E,
    KeyCode::F,
    KeyCode::G,
    KeyCode::H,
    KeyCode::I,
    KeyCode::J,
    KeyCode::K,
    KeyCode::L,
    KeyCode::M,
    KeyCode::N,
    KeyCode::O,
    KeyCode::P,
    KeyCode::Q,
    KeyCode::R,
    KeyCode::S,
    KeyCode::T,
    KeyCode::U,
    KeyCode::V,
    KeyCode::W,
    KeyCode::X,
    KeyCode::Y,
    KeyCode::Z,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::F1,
    KeyCode::F2,
    KeyCode::F3,
    KeyCode::F4,
    KeyCode::F5,
    KeyCode::F6,
    KeyCode::F7,
    KeyCode::F8,
    KeyCode::F9,
    KeyCode::F10,
    KeyCode::F11,
    KeyCode::F12,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
    KeyCode::None,
];

impl Input {
    pub(super) fn new(stdin: HANDLE, stdout: HANDLE, visible_region: SMALL_RECT, shared_visible_region: Arc<Mutex<SMALL_RECT>>) -> Self {
        Self {
            stdin,
            stdout,
            shift_state: KeyModifier::None,
            last_mouse_pos: Point::new(i32::MAX, i32::MAX),
            visible_region,
            shared_visible_region,
        }
    }
}

impl SystemEventReader for Input {
    fn read(&mut self) -> Option<SystemEvent> {
        let mut ir = INPUT_RECORD {
            event_type: 0,
            event: WindowsTerminalEvent { extra: 0 },
        };
        let mut nr_read = 0u32;

        unsafe {
            if (winapi::ReadConsoleInputW(self.stdin, &mut ir, 1, &mut nr_read) == FALSE) || (nr_read != 1) {
                return None;
            }
            //println!("Event: {}",ir.event_type);
        }

        // Key processings
        if ir.event_type == KEY_EVENT {
            let mut key_code = KeyCode::None;
            let mut key_modifier = KeyModifier::None;
            let mut character = '\0';
            unsafe {
                if (ir.event.key_event.unicode_char >= 32) && (ir.event.key_event.key_down == TRUE) {
                    let res = char::from_u32(ir.event.key_event.unicode_char as u32);
                    if res.is_some() {
                        character = res.unwrap();
                    }
                }
                if ir.event.key_event.virtual_key_code < 256 {
                    key_code = TRANSLATION_MATRIX[ir.event.key_event.virtual_key_code as usize];
                }

                if (ir.event.key_event.control_key_state & (LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED)) != 0 {
                    key_modifier |= KeyModifier::Alt;
                }
                if (ir.event.key_event.control_key_state & (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED)) != 0 {
                    key_modifier |= KeyModifier::Ctrl;
                }
                if (ir.event.key_event.control_key_state & SHIFT_PRESSED) != 0 {
                    key_modifier |= KeyModifier::Shift;
                }

                // if ALT or CTRL are pressed, clear the ascii code
                if key_modifier.contains_one(KeyModifier::Alt | KeyModifier::Ctrl) {
                    character = '\0';
                }
                if (key_code != KeyCode::None) || (character != '\0') {
                    if ir.event.key_event.key_down == FALSE {
                        // key is up (no need to send)
                        return None;
                    }
                } else {
                    // check for change in modifier
                    if self.shift_state == key_modifier {
                        // nothing changed --> return
                        return None;
                    }
                    let old_state = self.shift_state;
                    self.shift_state = key_modifier;
                    return Some(SystemEvent::KeyModifierChanged(KeyModifierChangedEvent {
                        new_state: key_modifier,
                        old_state,
                    }));
                }
            }
            return Some(SystemEvent::KeyPressed(KeyPressedEvent {
                key: Key::new(key_code, key_modifier),
                character,
            }));
        }

        // mouse processing
        if ir.event_type == MOUSE_EVENT {
            unsafe {
                let x = (ir.event.mouse_event.mouse_position.x as i32) - (self.visible_region.left as i32);
                let y = (ir.event.mouse_event.mouse_position.y as i32) - (self.visible_region.top as i32);
                // for Windows 11
                if ir.event.mouse_event.event_flags == 0x01 {
                    if (x == self.last_mouse_pos.x) && (y == self.last_mouse_pos.y) {
                        return None;
                    }

                    self.last_mouse_pos.x = x;
                    self.last_mouse_pos.y = y;
                }

                let button = {
                    if (ir.event.mouse_event.button_state & FROM_LEFT_1ST_BUTTON_PRESSED) != 0 {
                        MouseButton::Left
                    } else if (ir.event.mouse_event.button_state & RIGHTMOST_BUTTON_PRESSED) != 0 {
                        MouseButton::Right
                    } else if ir.event.mouse_event.button_state > 0 {
                        MouseButton::Center
                    } else {
                        MouseButton::None
                    }
                };

                match ir.event.mouse_event.event_flags {
                    0 => {
                        if ir.event.mouse_event.button_state != 0 {
                            return Some(SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button }));
                        } else {
                            return Some(SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button }));
                        }
                    }
                    DOUBLE_CLICK => {
                        return Some(SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button }));
                    }
                    MOUSE_MOVED => {
                        return Some(SystemEvent::MouseMove(MouseMoveEvent { x, y, button }));
                    }
                    MOUSE_HWHEELED => {
                        //println!("HWHEEL {}", ir.event.mouse_event.button_state);
                        if ir.event.mouse_event.button_state >= 0x80000000 {
                            return Some(SystemEvent::MouseWheel(MouseWheelEvent {
                                x,
                                y,
                                direction: MouseWheelDirection::Left,
                            }));
                        } else {
                            return Some(SystemEvent::MouseWheel(MouseWheelEvent {
                                x,
                                y,
                                direction: MouseWheelDirection::Right,
                            }));
                        }
                    }
                    MOUSE_WHEELED => {
                        if ir.event.mouse_event.button_state >= 0x80000000 {
                            return Some(SystemEvent::MouseWheel(MouseWheelEvent {
                                x,
                                y,
                                direction: MouseWheelDirection::Down,
                            }));
                        } else {
                            return Some(SystemEvent::MouseWheel(MouseWheelEvent {
                                x,
                                y,
                                direction: MouseWheelDirection::Up,
                            }));
                        }
                    }
                    _ => {
                        return None;
                    }
                }
            }
        }

        // resize
        if ir.event_type == WINDOW_BUFFER_SIZE_EVENT {
            if let Ok(info) = utils::get_console_screen_buffer_info(self.stdout) {
                let w = (info.window.right as u32) + 1 - (info.window.left as u32);
                let h = (info.window.bottom as u32) + 1 - (info.window.top as u32);
                self.visible_region = info.window;
                if let Ok(mut shared_data) = self.shared_visible_region.lock() {
                    *shared_data = info.window;
                }
                return Some(SystemEvent::Resize(Size::new(w, h)));
                // self.chars.resize((w as usize) * (h as usize) * 2, CHAR_INFO { code: 32, attr: 0 });
                // self.size = Size::new(w, h);
            }
            // return SystemEvent::Resize(self.size);
        }

        None
    }
}
