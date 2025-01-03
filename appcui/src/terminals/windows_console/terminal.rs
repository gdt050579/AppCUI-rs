use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

use crate::prelude::ErrorKind;
use crate::utils::GlyphParser;

use super::super::SystemEvent;
use super::super::SystemEventReader;
use super::super::Terminal;
use super::constants::*;
use super::input::Input;
use super::structs::*;
use super::utils;
use super::winapi;
use crate::graphics::*;
use crate::system::Error;

pub struct WindowsTerminal {
    //stdin_handle: HANDLE, // to be moved
    stdout: HANDLE,
    size: Size,
    chars: Vec<CHAR_INFO>,
    //shift_state: KeyModifier, // to be moved
    //last_mouse_pos: Point, // to be moved
    visible_region: SMALL_RECT, // to be moved ?!?
    _original_mode_flags: u32,
    shared_visible_region: Arc<Mutex<SMALL_RECT>>,
}

impl WindowsTerminal {
    // if size is present -> resize
    // if colors are present --> recolor
    // if font is present --> apply font & size

    fn string_to_wide(text: &str) -> Result<Vec<u16>, Error> {
        let mut result: Vec<u16> = Vec::with_capacity(text.len() + 1);
        for c in text.chars() {
            let unicode_id = c as u32;
            if unicode_id >= 0xFFFF {
                return Err(Error::new(
                    ErrorKind::InvalidParameter,
                    format!("Fail convert the string '{}' to windows WTF-16", text),
                ));
            }
            if unicode_id == 0 {
                return Err(Error::new(
                    ErrorKind::InvalidParameter,
                    format!("Found NULL (\\0 character) in title '{}'. This can not be accurately translated into windows WTF-16 that is NULL terminated !", text),
                ));
            }
            result.push(unicode_id as u16);
        }
        result.push(0);
        Ok(result)
    }
    fn set_title(title: &str) -> Result<(), Error> {
        let title_wtf16 = WindowsTerminal::string_to_wide(title)?;

        unsafe {
            if winapi::SetConsoleTitleW(title_wtf16.as_ptr()) == FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "SetConsoleTitleW failed while attemting change the title of the console to '{}'. Error Code = {} !",
                        title,
                        winapi::GetLastError()
                    ),
                ));
            }
        }
        Ok(())
    }
    fn resize(size: Size, stdout: HANDLE) -> Result<(), Error> {
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
        let window_size = SMALL_RECT {
            left: 0,
            top: 0,
            right: size.width as i16 - 1,
            bottom: size.height as i16 - 1,
        };
        unsafe {
            if winapi::SetConsoleWindowInfo(stdout, TRUE, &window_size) == FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "SetConsoleWindowsInfo failed while attemting to resize console to {}x{}. Error Code = {} !",
                        size.width,
                        size.height,
                        winapi::GetLastError()
                    ),
                ));
            }
        }
        let buffer_size = COORD {
            x: size.width as i16,
            y: size.height as i16,
        };
        unsafe {
            if winapi::SetConsoleScreenBufferSize(stdout, buffer_size) == FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "SetConsoleScreenBufferSize failed while attemting to resize console buttef to {}x{}. Error Code = {} !",
                        size.width,
                        size.height,
                        winapi::GetLastError()
                    ),
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        let stdin = utils::get_stdin_handle()?;
        let stdout = utils::get_stdout_handle()?;
        let mut original_mode_flags = 0u32;

        if let Some(new_size) = builder.size {
            WindowsTerminal::resize(new_size, stdout)?;
        }
        if let Some(title) = &builder.title {
            WindowsTerminal::set_title(title)?;
        }

        unsafe {
            if winapi::GetConsoleMode(stdin, &mut original_mode_flags) == FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    "GetConsoleMode failed to aquire original mode for current console !".to_string(),
                ));
            }
            if winapi::SetConsoleMode(stdin, ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS) == FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!("Fail to set current console flags to 'ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS' via SetConsoleMode API.\nWindow code error: {} ",winapi::GetLastError()),
                ));
            }
        }
        let info = utils::get_console_screen_buffer_info(stdout)?;
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

        // // create the comunication channel
        // let (sender, receiver) = mpsc::channel::<INPUT_RECORD>();
        // // create the thread that will read the input
        // let event_sender = sender.clone();
        // std::thread::spawn(move || {
        //     let mut ir = INPUT_RECORD {
        //         event_type: 0,
        //         event: WindowsTerminalEvent { extra: 0 },
        //     };
        //     loop {
        //         ir.event_type = 0;
        //         let mut nr_read = 0u32;

        //         unsafe {
        //             if (winapi::ReadConsoleInputW(stdin, &mut ir, 1, &mut nr_read) == TRUE) && (nr_read == 1) {
        //                 if event_sender.send(ir).is_err() {
        //                     break;
        //                 }
        //             }
        //         }
        //     }
        // });

        let mut term = WindowsTerminal {
            //stdin_handle: stdin,
            stdout,
            size: Size::new(w, h),
            chars: Vec::with_capacity(1024),
            //shift_state: KeyModifier::None,
            //last_mouse_pos: Point::new(i32::MAX, i32::MAX),
            visible_region: info.window,
            _original_mode_flags: original_mode_flags,
            shared_visible_region: Arc::new(Mutex::new(info.window)),
        };
        //println!("Start region: {:?}",term.visible_region);
        term.chars.resize(
            (term.size.width as usize) * (term.size.height as usize) * 2,
            CHAR_INFO { code: 32, attr: 0 },
        );
        // start the event thread
        Input::new(stdin, stdout, info.window, term.shared_visible_region.clone()).start(sender);
        // all good - start the sender thread
        Ok(term)
    }
    // fn update_size(&mut self) {
    //     // if let Ok(info) = get_console_screen_buffer_info(self.stdout_handle) {
    //     //     let w = (info.window.right as u32) + 1 - (info.window.left as u32);
    //     //     let h = (info.window.bottom as u32) + 1 - (info.window.top as u32);
    //     //     // println!(
    //     //     //     "OnResize: \n - received:{:?}\n - actual:w={w},h={h}\n - visible:{:?}",
    //     //     //     self.size, info.window
    //     //     // );
    //     //     self.visible_region = info.window;
    //     //     self.chars.resize((w as usize) * (h as usize) * 2, CHAR_INFO { code: 32, attr: 0 });
    //     //     self.size = Size::new(w, h);
    //     // }
    // }
}

