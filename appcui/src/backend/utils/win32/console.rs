use std::sync::Arc;
use std::sync::Mutex;

use super::{api, constants, structs};
use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::input::MouseWheelDirection;
use crate::prelude::Point;
use crate::prelude::Size;
use crate::system::Error;
use crate::system::ErrorKind;
use crate::system::KeyModifierChangedEvent;
use crate::system::KeyPressedEvent;
use crate::system::MouseButtonDownEvent;
use crate::system::MouseButtonUpEvent;
use crate::system::MouseDoubleClickEvent;
use crate::system::MouseMoveEvent;
use crate::system::MouseWheelEvent;
use crate::system::SystemEvent;

#[derive(Clone)]
pub(crate) struct Console {
    stdin: structs::HANDLE,
    stdout: structs::HANDLE,
    stdin_original_mode_flags: u32,
    stdout_original_mode_flags: u32,
    size: Size,
    visible_region: structs::SMALL_RECT,
    shift_state: KeyModifier,
    last_mouse_pos: Point,
    shared_visible_region: Arc<Mutex<structs::SMALL_RECT>>,
}

impl Console {
    pub(crate) fn new(builder: &crate::system::Builder, vt: bool) -> Result<Self, Error> {
        unsafe {
            let h_stdin = api::GetStdHandle(constants::STD_INPUT_HANDLE);
            if h_stdin == constants::INVALID_HANDLE_VALUE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    "Unable to get a valid stdin handle from GetStdHandle WinApi function !".to_string(),
                ));
            }
            let h_stdout = api::GetStdHandle(constants::STD_OUTPUT_HANDLE);
            if h_stdout == constants::INVALID_HANDLE_VALUE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    "Unable to get a valid stdout handle from GetStdHandle WinApi function !".to_string(),
                ));
            }
            if let Some(new_size) = builder.size {
                Self::resize(new_size, h_stdout)?;
            }
            if let Some(title) = &builder.title {
                Self::set_title(title)?;
            }

            let mut stdin_original_mode_flags = 0u32;
            if api::GetConsoleMode(h_stdin, &mut stdin_original_mode_flags) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    "GetConsoleMode failed to aquire original mode for current console stdin !".to_string(),
                ));
            }
            let mut stdout_original_mode_flags = 0u32;
            if api::GetConsoleMode(h_stdout, &mut stdout_original_mode_flags) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    "GetConsoleMode failed to aquire original mode for current console stdout !".to_string(),
                ));
            }

            if api::SetConsoleMode(
                h_stdin,
                constants::ENABLE_WINDOW_INPUT | constants::ENABLE_MOUSE_INPUT | constants::ENABLE_EXTENDED_FLAGS,
            ) == constants::FALSE
            {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!("Fail to set current console flags to 'ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS' via SetConsoleMode API.\nWindow code error: {} ",api::GetLastError()),
                ));
            }
            if vt && api::SetConsoleMode(h_stdout, stdout_original_mode_flags | constants::ENABLE_VIRTUAL_TERMINAL_PROCESSING) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "Fail to set current console flags to 'ENABLE_VIRTUAL_TERMINAL_PROCESSING' via SetConsoleMode API.\nWindow code error: {} ",
                        api::GetLastError()
                    ),
                ));
            }

            let mut info = Self::screen_buffer_info(h_stdout)?;
            if (info.size.x < 1) || (info.size.y < 1) {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "Invalid console size returned by GetConsoleScreenBufferInfo: width={},height={}\nWindow code error: {}",
                        info.size.x,
                        info.size.y,
                        api::GetLastError()
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
                        api::GetLastError()
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
                        api::GetLastError()
                    )
                ));
            }
            let w = (info.window.right as u32) + 1 - (info.window.left as u32);
            let h = (info.window.bottom as u32) + 1 - (info.window.top as u32);
            if !builder.use_color_schema {
                // set the color schema to match AppCUI colors
                info.color_table = constants::APPCUI_COLOR_SCEHMA;
                if api::SetConsoleScreenBufferInfoEx(h_stdout, &info) == constants::FALSE {
                    return Err(Error::new(
                        ErrorKind::InitializationFailure,
                        format!(
                            "SetConsoleScreenBufferInfoEx failed to sey a new color schema on current console !\nWindow code error: {}",
                            api::GetLastError()
                        ),
                    ));                    
                }
            }

            Ok(Self {
                stdin: h_stdin,
                stdout: h_stdout,
                size: Size::new(w, h),
                stdin_original_mode_flags,
                stdout_original_mode_flags,
                visible_region: info.window,
                shift_state: KeyModifier::None,
                last_mouse_pos: Point::new(i32::MAX, i32::MAX),
                shared_visible_region: Arc::new(Mutex::new(info.window)),
            })
        }
    }
    #[inline(always)]
    pub(crate) fn stdin(&self) -> structs::HANDLE {
        self.stdin
    }
    #[inline(always)]
    pub(crate) fn stdout(&self) -> structs::HANDLE {
        self.stdout
    }
    #[inline(always)]
    pub(crate) fn size(&self) -> Size {
        self.size
    }
    #[inline(always)]
    pub(crate) fn visible_region(&self) -> structs::SMALL_RECT {
        self.visible_region
    }

    #[inline(always)]
    fn process_key_event(&mut self, evnt: &structs::KEY_EVENT_RECORD) -> Option<SystemEvent> {
        let mut key_code = KeyCode::None;
        let mut key_modifier = KeyModifier::None;
        let mut character = '\0';
        if (evnt.unicode_char >= 32) && (evnt.key_down == constants::TRUE) {
            if let Some(ch) = char::from_u32(evnt.unicode_char as u32) {
                character = ch;
            }
        }
        if evnt.virtual_key_code < 256 {
            key_code = constants::TRANSLATION_MATRIX[evnt.virtual_key_code as usize];
        }

        if (evnt.control_key_state & (constants::LEFT_ALT_PRESSED | constants::RIGHT_ALT_PRESSED)) != 0 {
            key_modifier |= KeyModifier::Alt;
        }
        if (evnt.control_key_state & (constants::LEFT_CTRL_PRESSED | constants::RIGHT_CTRL_PRESSED)) != 0 {
            key_modifier |= KeyModifier::Ctrl;
        }
        if (evnt.control_key_state & constants::SHIFT_PRESSED) != 0 {
            key_modifier |= KeyModifier::Shift;
        }

        // if ALT or CTRL are pressed, clear the ascii code
        if key_modifier.contains_one(KeyModifier::Alt | KeyModifier::Ctrl) {
            character = '\0';
        }
        if (key_code != KeyCode::None) || (character != '\0') {
            if evnt.key_down == constants::FALSE {
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

        Some(SystemEvent::KeyPressed(KeyPressedEvent {
            key: Key::new(key_code, key_modifier),
            character,
        }))
    }

    #[inline(always)]
    fn process_mouse_event(&mut self, evnt: &structs::MOUSE_EVENT_RECORD) -> Option<SystemEvent> {
        let origin = Point::new(self.visible_region().left as i32, self.visible_region().top as i32);
        let x = (evnt.mouse_position.x as i32) - origin.x;
        let y = (evnt.mouse_position.y as i32) - origin.y;
        // for Windows 11
        if evnt.event_flags == 0x01 {
            if (x == self.last_mouse_pos.x) && (y == self.last_mouse_pos.y) {
                return None;
            }

            self.last_mouse_pos = Point::new(x, y);
        }

        let button = {
            if (evnt.button_state & constants::FROM_LEFT_1ST_BUTTON_PRESSED) != 0 {
                MouseButton::Left
            } else if (evnt.button_state & constants::RIGHTMOST_BUTTON_PRESSED) != 0 {
                MouseButton::Right
            } else if evnt.button_state > 0 {
                MouseButton::Center
            } else {
                MouseButton::None
            }
        };

        match evnt.event_flags {
            0 => {
                if evnt.button_state != 0 {
                    Some(SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button }))
                } else {
                    Some(SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button }))
                }
            }
            constants::DOUBLE_CLICK => Some(SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button })),
            constants::MOUSE_MOVED => Some(SystemEvent::MouseMove(MouseMoveEvent { x, y, button })),
            constants::MOUSE_HWHEELED => {
                //println!("HWHEEL {}", self.button_state);
                if evnt.button_state >= 0x80000000 {
                    Some(SystemEvent::MouseWheel(MouseWheelEvent {
                        x,
                        y,
                        direction: MouseWheelDirection::Left,
                    }))
                } else {
                    Some(SystemEvent::MouseWheel(MouseWheelEvent {
                        x,
                        y,
                        direction: MouseWheelDirection::Right,
                    }))
                }
            }
            constants::MOUSE_WHEELED => {
                if evnt.button_state >= 0x80000000 {
                    Some(SystemEvent::MouseWheel(MouseWheelEvent {
                        x,
                        y,
                        direction: MouseWheelDirection::Down,
                    }))
                } else {
                    Some(SystemEvent::MouseWheel(MouseWheelEvent {
                        x,
                        y,
                        direction: MouseWheelDirection::Up,
                    }))
                }
            }
            _ => None,
        }
    }

    #[inline(always)]
    pub(crate) fn read_event(&mut self) -> Option<SystemEvent> {
        let mut ir = structs::INPUT_RECORD {
            event_type: 0,
            event: structs::WindowsTerminalEvent { extra: 0 },
        };
        let mut nr_read = 0u32;

        unsafe {
            if (api::ReadConsoleInputW(self.stdin(), &mut ir, 1, &mut nr_read) == constants::FALSE) || (nr_read != 1) {
                return None;
            }
        }

        // Key processings
        if ir.event_type == constants::KEY_EVENT {
            return self.process_key_event(unsafe { &ir.event.key_event });
        }

        // mouse processing
        if ir.event_type == constants::MOUSE_EVENT {
            return self.process_mouse_event(unsafe { &ir.event.mouse_event });
        }

        // resize
        if ir.event_type == constants::WINDOW_BUFFER_SIZE_EVENT {
            if let Ok(info) = Self::screen_buffer_info(self.stdout) {
                let w = (info.window.right as u32) + 1 - (info.window.left as u32);
                let h = (info.window.bottom as u32) + 1 - (info.window.top as u32);
                self.visible_region = info.window;
                if let Ok(mut shared_data) = self.shared_visible_region.lock() {
                    *shared_data = info.window;
                }
                return Some(SystemEvent::Resize(Size::new(w, h)));
            }
        }

        None
    }

    pub(crate) fn on_resize(&mut self, new_size: Size) {
        self.size = new_size;
        if let Ok(data) = self.shared_visible_region.lock() {
            self.visible_region = *data;
            //println!("OnResize: -> region: {:?}",self.visible_region);
        }
    }

    pub(crate) fn on_close(&mut self) {
        unsafe {
            let _ = api::SetConsoleMode(self.stdin, self.stdin_original_mode_flags);
            let _ = api::SetConsoleMode(self.stdout, self.stdout_original_mode_flags);
        }
    }

    fn screen_buffer_info(stdout: structs::HANDLE) -> Result<structs::CONSOLE_SCREEN_BUFFER_INFOEX, Error> {
        unsafe {
            let mut cbuf = structs::CONSOLE_SCREEN_BUFFER_INFOEX {
                structure_size: 0x60,
                ..Default::default()
            };
            if api::GetConsoleScreenBufferInfoEx(stdout, &mut cbuf) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "GetConsoleScreenBufferInfoEx failed to get information on current console !\nWindow code error: {}",
                        api::GetLastError()
                    ),
                ));
            }
            Ok(cbuf)
        }
    }

    fn string_to_wide(text: &str) -> Result<Vec<u16>, Error> {
        let mut result: Vec<u16> = Vec::with_capacity(text.len() + 1);
        for c in text.chars() {
            let unicode_id = c as u32;
            if unicode_id >= 0xFFFF {
                return Err(Error::new(
                    ErrorKind::InvalidParameter,
                    format!("Fail convert the string '{text}' to windows WTF-16"),
                ));
            }
            if unicode_id == 0 {
                return Err(Error::new(
                    ErrorKind::InvalidParameter,
                    format!("Found NULL (\\0 character) in title '{text}'. This can not be accurately translated into windows WTF-16 that is NULL terminated !"),
                ));
            }
            result.push(unicode_id as u16);
        }
        result.push(0);
        Ok(result)
    }
    pub(crate) fn set_title(title: &str) -> Result<(), Error> {
        let title_wtf16 = Self::string_to_wide(title)?;

        unsafe {
            if api::SetConsoleTitleW(title_wtf16.as_ptr()) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "SetConsoleTitleW failed while attemting change the title of the console to '{}'. Error Code = {} !",
                        title,
                        api::GetLastError()
                    ),
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn resize(size: Size, stdout: structs::HANDLE) -> Result<(), Error> {
        // sanity check
        if (size.width > 30000) || (size.width < 5) {
            return Err(Error::new(
                ErrorKind::InvalidParameter,
                format!(
                    "The width paramater for console resize shoule be between 5 and 30000. Current value is invalid: 'width={}'",
                    size.width
                ),
            ));
        }
        if (size.height > 30000) || (size.height < 5) {
            return Err(Error::new(
                ErrorKind::InvalidParameter,
                format!(
                    "The height paramater for console resize shoule be between 5 and 30000. Current value is invalid: 'height={}'",
                    size.height
                ),
            ));
        }
        let window_size = structs::SMALL_RECT {
            left: 0,
            top: 0,
            right: size.width as i16 - 1,
            bottom: size.height as i16 - 1,
        };
        unsafe {
            if api::SetConsoleWindowInfo(stdout, constants::TRUE, &window_size) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "SetConsoleWindowsInfo failed while attemting to resize console to {}x{}. Error Code = {} !",
                        size.width,
                        size.height,
                        api::GetLastError()
                    ),
                ));
            }
        }
        let buffer_size = structs::COORD {
            x: size.width as i16,
            y: size.height as i16,
        };
        unsafe {
            if api::SetConsoleScreenBufferSize(stdout, buffer_size) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "SetConsoleScreenBufferSize failed while attemting to resize console buttef to {}x{}. Error Code = {} !",
                        size.width,
                        size.height,
                        api::GetLastError()
                    ),
                ));
            }
        }
        Ok(())
    }
}
