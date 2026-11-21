use super::input::Input;
use crate::{
    backend::{
        utils::{AnsiFlags, AnsiFormatter},
        Backend, ColorPalette, Palette, SystemEventReader,
    },
    graphics::{CharFlags, Color, Size, Surface},
    system::{Error, SystemEvent},
};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{Color as CrosstermColor, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use std::sync::mpsc::Sender;
use crate::prelude::Character;

#[cfg(target_os = "windows")]
use crate::backend::utils::win32;

#[cfg(target_family = "unix")]
use copypasta::ClipboardContext;
#[cfg(target_family = "unix")]
use copypasta::ClipboardProvider;

pub(crate) struct CrossTerm {
    size: Size,
    use_rgb_for_palette: bool,
    palette_changed: bool,
    rgb_palette: ColorPalette,
    screen_chars: Vec<Character> // Contains characters painted on screen
}

impl CrossTerm {
    pub(crate) fn new(builder: &crate::system::InternalBuilder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
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
            use_rgb_for_palette: builder.use_rgb_for_palette,
            palette_changed: builder.is_palette_changed(),
            rgb_palette: match builder.palette {
                Palette::Custom(palette) => palette,
                // Palette::Terminal never reaches RGB mode (rejected by InternalBuilder::build)
                _ => ColorPalette::built_in(),
            },
            screen_chars: Vec::new()
        };
        
        for _n in 0..(width * height) {
          term.screen_chars.push(Character::new(0, Color::Transparent, Color::Transparent, CharFlags::None));
        }

        if let Some(sz) = builder.size {
            term.size = sz;
        }

        execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All), Hide).map_err(|e| {
            Error::new(
                crate::system::ErrorKind::InitializationFailure,
                format!("Failed to initialize terminal: {}", e),
            )
        })?;

        if term.palette_changed {
            // indexed mode: reprogram the 16 terminal slots (OSC 4)
            let mut ansi = AnsiFormatter::new(512, AnsiFlags::Use16ColorSchema);
            ansi.set_palette(builder.palette);
            let _ = execute!(stdout(), Print(ansi.text()));
        }

        Input::new().start(sender);

        Ok(term)
    }

    fn convert_color(&self, color: Color) -> CrosstermColor {
        if !self.use_rgb_for_palette {
            match color {
                Color::Black => CrosstermColor::Black,
                Color::DarkRed => CrosstermColor::DarkRed,
                Color::DarkGreen => CrosstermColor::DarkGreen,
                Color::Olive => CrosstermColor::DarkYellow,
                Color::DarkBlue => CrosstermColor::DarkBlue,
                Color::Magenta => CrosstermColor::DarkMagenta,
                Color::Teal => CrosstermColor::DarkCyan,
                Color::Silver => CrosstermColor::Grey,
                Color::Gray => CrosstermColor::DarkGrey,
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
        } else {
            match color {
                Color::Transparent => CrosstermColor::Reset,
                #[cfg(feature = "TRUE_COLORS")]
                Color::RGB(r, g, b) => CrosstermColor::Rgb { r, g, b },
                _ => {
                    let (r, g, b) = self.rgb_palette.colors[color.as_color_index() as usize];
                    CrosstermColor::Rgb { r, g, b }
                }
            }
        }
    }
}

