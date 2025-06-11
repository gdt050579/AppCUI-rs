use std::sync::Arc;
use std::sync::Mutex;

use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::input::MouseWheelDirection;
use crate::terminals::SystemEventReader;

use super::super::system_event::KeyModifierChangedEvent;
use super::super::utils::win32;
use super::super::utils::win32::api;
use super::super::KeyPressedEvent;
use super::super::MouseButtonDownEvent;
use super::super::MouseButtonUpEvent;
use super::super::MouseDoubleClickEvent;
use super::super::MouseMoveEvent;
use super::super::MouseWheelEvent;
use super::super::SystemEvent;
use crate::graphics::*;
use crate::terminals::utils::win32::constants::*;
use crate::terminals::utils::win32::structs::*;

pub(crate) struct Input {
    stdin: HANDLE,
    stdout: HANDLE,
    shift_state: KeyModifier,
    last_mouse_pos: Point,
    visible_region: SMALL_RECT,
    shared_visible_region: Arc<Mutex<SMALL_RECT>>,
}

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
            if (win32::api::ReadConsoleInputW(self.stdin, &mut ir, 1, &mut nr_read) == FALSE) || (nr_read != 1) {
                return None;
            }
            //println!("Event: {}",ir.event_type);
        }

        // Key processings
        if ir.event_type == KEY_EVENT {
            return unsafe { ir.event.key_event.to_system_event(&mut self.shift_state) };
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
            if let Ok(info) = win32::console_screen_buffer_info(self.stdout) {
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
