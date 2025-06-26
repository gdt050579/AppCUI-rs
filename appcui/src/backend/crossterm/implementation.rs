use super::input::Input;
use crate::{
    backend::{Backend, SystemEventReader},
    graphics::{Color, Size, Surface},
    system::{Error, SystemEvent},
};
use crossterm::event::EnableMouseCapture;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{Color as CrosstermColor, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use std::sync::mpsc::Sender;

pub(crate) struct CrossTerm {
    size: Size,
}

impl CrossTerm {
    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        // Enable raw mode
        enable_raw_mode().map_err(|e| {
            Error::new(
                crate::system::ErrorKind::InitializationFailure,
                format!("Failed to enable raw mode: {}", e),
            )
        })?;

        execute!(stdout(), EnableMouseCapture).map_err(|e| {
            Error::new(
                crate::system::ErrorKind::InitializationFailure,
                format!("Failed to enable mouse capture: {}", e),
            )
        })?;

        let (width, height) = size().map_err(|e| {
            Error::new(
                crate::system::ErrorKind::InitializationFailure,
                format!("Failed to get terminal size: {}", e),
            )
        })?;

        let mut term = CrossTerm {
            size: Size::new(width as u32, height as u32),
        };

        if let Some(sz) = builder.size {
            term.size = sz;
        }

        execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All), Hide).map_err(|e| {
            Error::new(
                crate::system::ErrorKind::InitializationFailure,
                format!("Failed to initialize terminal: {}", e),
            )
        })?;

        Input::new().start(sender);

        Ok(term)
    }

    fn convert_color(&self, color: Color) -> CrosstermColor {
        match color {
            Color::Black => CrosstermColor::Black,
            Color::DarkRed => CrosstermColor::DarkRed,
            Color::DarkGreen => CrosstermColor::DarkGreen,
            Color::Olive => CrosstermColor::DarkYellow,
            Color::DarkBlue => CrosstermColor::DarkBlue,
            Color::Magenta => CrosstermColor::DarkMagenta,
            Color::Teal => CrosstermColor::DarkCyan,
            Color::Silver => CrosstermColor::White,
            Color::Gray => CrosstermColor::Grey,
            Color::Red => CrosstermColor::Red,
            Color::Green => CrosstermColor::Green,
            Color::Yellow => CrosstermColor::Yellow,
            Color::Blue => CrosstermColor::Blue,
            Color::Pink => CrosstermColor::Magenta,
            Color::Aqua => CrosstermColor::Cyan,
            Color::White => CrosstermColor::White,
            Color::Transparent => CrosstermColor::Reset,
            #[cfg(feature = "TRUE_COLORS")]
            Color::RGB(r, g, b) => CrosstermColor::Rgb { r, g, b },
        }
    }
}

impl Backend for CrossTerm {
    fn update_screen(&mut self, surface: &Surface) {
        let mut stdout = stdout();
        use crossterm::queue;
        use std::io::Write;

        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        let mut x = 0;
        let mut y = 0;
        let w = surface.size.width as u16;

        let mut current_fg = None;
        let mut current_bg = None;

        for ch in surface.chars.iter() {
            if Some(ch.foreground) != current_fg {
                queue!(stdout, SetForegroundColor(self.convert_color(ch.foreground))).unwrap();
                current_fg = Some(ch.foreground);
            }
            if Some(ch.background) != current_bg {
                queue!(stdout, SetBackgroundColor(self.convert_color(ch.background))).unwrap();
                current_bg = Some(ch.background);
            }
            queue!(stdout, Print(ch.code)).unwrap();

            x += 1;
            if x >= w {
                x = 0;
                y += 1;
                if y < surface.size.height {
                    queue!(stdout, MoveTo(0, y as u16)).unwrap();
                }
            }
        }

        if surface.cursor.is_visible() {
            queue!(stdout, Show, MoveTo(surface.cursor.x as u16, surface.cursor.y as u16)).unwrap();
        } else {
            queue!(stdout, Hide).unwrap();
        }

        stdout.flush().unwrap();
    }

    fn on_resize(&mut self, new_size: Size) {
        self.size = new_size;
    }

    fn size(&self) -> Size {
        self.size
    }

    fn clipboard_text(&self) -> Option<String> {
        None
    }

    fn set_clipboard_text(&mut self, _text: &str) {}

    fn has_clipboard_text(&self) -> bool {
        false
    }

    fn is_single_threaded(&self) -> bool {
        false
    }

    fn on_close(&mut self) {
        let _ = execute!(stdout(), LeaveAlternateScreen, Show, ResetColor);
        let _ = disable_raw_mode();
    }
}
