use crate::input::KeyCode;
use crate::input::KeyModifier;

use super::Attribute;
use super::Color;
use super::Key;
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
const WINDOW_BUFFER_SIZE_EVENT: u16 = 0x0004;

const RIGHT_ALT_PRESSED: u32 = 0x0001;
const LEFT_ALT_PRESSED: u32 = 0x0002;
const RIGHT_CTRL_PRESSED: u32 = 0x0004;
const LEFT_CTRL_PRESSED: u32 = 0x0008;
const SHIFT_PRESSED: u32 = 0x0010;

const translation_matrix: [KeyCode; 256] = [
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
struct CHAR_INFO {
    code: u16,
    attr: u16,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
struct KEY_EVENT_RECORD {
    bKeyDown: BOOL,
    wRepeatCount: u16,
    wVirtualKeyCode: u16,
    wVirtualScanCode: u16,
    UnicodeChar: u16,
    dwControlKeyState: u32,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Copy, Clone)]
union WindowsTerminalEvent {
    KeyEvent: KEY_EVENT_RECORD,
    WindowBufferSizeEvent: SIZE,
    extra: u32,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Copy, Clone)]
struct INPUT_RECORD {
    EventType: u16,
    Event: WindowsTerminalEvent,
}

extern "system" {
    #[warn(non_camel_case_types)]
    fn GetStdHandle(v: i32) -> HANDLE;
    #[warn(non_camel_case_types)]
    fn SetConsoleCursorPosition(handle: HANDLE, pos: COORD) -> BOOL;
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

fn set_console_cursor_pos(handle: u32, x: i32, y: i32) -> bool {
    let pos = COORD {
        x: x as i16,
        y: y as i16,
    };
    unsafe { SetConsoleCursorPosition(handle, pos) != 0 }
}

fn update_screen(handle: u32, buf: &[CHAR_INFO], width: u32, height: u32) {
    let sz = COORD {
        x: width as i16,
        y: height as i16,
    };
    let start = COORD { x: 0, y: 0 };
    let region = SMALL_RECT {
        left: 0,
        top: 0,
        right: sz.x,
        bottom: sz.y,
    };
    unsafe {
        WriteConsoleOutputW(handle, buf.as_ptr(), sz, start, &region);
    }
}

pub struct WindowsTerminal {
    stdin_handle: HANDLE,
    stdout_handle: HANDLE,
    width: u32,
    height: u32,
    chars: Vec<CHAR_INFO>,
    shift_state: KeyModifier,
}

impl WindowsTerminal {
    pub fn create() -> Option<Box<WindowsTerminal>> {
        let stdin = get_handle(STD_INPUT_HANDLE)?;
        let stdout = get_handle(STD_OUTPUT_HANDLE)?;
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
            if ch.attribute.contains(Attribute::Underline) {
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
    }
    fn get_width(&self) -> u32 {
        return self.width;
    }
    fn get_height(&self) -> u32 {
        return self.height;
    }
    fn get_system_event(&mut self) -> SystemEvent {
        let mut ir = INPUT_RECORD {
            EventType: 0,
            Event: WindowsTerminalEvent { extra: 0 },
        };
        let mut nr_read = 0u32;

        unsafe {
            if (ReadConsoleInputW(self.stdin_handle, &mut ir, 1, &mut nr_read) == FALSE)
                || (nr_read != 1)
            {
                return SystemEvent::None;
            }
        }

        // Key processing
        if ir.EventType == KEY_EVENT {
            let mut key = Key::default();
            unsafe {
                if (ir.Event.KeyEvent.UnicodeChar >= 32) && (ir.Event.KeyEvent.bKeyDown == TRUE) {
                    let res = char::from_u32(ir.Event.KeyEvent.UnicodeChar as u32);
                    if res.is_some() {
                        key.character = res.unwrap();
                    }
                }
                if ir.Event.KeyEvent.wVirtualKeyCode < 256 {
                    key.code = translation_matrix[ir.Event.KeyEvent.wVirtualKeyCode as usize];
                }

                if (ir.Event.KeyEvent.dwControlKeyState & (LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED))
                    != 0
                {
                    key.modifier |= KeyModifier::Alt;
                }
                if (ir.Event.KeyEvent.dwControlKeyState & (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED))
                    != 0
                {
                    key.modifier |= KeyModifier::Ctrl;
                }
                if (ir.Event.KeyEvent.dwControlKeyState & SHIFT_PRESSED) != 0 {
                    key.modifier |= KeyModifier::Shift;
                }

                // if ALT or CTRL are pressed, clear the ascii code
                if key
                    .modifier
                    .contains_one(KeyModifier::Alt | KeyModifier::Ctrl)
                {
                    key.character = 0 as char;
                }
                if key.has_key() {
                    if ir.Event.KeyEvent.bKeyDown == FALSE {
                        // key is up (no need to send)
                        return SystemEvent::None;
                    }
                } else {
                    // check for change in modifier
                    if self.shift_state == key.modifier {
                        // nothing changed --> return
                        return SystemEvent::None;
                    }
                    self.shift_state = key.modifier;
                }
            }
            return SystemEvent::Key(key);
        }

        // resize
        if ir.EventType == WINDOW_BUFFER_SIZE_EVENT {
            unsafe {
                return SystemEvent::Resize(super::Size::new(
                    ir.Event.WindowBufferSizeEvent.width as u32,
                    ir.Event.WindowBufferSizeEvent.height as u32,
                ));
            }
        }

        return SystemEvent::None;
    }
}
