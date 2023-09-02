use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::input::MouseWheelDirection;
use crate::prelude::ErrorKind;

use super::super::system_event::KeyModifierChangedEvent;
use super::super::CharFlags;
use super::super::Color;
use super::super::KeyPressedEvent;
use super::super::MouseButtonDownEvent;
use super::super::MouseButtonUpEvent;
use super::super::MouseDoubleClickEvent;
use super::super::MouseMoveEvent;
use super::super::MouseWheelEvent;
use super::super::SystemEvent;
use super::super::Terminal;
use super::constants::*;
use super::structs::*;
use super::winapi;
use crate::graphics::*;
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

fn get_stdin_handle() -> Result<HANDLE, Error> {
    unsafe {
        let h = winapi::GetStdHandle(STD_INPUT_HANDLE);
        if h == INVALID_HANDLE_VALUE {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                format!("Unable to get a valid stdin handle from GetStdHandle WinApi function !"),
            ));
        }
        return Ok(h);
    }
}
fn get_stdout_handle() -> Result<HANDLE, Error> {
    unsafe {
        let h = winapi::GetStdHandle(STD_OUTPUT_HANDLE);
        if h == INVALID_HANDLE_VALUE {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                format!("Unable to get a valid stdout handle from GetStdHandle WinApi function !"),
            ));
        }
        return Ok(h);
    }
}

fn get_console_screen_buffer_info(handle: HANDLE) -> Result<CONSOLE_SCREEN_BUFFER_INFO, Error> {
    unsafe {
        let mut cbuf = CONSOLE_SCREEN_BUFFER_INFO::default();
        if winapi::GetConsoleScreenBufferInfo(handle, &mut cbuf) == FALSE {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                format!("GetConsoleScreenBufferInfo failed to get information on current console !\nWindow code error: {}",winapi::GetLastError()),
            ));
        }
        return Ok(cbuf);
    }
}

pub struct WindowsTerminal {
    stdin_handle: HANDLE,
    stdout_handle: HANDLE,
    size: Size,
    chars: Vec<CHAR_INFO>,
    shift_state: KeyModifier,
    last_mouse_pos: Point,
    visible_region: SMALL_RECT,
    original_mode_flags: u32,
}

impl WindowsTerminal {
    // if size is present -> resize
    // if colors are present --> recolor
    // if font is present --> apply font & size

