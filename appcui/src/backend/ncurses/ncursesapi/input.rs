use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::input::MouseWheelDirection;
use crate::backend::SystemEventReader;
use crate::system::KeyPressedEvent;
use crate::system::SystemEvent;
use crate::system::MouseButtonDownEvent;
use crate::system::MouseButtonUpEvent;
use crate::system::MouseDoubleClickEvent;
use crate::system::MouseMoveEvent;
use crate::system::MouseWheelEvent;
use super::constants::*;
use crate::graphics::*;
use super::lib::*;
use super::structs::*;  
use std::time::Instant;

    
const SHFIT_NUM: [i32; 10] = [41, 33, 64, 35, 36, 37, 94, 38, 42, 40];
pub fn get_key_struct(ch: u32) -> KeyPressedEvent {
    let key_code;
    let mut key_modifier = KeyModifier::None;
    let character: char = ch as u8 as char;

    if ch == 13{
        key_code = KeyCode::Enter;
    }
    else if (97..=122).contains(&ch) {
        key_code = KeyCode::from((ch - 69) as u8);
    } else if (65..=90).contains(&ch) {
        key_code = KeyCode::from((ch - 37) as u8);
        key_modifier = KeyModifier::Shift;
    } else if (48..=57).contains(&ch) {
        key_code = KeyCode::from((ch + 6) as u8);
    } else if SHFIT_NUM.contains(&(ch as i32)) {
        let pos = SHFIT_NUM.iter().position(|&r| r == ch as i32).unwrap();
        key_code = KeyCode::from((pos + 54) as u8);
        key_modifier = KeyModifier::Shift;
    } else if (1..=26).contains(&ch) && ch != 13{
        key_code = KeyCode::from((ch + 27_u32) as u8);
        key_modifier = KeyModifier::Ctrl;
    } else {
        match ch {
            32 => key_code = KeyCode::Space,
            9 => key_code = KeyCode::Tab,
            13 => key_code = KeyCode::Enter,
            27 => key_code = KeyCode::Escape,
            263 => key_code = KeyCode::Backspace,
            _ => key_code = KeyCode::None,
        }
    }
    KeyPressedEvent {
        key: Key {
            code: key_code,
            modifier: key_modifier,
        },
        character,
    }
}