impl Terminal for WindowsTerminal {
    fn on_resize(&mut self, new_size: Size) {
        let w = new_size.width as usize;
        let h = new_size.height as usize;
        self.chars.resize(w * h * 2, CHAR_INFO { code: 32, attr: 0 });
        self.size = new_size;
        if let Ok(data) = self.shared_visible_region.lock() {
            self.visible_region = *data;
            //println!("OnResize: -> region: {:?}",self.visible_region);
        }
    }
    fn update_screen(&mut self, surface: &Surface) {
        // println!("Update the screen: capacity: {}, size: {:?}, region: {:?}, surface_size: {:?}",self.chars.len(),self.size,self.visible_region,surface.size);
        // safety check --> surface size should be the same as self.width/height size
        if surface.size != self.size {
            panic!("Invalid size !!!");
        }
        // check if allocated space si twice the size (to account for surrogates)
        if self.chars.len() != (self.size.width as usize) * (self.size.height as usize) * 2 {
            panic!("Invalid size for CHAR_INFO buffer !!!");
        }

        // copy surface into CHAR_INFO
        let mut pos = 0;
        let mut x = 0;
        let mut y = 0;
        let mut start_y = 0;
        let w = surface.size.width as i32;
        let mut surrogate_used = 0;
        for ch in surface.chars.iter() {
            let screen_char = &mut (self.chars[pos]);
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

            match ch.code as u32 {
                0 => {
                    screen_char.code = 32;
                    if surrogate_used > 0 {
                        surrogate_used -= 1;
                    } else {
                        pos += 1;
                    }
                }
                0x0001..=0xD7FF => {
                    screen_char.code = ch.code as u16;
                    if surrogate_used > 0 {
                        surrogate_used -= 1;
                    } else {
                        pos += 1;
                    }
                }
                0x10000..=0x10FFFF => {
                    // surrogate pair
                    let v = (ch.code as u32) - 0x10000;
                    let h = v / 0x400 + 0xD800;
                    let l = v % 0x400 + 0xDC00;
                    screen_char.code = h as u16;
                    let attr = screen_char.attr;
                    pos += 1;
                    let screen_char = &mut (self.chars[pos]);
                    screen_char.attr = attr;
                    screen_char.code = l as u16;
                    pos += 1;
                    surrogate_used += 1;
                }
                _ => {
                    // unknown character --> use '?'
                    screen_char.code = b'?' as u16;
                    if surrogate_used > 0 {
                        surrogate_used -= 1;
                    } else {
                        pos += 1;
                    }
                }
            }
            x += 1;
            if x >= w {
                x = 0;
                y += 1;
                if surrogate_used > 0 {
                    let sz = COORD { x: w as i16, y: y - start_y };
                    let vis_region = SMALL_RECT {
                        left: self.visible_region.left,
                        top: self.visible_region.top + start_y,
                        right: self.visible_region.right,
                        bottom: self.visible_region.top + y - 1,
                    };
                    unsafe {
                        winapi::WriteConsoleOutputW(self.stdout, self.chars.as_ptr(), sz, COORD { x: 0, y: 0 }, &vis_region);
                    }
                    pos = 0;
                    start_y = y;
                }
                surrogate_used = 0;
            }
        }
        if start_y == 0 {
            // no surrogates --> write the entire buffer
            let sz = COORD {
                x: self.size.width as i16,
                y: self.size.height as i16,
            };
            unsafe {
                winapi::WriteConsoleOutputW(self.stdout, self.chars.as_ptr(), sz, COORD { x: 0, y: 0 }, &self.visible_region);
            }
        } else if start_y < y {
            let sz = COORD { x: w as i16, y: y - start_y };
            let vis_region = SMALL_RECT {
                left: self.visible_region.left,
                top: self.visible_region.top + start_y,
                right: self.visible_region.right,
                bottom: self.visible_region.top + y - 1,
            };
            unsafe {
                winapi::WriteConsoleOutputW(self.stdout, self.chars.as_ptr(), sz, COORD { x: 0, y: 0 }, &vis_region);
            }
        }
        // update the cursor
        if surface.cursor.is_visible() {
            let pos = COORD {
                x: (surface.cursor.x as i16) + self.visible_region.left,
                y: (surface.cursor.y as i16) + self.visible_region.top,
            };
            let info = CONSOLE_CURSOR_INFO { size: 10, visible: TRUE };
            unsafe {
                winapi::SetConsoleCursorPosition(self.stdout, pos);
                winapi::SetConsoleCursorInfo(self.stdout, &info);
            }
        } else {
            let info = CONSOLE_CURSOR_INFO { size: 10, visible: FALSE };
            unsafe {
                winapi::SetConsoleCursorInfo(self.stdout, &info);
            }
        }
    }
    fn get_size(&self) -> Size {
        self.size
    }
    /*
        fn get_system_event(&mut self) -> SystemEvent {
            // let mut ir = INPUT_RECORD {
            //     event_type: 0,
            //     event: WindowsTerminalEvent { extra: 0 },
            // };
            // let mut nr_read = 0u32;

            // unsafe {
            //     if (winapi::ReadConsoleInputW(self.stdin_handle, &mut ir, 1, &mut nr_read) == FALSE) || (nr_read != 1) {
            //         return SystemEvent::None;
            //     }
            //     //println!("Event: {}",ir.event_type);
            // }
            let ir = match self.receiver.recv() {
                Ok(ir) => ir,
                Err(_) => return SystemEvent::None,
            };

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
                            old_state,
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
                        MOUSE_HWHEELED => {
                            //println!("HWHEEL {}", ir.event.mouse_event.button_state);
                            if ir.event.mouse_event.button_state >= 0x80000000 {
                                return SystemEvent::MouseWheel(MouseWheelEvent {
                                    x,
                                    y,
                                    direction: MouseWheelDirection::Left,
                                });
                            } else {
                                return SystemEvent::MouseWheel(MouseWheelEvent {
                                    x,
                                    y,
                                    direction: MouseWheelDirection::Right,
                                });
                            }
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

            SystemEvent::None
        }
    */
    fn get_clipboard_text(&self) -> Option<String> {
        unsafe {
            if winapi::OpenClipboard(0) == FALSE {
                return None;
            }

            let hmem = winapi::GetClipboardData(CF_UNICODETEXT);
            if hmem == 0 {
                winapi::CloseClipboard();
                return None;
            }
            let mut ptr = winapi::GlobalLock(hmem) as *mut u16;
            if ptr.is_null() {
                winapi::CloseClipboard();
                return None;
            }
            let mut s = String::with_capacity(16);
            while let Some(ch) = char::from_u32((*ptr) as u32) {
                if (ch as u32) == 0 {
                    break;
                }
                s.push(ch);
                ptr = ptr.add(1);
            }
            winapi::GlobalUnlock(hmem);
            winapi::CloseClipboard();
            Some(s)
        }
    }

