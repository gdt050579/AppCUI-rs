use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::input::MouseEvent;
use crate::input::MouseEventType;
use crate::input::MouseWheelDirection;

use super::CharFlags;
use super::Color;
use super::KeyEvent;
use super::Surface;
use super::SystemEvent;
use super::Terminal;

type HANDLE = u32;
type BOOL = u32;

const INVALID_HANDLE_VALUE: u32 = 0xFFFFFFFF;
const STD_INPUT_HANDLE: i32 = -10;
const STD_OUTPUT_HANDLE: i32 = -11;
const FALSE: u32 = 0;
const TRUE: u32 = 1;
const COMMON_LVB_UNDERSCORE: u16 = 0x8000;
const KEY_EVENT: u16 = 0x0001;
const MOUSE_EVENT: u16 = 0x0002;
const WINDOW_BUFFER_SIZE_EVENT: u16 = 0x0004;
const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 0x0001;
const RIGHTMOST_BUTTON_PRESSED: u32 = 0x0002;
const RIGHT_ALT_PRESSED: u32 = 0x0001;
const DOUBLE_CLICK: u32 = 0x0002;
const MOUSE_MOVED: u32 = 0x0001;
const MOUSE_WHEELED: u32 = 0x0004;
const LEFT_ALT_PRESSED: u32 = 0x0002;
const RIGHT_CTRL_PRESSED: u32 = 0x0004;
const LEFT_CTRL_PRESSED: u32 = 0x0008;
const SHIFT_PRESSED: u32 = 0x0010;
const ENABLE_WINDOW_INPUT: u32 = 0x0008;
const ENABLE_MOUSE_INPUT: u32 = 0x0010;
const ENABLE_EXTENDED_FLAGS: u32 = 0x0080;

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

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct SIZE {
    width: u16,
    height: u16,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct COORD {
    x: i16,
    y: i16,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct SMALL_RECT {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}
#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct CONSOLE_SCREEN_BUFFER_INFO {
    size: COORD,
    cursor_pos: COORD,
    _attributes: u16,
    _window: SMALL_RECT,
    _max_size: COORD,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct CONSOLE_CURSOR_INFO {
    size: u32,
    visible: BOOL,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct CHAR_INFO {
    code: u16,
    attr: u16,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
struct KEY_EVENT_RECORD {
    key_down: BOOL,
    repeat_count: u16,
    virtual_key_code: u16,
    virtual_scan_code: u16,
    unicode_char: u16,
    control_key_state: u32,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
struct MOUSE_EVENT_RECORD {
    mouse_position: COORD,
    button_state: u32,
    control_key_state: u32,
    event_flags: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
union WindowsTerminalEvent {
    key_event: KEY_EVENT_RECORD,
    mouse_event: MOUSE_EVENT_RECORD,
    window_buffer_size_event: SIZE,
    extra: u32,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Copy, Clone)]
struct INPUT_RECORD {
    event_type: u16,
    event: WindowsTerminalEvent,
}

extern "system" {
    #[warn(non_camel_case_types)]
    fn GetStdHandle(v: i32) -> HANDLE;
    #[warn(non_camel_case_types)]
    fn SetConsoleCursorPosition(handle: HANDLE, pos: COORD) -> BOOL;
    #[warn(non_camel_case_types)]
    fn SetConsoleCursorInfo(handle: HANDLE, info: &CONSOLE_CURSOR_INFO) -> BOOL;
    #[warn(non_camel_case_types)]
    fn GetConsoleMode(handle: HANDLE, mode_flags: &mut u32) -> BOOL;
    #[warn(non_camel_case_types)]
    fn SetConsoleMode(handle: HANDLE, mode_flags: u32) -> BOOL;
    #[warn(non_camel_case_types)]
    fn WriteConsoleOutputW(
        handle: HANDLE,
        lpBuffer: *const CHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpWriteRegion: &SMALL_RECT,
    );
    #[warn(non_camel_case_types)]
    fn GetConsoleScreenBufferInfo(
        handle: HANDLE,
        lpConsoleScreenBufferInfo: &mut CONSOLE_SCREEN_BUFFER_INFO,
    ) -> BOOL;

    #[warn(non_camel_case_types)]
    fn ReadConsoleInputW(
        handle: HANDLE,
        lpBuffer: *mut INPUT_RECORD,
        nLength: u32,
        lpNumberOfEventsRead: &mut u32,
    ) -> BOOL;

}

fn get_handle(handle_id: i32) -> Option<u32> {
    unsafe {
        let h = GetStdHandle(handle_id);
        if h == INVALID_HANDLE_VALUE {
            return None;
        }
        return Some(h);
    }
}

fn get_console_screen_buffer_info(handle: HANDLE) -> Option<CONSOLE_SCREEN_BUFFER_INFO> {
    unsafe {
        let mut cbuf = CONSOLE_SCREEN_BUFFER_INFO::default();
        if GetConsoleScreenBufferInfo(handle, &mut cbuf) == FALSE {
            return None;
        }
        return Some(cbuf);
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
    pub fn create() -> Option<Box<WindowsTerminal>> {
        let stdin = get_handle(STD_INPUT_HANDLE)?;
        let stdout = get_handle(STD_OUTPUT_HANDLE)?;
        let mut original_mode_flags = 0u32;
        unsafe {
            if GetConsoleMode(stdin, &mut original_mode_flags) == FALSE {
                return None;
            }
            if SetConsoleMode(
                stdin,
                ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS,
            ) == FALSE
            {
                return None;
            }
        }
        let info = get_console_screen_buffer_info(stdout)?;
        if (info.size.x < 1) || (info.size.y < 1) {
            return None;
        }
        let mut term = Box::new(WindowsTerminal {
            stdin_handle: stdin,
            stdout_handle: stdout,
            width: info.size.x as u32,
            height: info.size.y as u32,
            chars: Vec::with_capacity(100),
            shift_state: KeyModifier::None,
            last_mouse_x: i32::MAX,
            last_mouse_y: i32::MAX,
            original_mode_flags: original_mode_flags,
        });
        term.chars.resize(
            (term.width as usize) * (term.height as usize),
            CHAR_INFO { code: 32, attr: 0 },
        );
        return Some(term);
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
            WriteConsoleOutputW(self.stdout_handle, self.chars.as_ptr(), sz, start, &region);
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
                SetConsoleCursorPosition(self.stdout_handle, pos);
                SetConsoleCursorInfo(self.stdout_handle, &info);
            }
        } else {
            let info = CONSOLE_CURSOR_INFO {
                size: 10,
                visible: FALSE,
            };
            unsafe {
                SetConsoleCursorInfo(self.stdout_handle, &info);
            }
        }
    }
    fn get_width(&self) -> u32 {
        return self.width;
    }
    fn get_height(&self) -> u32 {
        return self.height;
    }
    fn get_system_event(&mut self) -> SystemEvent {
        let mut ir = INPUT_RECORD {
            event_type: 0,
            event: WindowsTerminalEvent { extra: 0 },
        };
        let mut nr_read = 0u32;

        unsafe {
            if (ReadConsoleInputW(self.stdin_handle, &mut ir, 1, &mut nr_read) == FALSE)
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
                    return SystemEvent::KeyEvent(KeyEvent::new_modifier_changed(
                        old_state,
                        key_modifier,
                    ));
                }
            }
            return SystemEvent::KeyEvent(KeyEvent::new_key_pressed(
                key_code,
                key_modifier,
                character,
            ));
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

                let mut mouse_event = MouseEvent::default();
                mouse_event.x = ir.event.mouse_event.mouse_position.x as i32;
                mouse_event.y = ir.event.mouse_event.mouse_position.y as i32;

                if (ir.event.mouse_event.button_state & FROM_LEFT_1ST_BUTTON_PRESSED) != 0 {
                    mouse_event.button = MouseButton::Left;
                } else if (ir.event.mouse_event.button_state & RIGHTMOST_BUTTON_PRESSED) != 0 {
                    mouse_event.button = MouseButton::Right;
                } else if ir.event.mouse_event.button_state > 0 {
                    mouse_event.button = MouseButton::Center;
                }

                match ir.event.mouse_event.event_flags {
                    0 => {
                        if ir.event.mouse_event.button_state != 0 {
                            mouse_event.event = MouseEventType::ButtonDown;
                        } else {
                            mouse_event.event = MouseEventType::ButtonUp;
                        }
                    }
                    DOUBLE_CLICK => {
                        mouse_event.event = MouseEventType::DoubleClick;
                    }
                    MOUSE_MOVED => {
                        mouse_event.event = MouseEventType::Move;
                    }
                    MOUSE_WHEELED => {
                        mouse_event.event = MouseEventType::Wheel;
                        if ir.event.mouse_event.button_state >= 0x80000000 {
                            mouse_event.wheel_direction = MouseWheelDirection::Down;
                        } else {
                            mouse_event.wheel_direction = MouseWheelDirection::Up;
                        }
                    }
                    _ => {
                        return SystemEvent::None;
                    }
                }

                return SystemEvent::MouseEvent(mouse_event);
            }
        }

        // resize
        if ir.event_type == WINDOW_BUFFER_SIZE_EVENT {
            unsafe {
                return SystemEvent::Resize(super::Size::new(
                    ir.event.window_buffer_size_event.width as u32,
                    ir.event.window_buffer_size_event.height as u32,
                ));
            }
        }

        return SystemEvent::None;
    }
}