impl Backend for CrossTerm {
    fn update_screen(&mut self, surface: &Surface) {
        let mut stdout = stdout();
        use crossterm::queue;
        use std::io::Write;

        queue!(stdout, Hide).unwrap(); // Draw cursor on every position is very slow
        queue!(stdout, MoveTo(0, 0)).unwrap();

        let mut x = 0;
        let mut y = 0;
        let w = surface.size.width as u16;

        let mut current_fg = None;
        let mut current_bg = None;
        let mut flags = CharFlags::None;
        
        let mut current_screen_char_index = 0;
        let mut position_changed = true; // Controls if cursor position has been changed

        for ch in surface.chars.iter() {
            let current_screen_char = &self.screen_chars[current_screen_char_index];
            // If the character is already painted, don't paint the character again
            if *current_screen_char != *ch {
                self.screen_chars[current_screen_char_index] = *ch;
                if Some(ch.foreground) != current_fg {
                    queue!(stdout, SetForegroundColor(self.convert_color(ch.foreground))).unwrap();
                    current_fg = Some(ch.foreground);
                }
                if Some(ch.background) != current_bg {
                    queue!(stdout, SetBackgroundColor(self.convert_color(ch.background))).unwrap();
                    current_bg = Some(ch.background);
                }
                if ch.flags != flags {
                    queue!(stdout, crossterm::style::SetAttribute(crossterm::style::Attribute::Reset)).unwrap();
                    if ch.flags.contains(CharFlags::Bold) {
                        queue!(stdout, crossterm::style::SetAttribute(crossterm::style::Attribute::Bold)).unwrap();
                    }
                    if ch.flags.contains(CharFlags::Italic) {
                        queue!(stdout, crossterm::style::SetAttribute(crossterm::style::Attribute::Italic)).unwrap();
                    }
                    if ch.flags.contains(CharFlags::Underline) {
                        queue!(stdout, crossterm::style::SetAttribute(crossterm::style::Attribute::Underlined)).unwrap();
                    }
                    if ch.flags.contains(CharFlags::DoubleUnderline) {
                        queue!(stdout, crossterm::style::SetAttribute(crossterm::style::Attribute::DoubleUnderlined)).unwrap();
                    }
                    if ch.flags.contains(CharFlags::CurlyUnderline) {
                        queue!(stdout, crossterm::style::SetAttribute(crossterm::style::Attribute::Undercurled)).unwrap();
                    }
                    if ch.flags.contains(CharFlags::DottedUnderline) {
                        queue!(stdout, crossterm::style::SetAttribute(crossterm::style::Attribute::Underdotted)).unwrap();
                    }
                    if ch.flags.contains(CharFlags::StrikeThrough) {
                        queue!(stdout, crossterm::style::SetAttribute(crossterm::style::Attribute::CrossedOut)).unwrap();
                    }

                    flags = ch.flags;
                }
                if ! position_changed {
                    queue!(stdout, MoveTo(x as u16, y as u16)).unwrap();
                    position_changed = true;
                }
                if ! ch.code.is_ascii() {
                    // Errors on painting could be due to wide chars. 
                    // Cursor must be reposition after each wide character.
                    position_changed = false;
                }   
                queue!(stdout, Print(ch.code)).unwrap();
            } else if position_changed {
                position_changed = false;
            }
          
            current_screen_char_index += 1;
            x += 1;
            if x >= w {
                x = 0;
                y += 1;
                if y < surface.size.height && !position_changed {
                    queue!(stdout, MoveTo(0, y as u16)).unwrap();
                    position_changed = true;
                }
            }
        }

        if surface.cursor.is_visible() {
            queue!(stdout, Show, MoveTo(surface.cursor.x as u16, surface.cursor.y as u16)).unwrap();
        }

        stdout.flush().unwrap();
    }

    fn on_resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.screen_chars = Vec::new();
        for _n in 0..(self.size.width * self.size.height) {
          self.screen_chars.push(Character::new(0, Color::Transparent, Color::Transparent, CharFlags::None));
        }
    }

    fn size(&self) -> Size {
        self.size
    }

    fn clipboard_text(&self) -> Option<String> {
        #[cfg(target_os = "windows")]
        {
            win32::Clipboard::text()
        }

        #[cfg(target_family = "unix")]
        {
            let mut ctx: ClipboardContext = ClipboardContext::new().ok()?;
            ctx.get_contents().ok()
        }
    }

    fn set_clipboard_text(&mut self, text: &str) {
        #[cfg(target_os = "windows")]
        {
            win32::Clipboard::set_text(text);
        }

        #[cfg(target_family = "unix")]
        {
            let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
            ctx.set_contents(text.to_owned()).unwrap();
        }
    }

    fn has_clipboard_text(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            win32::Clipboard::has_text()
        }
        #[cfg(target_family = "unix")]
        {
            let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
            ctx.get_contents().is_ok()
        }
    }
    fn is_single_threaded(&self) -> bool {
        false
    }

    fn on_close(&mut self) {
        let _ = execute!(stdout(), DisableMouseCapture);
        if self.palette_changed {
            let mut ansi = AnsiFormatter::new(256, AnsiFlags::Use16ColorSchema);
            ansi.reset_palette();
            let _ = execute!(stdout(), Print(ansi.text()));
        }
        let _ = execute!(stdout(), LeaveAlternateScreen, Show, ResetColor);
        let _ = disable_raw_mode();
    }
}