    fn set_clipboard_text(&mut self, text: &str) {
        if text.is_empty() {
            unsafe {
                if winapi::OpenClipboard(0) != FALSE {
                    winapi::EmptyClipboard();
                    winapi::CloseClipboard();
                }
            }
        } else {
            unsafe {
                if winapi::OpenClipboard(0) == FALSE {
                    return;
                }

                winapi::EmptyClipboard();

                let len = text.count_glyphs() + 1;
                // alocate twice as much bytes (windows unicode)
                let hmem = winapi::GlobalAlloc(GMEM_MOVEABLE, len * 2);
                if hmem == 0 {
                    winapi::CloseClipboard();
                    return;
                }

                let mut ptr = winapi::GlobalLock(hmem) as *mut u16;
                if ptr.is_null() {
                    winapi::CloseClipboard();
                    winapi::GlobalFree(hmem);
                    return;
                }

                let mut pos = 0;
                while let Some((ch, size)) = text.glyph(pos) {
                    pos += size as usize;
                    if ch as u32 <= 0xFFFFu32 {
                        *ptr = ch as u16;
                    } else {
                        *ptr = b'?' as u16;
                    }
                    ptr = ptr.add(1);
                }
                // last null terminator character
                *ptr = 0;

                winapi::GlobalUnlock(hmem);

                if winapi::SetClipboardData(CF_UNICODETEXT, hmem) == 0 {
                    winapi::CloseClipboard();
                    winapi::GlobalFree(hmem);
                    return;
                }

                winapi::CloseClipboard();
            }
        }
    }

    fn has_clipboard_text(&self) -> bool {
        unsafe { (winapi::IsClipboardFormatAvailable(CF_TEXT) != FALSE) || (winapi::IsClipboardFormatAvailable(CF_UNICODETEXT) != FALSE) }
    }
}
