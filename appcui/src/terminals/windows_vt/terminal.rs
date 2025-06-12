use std::sync::mpsc::Sender;
use super::super::SystemEvent;
use super::super::SystemEventReader;
use super::super::Terminal;
use super::input::Input;
use super::super::utils::win32;
use crate::terminals::utils::win32::constants::*;
use crate::terminals::utils::win32::structs::*;
use crate::graphics::*;
use crate::system::Error;
use crate::terminals::utils::AnsiFormatter;

pub struct WindowsVTTerminal {
    console: win32::Console,
    ansi_formatter: AnsiFormatter,
}

impl WindowsVTTerminal {
    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        let console = win32::Console::new(builder, true)?;
        let input_console = console.clone();
        Input::new(input_console).start(sender);
        Ok(WindowsVTTerminal {
            console,
            ansi_formatter: AnsiFormatter::with_capacity(16384),
        })
    }

}

impl Terminal for WindowsVTTerminal {
    fn is_single_threaded(&self) -> bool {
        false
    }
    fn on_resize(&mut self, new_size: Size) {
        self.console.on_resize(new_size);
    }
    fn update_screen(&mut self, surface: &Surface) {
        // println!("Update the screen: capacity: {}, size: {:?}, region: {:?}, surface_size: {:?}",self.chars.len(),self.size,self.visible_region,surface.size);
        // safety check --> surface size should be the same as self.width/height size 
        if surface.size != self.console.size() {
            panic!("Invalid size !!!");
        }

        // draw characters using ANSI formatter
        self.ansi_formatter.clear();
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
    fn get_size(&self) -> Size {
        self.console.size()
    }

    fn get_clipboard_text(&self) -> Option<String> {
        win32::Clipboard::text()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        win32::Clipboard::set_text(text);
    }

    fn has_clipboard_text(&self) -> bool {
        win32::Clipboard::has_text()
    }
}
