use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::input::MouseWheelDirection;

use super::super::system_event::KeyModifierChangedEvent;
use super::super::CharFlags;
use super::super::Color;
use super::super::KeyPressedEvent;
use super::super::MouseButtonDownEvent;
use super::super::MouseButtonUpEvent;
use super::super::MouseDoubleClickEvent;
use super::super::MouseMoveEvent;
use super::super::MouseWheelEvent;
use super::super::Surface;
use super::super::SystemEvent;
use super::super::Terminal;
use super::structs::*;
use super::constants::*;
use super::winapi;
use crate::system::Error;




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


fn get_stdin_handle() -> Result<HANDLE,Error> {
    unsafe {
        let h = winapi::GetStdHandle(STD_INPUT_HANDLE);
        if h == INVALID_HANDLE_VALUE {
            return Err(Error::FailToGetStdInHandle);
        }
        return Ok(h);
    }
}
fn get_stdout_handle() -> Result<HANDLE,Error> {
    unsafe {
        let h = winapi::GetStdHandle(STD_OUTPUT_HANDLE);
        if h == INVALID_HANDLE_VALUE {
            return Err(Error::FailToGetStdOutHandle);
        }
        return Ok(h);
    }
}

fn get_console_screen_buffer_info(handle: HANDLE) -> Result<CONSOLE_SCREEN_BUFFER_INFO,Error> {
    unsafe {
        let mut cbuf = CONSOLE_SCREEN_BUFFER_INFO::default();
        if winapi::GetConsoleScreenBufferInfo(handle, &mut cbuf) == FALSE {
            return Err(Error::GetConsoleScreenBufferInfoFailed);
        }
        return Ok(cbuf);
    }
}

pub struct WindowsTerminal {
    stdin_handle: HANDLE,
    stdout_handle: HANDLE,
    width: u32,
    height: u32,
    chars: Vec<CHAR_INFO>,
    shift_state: KeyModifier,
    last_mouse_x: i32,
    last_mouse_y: i32,
    original_mode_flags: u32,
}

impl WindowsTerminal {
    pub (crate) fn create() -> Result<Box<dyn Terminal>,Error> {
        let stdin = get_stdin_handle()?;
        let stdout = get_stdout_handle()?;
        let mut original_mode_flags = 0u32;
        
        unsafe {
            if winapi::GetConsoleMode(stdin, &mut original_mode_flags) == FALSE {
                return Err(Error::GetConsoleModeFailed);
            }
            if winapi::SetConsoleMode(
                stdin,
                ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS,
            ) == FALSE
            {
                return Err(Error::SetConsoleModeFailed);
            }
        }
        let info = get_console_screen_buffer_info(stdout)?;
        if (info.size.x < 1) || (info.size.y < 1) {
            return Err(Error::InvalidSize);
        }
        println!("{:?}",info);
        panic!("exit");
        let mut term = Box::new(WindowsTerminal {
            stdin_handle: stdin,
            stdout_handle: stdout,
            width: info.size.x as u32,
            height: info.size.y as u32,
            chars: Vec::with_capacity(1024),
            shift_state: KeyModifier::None,
            last_mouse_x: i32::MAX,
            last_mouse_y: i32::MAX,
            original_mode_flags: original_mode_flags,
        });
        term.chars.resize(
            (term.width as usize) * (term.height as usize),
            CHAR_INFO { code: 32, attr: 0 },
        );
        return Ok(term);
    }
}

