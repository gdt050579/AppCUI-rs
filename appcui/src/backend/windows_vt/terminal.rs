use super::super::utils::win32;
use super::super::SystemEventReader;
use super::super::Backend;
use super::input::Input;
use crate::graphics::*;
use crate::system::Error;
use crate::backend::utils::AnsiFormatter;
use std::io::Write;
use std::sync::mpsc::Sender;
use crate::system::SystemEvent;

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
    #[inline(always)]
    fn is_wide_char(ch: char) -> bool {
        matches!(ch as u32, 0x1100..=0x115F
            | 0x2329..=0x232A
            | 0x2E80..=0x303E
            | 0x3040..=0xA4CF
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
            | 0x1F300..=0x1F64F
            | 0x1F900..=0x1F9FF
            | 0x20000..=0x2FFFD
            | 0x30000..=0x3FFFD)
    }
}

impl Backend for WindowsVTTerminal {
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
        self.ansi_formatter.reset_color();
        self.ansi_formatter.hide_cursor();
        let mut x = 0;
        let mut y = 0;
        let w = surface.size.width;
        let h = surface.size.height;
        let start_y = self.console.visible_region().top as i32;
        let mut f = None;
        let mut b = None;
        let chars = &surface.chars;
        while y < h {
            self.ansi_formatter.set_cursor_position(0, y as i32 + start_y);
            let ofs = y * w;
            while x < w {
                let ch = &chars[(ofs + x) as usize];
                if Some(ch.foreground) != f {
                    self.ansi_formatter.set_foreground_color(ch.foreground);
                    f = Some(ch.foreground);
                }
                if Some(ch.background) != b {
                    self.ansi_formatter.set_background_color(ch.background);
                    b = Some(ch.background);
                }
                if Self::is_wide_char(ch.code) {
                    // 1. write two spaces
                    self.ansi_formatter.write_string("  ");
                    // 2. reposition the cursor
                    self.ansi_formatter.set_cursor_position(x as i32, y as i32 + start_y);
                    // 3. write the character   
                    self.ansi_formatter.write_char(ch.code);
                    // 4. skip next position and reposition the cursor
                    x += 2;
                    self.ansi_formatter.set_cursor_position(x as i32, y as i32 + start_y);
                } else {
                    self.ansi_formatter.write_char(ch.code);
                    x += 1;
                }
            }
            y += 1;
            x = 0;
        }
        // update the cursor
        if surface.cursor.is_visible() {
            self.ansi_formatter.set_cursor_position(
                surface.cursor.x as i32 + self.console.visible_region().left as i32,
                surface.cursor.y as i32 + self.console.visible_region().top as i32,
            );
            self.ansi_formatter.show_cursor();
        } else {
            self.ansi_formatter.hide_cursor();
        }
        // write the ANSI formatter to the console
        // unsafe {
        //     let mut written = 0;
        //     let buf = self.ansi_formatter.text().as_bytes();
        //     win32::api::WriteFile(self.console.stdout(), buf.as_ptr(), buf.len() as u32, &mut written, std::ptr::null_mut());
        // }
        //print!("{}", self.ansi_formatter.text());
        let _ = std::io::stdout().write_all(self.ansi_formatter.text().as_bytes());
        let _ = std::io::stdout().flush();
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