pub(crate) struct Input {
    last_event: Option<SystemEvent>,
    next_event: Option<SystemEvent>,
    diff: Instant,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            last_event: None,
            next_event: None,
            diff: Instant::now(),
        }
    }


    fn read_event(&mut self) -> Option<SystemEvent> {
        let window = ncurses_stdscr();
        let ch = ncurses_wget_wch(window);
        ch.as_ref()?;

        match ch {
            Some(WchResult::KeyCode(KEY_MOUSE)) => {
                let mut mevent = MEVENT {
                    id: 0,
                    x: 0,
                    y: 0,
                    z: 0,
                    bstate: 0,
                };
                if ncurses_getmouse(&mut mevent) == OK {
                    let x = mevent.x;
                    let y = mevent.y;
                    let button = match mevent.bstate as i32 {
                        BUTTON1_PRESSED => MouseButton::Left,
                        BUTTON1_RELEASED => MouseButton::Left,
                        BUTTON1_CLICKED => MouseButton::Left,
                        BUTTON1_DOUBLE_CLICKED => MouseButton::Left,

                        BUTTON2_PRESSED => MouseButton::Center,
                        BUTTON2_RELEASED => MouseButton::Center,
                        BUTTON2_CLICKED => MouseButton::Center,
                        BUTTON2_DOUBLE_CLICKED => MouseButton::Center,

                        // ncursesapi::constants::BUTTON3_PRESSED => MouseButton::Right,
                        // ncursesapi::constants::BUTTON3_RELEASED => MouseButton::Right,
                        // ncursesapi::constants::BUTTON3_CLICKED => MouseButton::Right,
                        // ncursesapi::constants::BUTTON3_DOUBLE_CLICKED => MouseButton::Right,
                        _ => MouseButton::None,
                    };

                    if button == MouseButton::None {
                        let button = match mevent.bstate as i32 {
                            WHEEL_UP => Some(MouseWheelDirection::Up),
                            WHEEL_DOWN => Some(MouseWheelDirection::Down),
                            WHEEL_LEFT => Some(MouseWheelDirection::Left),
                            WHEEL_RIGHT => Some(MouseWheelDirection::Right),
                            _ => None,
                        };
                        if let Some(button) = button {
                            return Some(SystemEvent::MouseWheel(MouseWheelEvent { x, y, direction: button }));
                        }
                    }
                    let mut returned = None;
                    if mevent.bstate as i32 & BUTTON1_PRESSED != 0 {
                        returned = Some(SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button }));
                    } else if mevent.bstate as i32 & BUTTON1_RELEASED != 0 {
                        returned = Some(SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button }));
                        if (self.last_event == returned) && (self.diff.elapsed().as_millis() < 300) {
                            self.next_event = Some(SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button }));
                        }
                        self.diff = Instant::now();
                        self.last_event = returned;
                    } else if mevent.bstate as i32 & REPORT_MOUSE_POSITION != 0 {
                        returned = Some(SystemEvent::MouseMove(MouseMoveEvent { x, y, button }));
                    }
                    return returned;
                }
            }
            Some(WchResult::KeyCode(KEY_RESIZE)) => {
                let w = ncurses_stdscr();
                let mut x: i32 = 0;
                let mut y: i32 = 0;
                ncurses_getmaxyx(w, &mut y, &mut x);
                let new_size = Size::new(x as u32, y as u32);
                return Some(SystemEvent::Resize(new_size));
            }
            // F1 - F12
            Some(WchResult::KeyCode(265..=276)) => {
                let key_code = match ch {
                    Some(WchResult::KeyCode(265)) => KeyCode::F1,
                    Some(WchResult::KeyCode(266)) => KeyCode::F2,
                    Some(WchResult::KeyCode(267)) => KeyCode::F3,
                    Some(WchResult::KeyCode(268)) => KeyCode::F4,
                    Some(WchResult::KeyCode(269)) => KeyCode::F5,
                    Some(WchResult::KeyCode(270)) => KeyCode::F6,
                    Some(WchResult::KeyCode(271)) => KeyCode::F7,
                    Some(WchResult::KeyCode(272)) => KeyCode::F8,
                    Some(WchResult::KeyCode(273)) => KeyCode::F9,
                    Some(WchResult::KeyCode(274)) => KeyCode::F10,
                    Some(WchResult::KeyCode(275)) => KeyCode::F11,
                    Some(WchResult::KeyCode(276)) => KeyCode::F12,
                    _ => KeyCode::None,
                };
                return Some(SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: key_code,
                        modifier: KeyModifier::None,
                    },
                    character: '\0',
                }));
            }
            // Delete
            Some(WchResult::KeyCode(330)) => {
                return Some(SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: KeyCode::Delete,
                        modifier: KeyModifier::None,
                    },
                    character: '\0',
                }));
            }
            // Arrow keys
            Some(WchResult::KeyCode(
                KEY_UP
                | KEY_DOWN
                | KEY_LEFT
                | KEY_RIGHT
                | 263,
            )) => {
                let key_code = match ch {
                    Some(WchResult::KeyCode(KEY_UP)) => KeyCode::Up,
                    Some(WchResult::KeyCode(KEY_DOWN)) => KeyCode::Down,
                    Some(WchResult::KeyCode(KEY_LEFT)) => KeyCode::Left,
                    Some(WchResult::KeyCode(KEY_RIGHT)) => KeyCode::Right,
                    Some(WchResult::KeyCode(263)) => KeyCode::Backspace,
                    _ => KeyCode::None,
                };
                return Some(SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: key_code,
                        modifier: KeyModifier::None,
                    },
                    character: '\0',
                }));
            }

            // Shift + Arrow keys
            Some(WchResult::KeyCode(
                KEY_SR | KEY_SF | KEY_SLEFT | KEY_SRIGHT,
            )) => {
                let key_code = match ch {
                    Some(WchResult::KeyCode(KEY_SR)) => KeyCode::Up,
                    Some(WchResult::KeyCode(KEY_SF)) => KeyCode::Down,
                    Some(WchResult::KeyCode(KEY_SLEFT)) => KeyCode::Left,
                    Some(WchResult::KeyCode(KEY_SRIGHT)) => KeyCode::Right,
                    _ => KeyCode::None,
                };
                return Some(SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: key_code,
                        modifier: KeyModifier::Shift,
                    },
                    character: '\0',
                }));
            }
            
            Some(WchResult::Char(ch)) => {
                if ch == 27 {
                    return Some(SystemEvent::KeyPressed(KeyPressedEvent {
                        key: Key {
                            code: KeyCode::Escape,
                            modifier: KeyModifier::None,
                        },
                        character: '\0',
                    }));
                }

                let mut key = get_key_struct(ch);
                if key.key.code == KeyCode::Backspace {
                    key.character = 8 as char;
                }
                return Some(SystemEvent::KeyPressed(key));
            }

            _ => return None,
        };

        None
    }

}

impl SystemEventReader for Input {
    fn read(&mut self) -> Option<SystemEvent> {
        if let Some(e) = self.next_event.take() {
            return Some(e);
        }
       if let Some(result) = self.read_event() {
            return Some(result);
       }
       self.last_event = None;
       None
    }
}