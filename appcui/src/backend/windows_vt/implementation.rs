use super::super::utils::win32;
use super::super::Backend;
use super::super::SystemEventReader;
use super::input::Input;
use crate::backend::utils::AnsiFlags;
use crate::backend::utils::AnsiFormatter;
use crate::backend::RestoreReason;
use crate::backend::TerminalRestorer;
use crate::graphics::*;
use crate::system::Error;
use crate::system::SystemEvent;
use std::sync::mpsc::Sender;

pub struct WindowsVTTerminal {
    console: win32::Console,
    ansi_formatter: AnsiFormatter,
    restorer: Option<Box<dyn TerminalRestorer>>,
}

struct WindowsVTTerminalRestorer {
    restorer: Option<Box<win32::ConsoleRestorer>>,
    palette_changed: bool,
}

impl TerminalRestorer for WindowsVTTerminalRestorer {
    fn restore(mut self: Box<Self>, reason: RestoreReason) {
        let mut af = AnsiFormatter::new(256, AnsiFlags::None);
        af.clear();
        af.reset_screen();
        if self.palette_changed {
            af.reset_palette();
        }
        af.disable_mouse_events();
        af.execute();
        match reason {
            RestoreReason::Normal => {
                if let Some(restorer) = self.restorer.take() {
                    restorer.restore(reason);
                }
            }
            RestoreReason::Panic => {
                af.clear();
                // move the cursor to the bottom of the screen
                af.set_cursor_position(0, 999);
                af.execute();
            }
        }
    }
}

impl WindowsVTTerminal {
    pub(crate) fn new(builder: &crate::system::InternalBuilder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        let (console, restorer) = win32::Console::new(builder, true)?;
        let input_console = console.clone();
        let palette_changed = builder.is_palette_changed();
        let mut ansi_formatter = AnsiFormatter::new(
            16384,
            if builder.use_rgb_for_palette {
                AnsiFlags::None
            } else {
                AnsiFlags::Use16ColorSchema
            },
        );
        ansi_formatter.set_palette(builder.palette);
        ansi_formatter.execute();
        Input::new(input_console).start(sender);
        Ok(WindowsVTTerminal {
            console,
            ansi_formatter,
            restorer: Some(Box::new(WindowsVTTerminalRestorer {
                restorer: Some(Box::new(restorer)),
                palette_changed,
            })),
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
    fn take_restorer(&mut self) -> Option<Box<dyn TerminalRestorer>> {
        self.restorer.take()
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
