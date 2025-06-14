use super::super::utils::win32;
use super::super::SystemEvent;
use super::super::SystemEventReader;
use super::super::Terminal;
use super::input::Input;
use crate::graphics::*;
use crate::system::Error;
use crate::terminals::utils::win32::constants::*;
use crate::terminals::utils::win32::structs::*;
use crate::terminals::utils::AnsiFormatter;
use std::io::Write;
use std::sync::mpsc::Sender;

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
        (ch >= '\u{1100}' && ch <= '\u{115F}') || // Hangul Jamo
        (ch >= '\u{2329}' && ch <= '\u{232A}') || // Angle brackets
        (ch >= '\u{2E80}' && ch <= '\u{A4CF}') || // CJK & radicals
        (ch >= '\u{AC00}' && ch <= '\u{D7A3}') || // Hangul Syllables
        (ch >= '\u{F900}' && ch <= '\u{FAFF}') || // CJK Compatibility Ideographs
        (ch >= '\u{FE10}' && ch <= '\u{FE19}') || // Vertical punctuation
        (ch >= '\u{FE30}' && ch <= '\u{FE6F}') || // More CJK symbols
        (ch >= '\u{FF00}' && ch <= '\u{FF60}') || // Fullwidth ASCII variants
        (ch >= '\u{FFE0}' && ch <= '\u{FFE6}') || // Fullwidth symbols
        (ch >= '\u{1F300}' && ch <= '\u{1FAFF}') // Emojis & pictographs
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
                self.ansi_formatter.write_char(ch.code);
                if Self::is_wide_char(ch.code) {
                    x += 1;
                }
                x += 1;
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