impl Terminal for WindowsTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        // safety check --> surface size should be the same as self.width/height size
        if (surface.width != self.width) || (surface.height != self.height) {
            return;
        }

        // copy surface into CHAR_INFO
        let sz = self.chars.len();
        for i in 0..sz {
            let ch = &surface.chars[i];
            let screen_char = &mut (self.chars[i]);
            if ch.code != (0 as char) {
                screen_char.code = ch.code as u16;
            } else {
                screen_char.code = 32; // fallback to space
            }
            screen_char.attr = 0;
            if ch.foreground != Color::Transparent {
                screen_char.attr = (ch.foreground as u8) as u16;
            }
            if ch.background != Color::Transparent {
                screen_char.attr |= ((ch.background as u8) as u16) << 4;
            }
            if ch.flags.contains(CharFlags::Underline) {
                screen_char.attr |= COMMON_LVB_UNDERSCORE;
            }
        }
        //
        let sz = COORD {
            x: self.width as i16,
            y: self.height as i16,
        };
        let start = COORD { x: 0, y: 0 };
        let region = SMALL_RECT {
            left: 0,
            top: 0,
            right: sz.x - 1,
            bottom: sz.y - 1,
        };
        unsafe {
            winapi::WriteConsoleOutputW(self.stdout_handle, self.chars.as_ptr(), sz, start, &region);
        }
        // update the cursor
        if surface.cursor.is_visible() {
            let pos = COORD {
                x: surface.cursor.x as i16,
                y: surface.cursor.y as i16,
            };
            let info = CONSOLE_CURSOR_INFO {
                size: 10,
                visible: TRUE,
            };
            unsafe {
                winapi::SetConsoleCursorPosition(self.stdout_handle, pos);
                winapi::SetConsoleCursorInfo(self.stdout_handle, &info);
            }
        } else {
            let info = CONSOLE_CURSOR_INFO {
                size: 10,
                visible: FALSE,
            };
            unsafe {
                winapi::SetConsoleCursorInfo(self.stdout_handle, &info);
            }
        }
    }
    fn get_width(&self) -> u32 {
        return self.width;
    }
    fn get_height(&self) -> u32 {
        return self.height;
    }
    fn on_resize(&mut self, new_size: crate::graphics::Size) {
        if (self.width == new_size.width) && (self.height == new_size.height) {
            return;
        }     
        self.chars.resize(
            (new_size.width as usize) * (new_size.height as usize),
            CHAR_INFO { code: 32, attr: 0 },
        );
        self.width = new_size.width;
        self.height = new_size.height;
    }
    fn get_system_event(&mut self) -> SystemEvent {
        let mut ir = INPUT_RECORD {
            event_type: 0,
            event: WindowsTerminalEvent { extra: 0 },
        };
        let mut nr_read = 0u32;

        unsafe {
            if (winapi::ReadConsoleInputW(self.stdin_handle, &mut ir, 1, &mut nr_read) == FALSE)
                || (nr_read != 1)
            {
                return SystemEvent::None;
            }
            //println!("Event: {}",ir.event_type);
        }

        // Key processings
        if ir.event_type == KEY_EVENT {
            let mut key_code = KeyCode::None;
            let mut key_modifier = KeyModifier::None;
            let mut character = '\0';
            unsafe {
                if (ir.event.key_event.unicode_char >= 32) && (ir.event.key_event.key_down == TRUE)
                {
                    let res = char::from_u32(ir.event.key_event.unicode_char as u32);
                    if res.is_some() {
                        character = res.unwrap();
                    }
                }
                if ir.event.key_event.virtual_key_code < 256 {
                    key_code = TRANSLATION_MATRIX[ir.event.key_event.virtual_key_code as usize];
                }

                if (ir.event.key_event.control_key_state & (LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED))
                    != 0
                {
                    key_modifier |= KeyModifier::Alt;
                }
                if (ir.event.key_event.control_key_state & (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED))
                    != 0
                {
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
                        return SystemEvent::None;
                    }
                } else {
                    // check for change in modifier
                    if self.shift_state == key_modifier {
                        // nothing changed --> return
                        return SystemEvent::None;
                    }
                    let old_state = self.shift_state;
                    self.shift_state = key_modifier;
                    return SystemEvent::KeyModifierChanged(KeyModifierChangedEvent {
                        new_state: key_modifier,
                        old_state: old_state,
                    });
                }
            }
            return SystemEvent::KeyPressed(KeyPressedEvent {
                key: Key::new(key_code, key_modifier),
                character,
            });
        }

        // mouse processing
        if ir.event_type == MOUSE_EVENT {
            unsafe {
                // for Windows 11
                if ir.event.mouse_event.event_flags == 0x01 {
                    if ((ir.event.mouse_event.mouse_position.x as i32) == self.last_mouse_x)
                        && ((ir.event.mouse_event.mouse_position.y as i32) == self.last_mouse_y)
                    {
                        return SystemEvent::None;
                    }

                    self.last_mouse_x = ir.event.mouse_event.mouse_position.x as i32;
                    self.last_mouse_y = ir.event.mouse_event.mouse_position.y as i32;
                }

                let x = ir.event.mouse_event.mouse_position.x as i32;
                let y = ir.event.mouse_event.mouse_position.y as i32;
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
                            return SystemEvent::MouseButtonDown(MouseButtonDownEvent {
                                x,
                                y,
                                button,
                            });
                        } else {
                            return SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button });
                        }
                    }
                    DOUBLE_CLICK => {
                        return SystemEvent::MouseDoubleClick(MouseDoubleClickEvent {
                            x,
                            y,
                            button,
                        });
                    }
                    MOUSE_MOVED => {
                        return SystemEvent::MouseMove(MouseMoveEvent { x, y, button });
                    }
                    MOUSE_WHEELED => {
                        if ir.event.mouse_event.button_state >= 0x80000000 {
                            return SystemEvent::MouseWheel(MouseWheelEvent {
                                x,
                                y,
                                direction: MouseWheelDirection::Down,
                            });
                        } else {
                            return SystemEvent::MouseWheel(MouseWheelEvent {
                                x,
                                y,
                                direction: MouseWheelDirection::Up,
                            });
                        }
                    }
                    _ => {
                        return SystemEvent::None;
                    }
                }
            }
        }

        // resize
        if ir.event_type == WINDOW_BUFFER_SIZE_EVENT {
            unsafe {
                return SystemEvent::Resize(super::super::Size::new(
                    ir.event.window_buffer_size_event.width as u32,
                    ir.event.window_buffer_size_event.height as u32,
                ));
            }
        }

        return SystemEvent::None;
    }
}