    pub(crate) fn new(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        let stdin = get_stdin_handle()?;
        let stdout = get_stdout_handle()?;
        let mut original_mode_flags = 0u32;

        unsafe {
            if winapi::GetConsoleMode(stdin, &mut original_mode_flags) == FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!("GetConsoleMode failed to aquire original mode for current console !"),
                ));
            }
            if winapi::SetConsoleMode(stdin, ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS) == FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!("Fail to set current console flags to 'ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS' via SetConsoleMode API.\nWindow code error: {} ",winapi::GetLastError()),
                ));
            }
        }
        let info = get_console_screen_buffer_info(stdout)?;
        if (info.size.x < 1) || (info.size.y < 1) {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                format!(
                    "Invalid console size returned by GetConsoleScreenBufferInfo: width={},height={}\nWindow code error: {}",
                    info.size.x,
                    info.size.y,
                    unsafe { winapi::GetLastError() }
                ),
            ));
        }
        // analyze the visible (window) part
        if (info.window.left > info.window.right) || (info.window.left < 0) {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                format!(
                    "Invalid console visible size returned by GetConsoleScreenBufferInfo: left={},top={},right={},bottom={}\nLeft value should be smaller tham the Right value\nWindow code error: {}",
                    info.window.left,
                    info.window.top,
                    info.window.right,
                    info.window.bottom,
                    unsafe { winapi::GetLastError() }
                )
            ));
        }
        if (info.window.top > info.window.bottom) || (info.window.top < 0) {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                format!(
                    "Invalid console visible size returned by GetConsoleScreenBufferInfo: left={},top={},right={},bottom={}\nTop value should be smaller tham the Bottom value\nWindow code error: {}",
                    info.window.left,
                    info.window.top,
                    info.window.right,
                    info.window.bottom,
                    unsafe { winapi::GetLastError() }
                )
            ));
        }

        let w = (info.window.right as u32) + 1 - (info.window.left as u32);
        let h = (info.window.bottom as u32) + 1 - (info.window.top as u32);

        let mut term = Box::new(WindowsTerminal {
            stdin_handle: stdin,
            stdout_handle: stdout,
            size: Size::new(w, h),
            chars: Vec::with_capacity(1024),
            shift_state: KeyModifier::None,
            last_mouse_pos: Point::new(i32::MAX, i32::MAX),
            visible_region: info.window,
            original_mode_flags: original_mode_flags,
        });
        term.chars
            .resize((term.size.width as usize) * (term.size.height as usize), CHAR_INFO { code: 32, attr: 0 });
        //println!("Init(size:{:?},visible:{:?})", term.size, info.window);
        return Ok(term);
    }
    fn update_size(&mut self) {
        if let Ok(info) = get_console_screen_buffer_info(self.stdout_handle) {
            let w = (info.window.right as u32) + 1 - (info.window.left as u32);
            let h = (info.window.bottom as u32) + 1 - (info.window.top as u32);
            // println!(
            //     "OnResize: \n - received:{:?}\n - actual:w={w},h={h}\n - visible:{:?}",
            //     self.size, info.window
            // );
            self.visible_region = info.window;
            self.chars.resize((w as usize) * (h as usize), CHAR_INFO { code: 32, attr: 0 });
            self.size = Size::new(w, h);
        }
    }
}

impl Terminal for WindowsTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        // safety check --> surface size should be the same as self.width/height size
        if surface.size != self.size {
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
            x: self.size.width as i16,
            y: self.size.height as i16,
        };
        unsafe {
            winapi::WriteConsoleOutputW(self.stdout_handle, self.chars.as_ptr(), sz, COORD { x: 0, y: 0 }, &self.visible_region);
        }
        // update the cursor
        if surface.cursor.is_visible() {
            let pos = COORD {
                x: surface.cursor.x as i16,
                y: surface.cursor.y as i16,
            };
            let info = CONSOLE_CURSOR_INFO { size: 10, visible: TRUE };
            unsafe {
                winapi::SetConsoleCursorPosition(self.stdout_handle, pos);
                winapi::SetConsoleCursorInfo(self.stdout_handle, &info);
            }
        } else {
            let info = CONSOLE_CURSOR_INFO { size: 10, visible: FALSE };
            unsafe {
                winapi::SetConsoleCursorInfo(self.stdout_handle, &info);
            }
        }
    }
    fn get_size(&self) -> Size {
        self.size
    }

    fn get_system_event(&mut self) -> SystemEvent {
        let mut ir = INPUT_RECORD {
            event_type: 0,
            event: WindowsTerminalEvent { extra: 0 },
        };
        let mut nr_read = 0u32;

        unsafe {
            if (winapi::ReadConsoleInputW(self.stdin_handle, &mut ir, 1, &mut nr_read) == FALSE) || (nr_read != 1) {
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
                let x = (ir.event.mouse_event.mouse_position.x as i32) - (self.visible_region.left as i32);
                let y = (ir.event.mouse_event.mouse_position.y as i32) - (self.visible_region.top as i32);
                // for Windows 11
                if ir.event.mouse_event.event_flags == 0x01 {
                    if (x == self.last_mouse_pos.x) && (y == self.last_mouse_pos.y) {
                        return SystemEvent::None;
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
                            return SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button });
                        } else {
                            return SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button });
                        }
                    }
                    DOUBLE_CLICK => {
                        return SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button });
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
            self.update_size();
            return SystemEvent::Resize(self.size);
        }

        return SystemEvent::None;
    }
}
