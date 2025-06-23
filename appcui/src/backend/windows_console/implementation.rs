use super::super::utils::win32;
use super::super::SystemEventReader;
use super::super::Backend;
use super::input::Input;
use crate::backend::utils::win32::constants::*;
use crate::backend::utils::win32::structs::*;
use crate::graphics::*;
use crate::system::Error;
use crate::system::SystemEvent;
use std::sync::mpsc::Sender;

pub struct WindowsConsoleTerminal {
    console: win32::Console,
    chars: Vec<CHAR_INFO>,
}

impl WindowsConsoleTerminal {
    // if size is present -> resize
    // if colors are present --> recolor
    // if font is present --> apply font & size

    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        let console = win32::Console::new(builder, false)?;
        let input_console = console.clone();
        let mut term = WindowsConsoleTerminal {
            console,
            chars: Vec::with_capacity(1024),
        };
        // println!("Start region: {:?}",term.visible_region);
        term.chars.resize(
            (term.console.size().width as usize) * (term.console.size().height as usize) * 2,
            CHAR_INFO { code: 32, attr: 0 },
        );
        // start the event thread
        Input::new(input_console).start(sender);
        // all good - start the sender thread
        Ok(term)
    }
}

impl Backend for WindowsConsoleTerminal {
    fn is_single_threaded(&self) -> bool {
        false
    }
    fn on_resize(&mut self, new_size: Size) {
        let w = new_size.width as usize;
        let h = new_size.height as usize;
        self.chars.resize(w * h * 2, CHAR_INFO { code: 32, attr: 0 });
        self.console.on_resize(new_size);
    }
    fn on_close(&mut self) {
        self.console.on_close();
    }
    fn update_screen(&mut self, surface: &Surface) {
        // println!("Update the screen: capacity: {}, size: {:?}, region: {:?}, surface_size: {:?}",self.chars.len(),self.size,self.visible_region,surface.size);
        // safety check --> surface size should be the same as self.width/height size
        if surface.size != self.console.size() {
            panic!("Invalid size !!!");
        }
        // check if allocated space si twice the size (to account for surrogates)
        if self.chars.len() != (self.console.size().width as usize) * (self.console.size().height as usize) * 2 {
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
                screen_char.attr = ch.foreground.as_color_index() as u16;
            }
            if ch.background != Color::Transparent {
                screen_char.attr |= (ch.background.as_color_index() as u16) << 4;
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
                        left: self.console.visible_region().left,
                        top: self.console.visible_region().top + start_y,
                        right: self.console.visible_region().right,
                        bottom: self.console.visible_region().top + y - 1,
                    };
                    unsafe {
                        win32::api::WriteConsoleOutputW(self.console.stdout(), self.chars.as_ptr(), sz, COORD { x: 0, y: 0 }, &vis_region);
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
                x: self.console.size().width as i16,
                y: self.console.size().height as i16,
            };
            unsafe {
                win32::api::WriteConsoleOutputW(
                    self.console.stdout(),
                    self.chars.as_ptr(),
                    sz,
                    COORD { x: 0, y: 0 },
                    &self.console.visible_region(),
                );
            }
        } else if start_y < y {
            let sz = COORD { x: w as i16, y: y - start_y };
            let vis_region = SMALL_RECT {
                left: self.console.visible_region().left,
                top: self.console.visible_region().top + start_y,
                right: self.console.visible_region().right,
                bottom: self.console.visible_region().top + y - 1,
            };
            unsafe {
                win32::api::WriteConsoleOutputW(self.console.stdout(), self.chars.as_ptr(), sz, COORD { x: 0, y: 0 }, &vis_region);
            }
        }
        // update the cursor
        if surface.cursor.is_visible() {
            let pos = COORD {
                x: (surface.cursor.x as i16) + self.console.visible_region().left,
                y: (surface.cursor.y as i16) + self.console.visible_region().top,
            };
            let info = CONSOLE_CURSOR_INFO { size: 10, visible: TRUE };
            unsafe {
                win32::api::SetConsoleCursorPosition(self.console.stdout(), pos);
                win32::api::SetConsoleCursorInfo(self.console.stdout(), &info);
            }
        } else {
            let info = CONSOLE_CURSOR_INFO { size: 10, visible: FALSE };
            unsafe {
                win32::api::SetConsoleCursorInfo(self.console.stdout(), &info);
            }
        }
    }
    #[inline(always)]
    fn size(&self) -> Size {
        self.console.size()
    }

    fn clipboard_text(&self) -> Option<String> {
        win32::Clipboard::text()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        win32::Clipboard::set_text(text);
    }

    fn has_clipboard_text(&self) -> bool {
        win32::Clipboard::has_text()
    }
}
