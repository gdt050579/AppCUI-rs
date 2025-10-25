use super::super::utils::win32;
use super::super::Backend;
use super::super::SystemEventReader;
use super::input::Input;
use crate::backend::utils::AnsiFlags;
use crate::backend::utils::AnsiFormatter;
use crate::graphics::*;
use crate::system::Error;
use crate::system::SystemEvent;
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
            ansi_formatter: AnsiFormatter::new(
                16384,
                if builder.use_color_schema {
                    AnsiFlags::Use16ColorSchema
                } else {
                    AnsiFlags::None
                },
            ),
        })
    }
}

impl Backend for WindowsVTTerminal {
    fn is_single_threaded(&self) -> bool {
        false
    }
    fn on_resize(&mut self, new_size: Size) {
        self.console.on_resize(new_size);
    }
    fn on_close(&mut self) {
        self.ansi_formatter.clear();
        self.ansi_formatter.reset_screen();
        self.ansi_formatter.disable_mouse_events();
        self.ansi_formatter.execute();
        self.console.on_close();
    }    
    fn update_screen(&mut self, surface: &Surface) {
        // println!("Update the screen: capacity: {}, size: {:?}, region: {:?}, surface_size: {:?}",self.chars.len(),self.size,self.visible_region,surface.size);
        // safety check --> surface size should be the same as self.width/height size
        if surface.size != self.console.size() {
            panic!("Invalid size !!!");
        }

        // draw characters using ANSI formatter
        let top = self.console.visible_region().top as i32;
        self.ansi_formatter.render(surface, Point::new(0, top));
        self.ansi_formatter.execute();
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
