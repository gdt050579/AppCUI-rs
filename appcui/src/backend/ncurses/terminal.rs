use super::super::SystemEvent;
use super::super::Terminal;
use super::ncursesapi::lib::ncurses_wcwidth;
use super::ncursesapi::externs::*;
use crate::backends::ncurses::ncursesapi::input::Input;
use std::sync::mpsc::Sender;

use crate::graphics::*;
use crate::system::Error;
use crate::backends::SystemEventReader;

use super::ncursesapi;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;

#[cfg(target_family = "unix")]
use ncursesapi::constants::mmask_t;

#[cfg(target_family = "unix")]
pub struct NcursesTerminal {
    size: Size,
    win: WINDOW,
}

#[cfg(target_family = "unix")]
impl NcursesTerminal {
    pub(crate) fn new(_builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        ncursesapi::lib::ncurses_endwin();
        ncursesapi::lib::setlocale(ncursesapi::structs::LcCategory::all, "").unwrap();
        let win = ncursesapi::lib::ncurses_initscr();
        ncursesapi::lib::ncurses_start_color();
        ncursesapi::lib::ncurses_use_default_colors();

        ncursesapi::lib::ncurses_nodelay(win, false);
        ncursesapi::lib::ncurses_keypad(win, true);
        ncursesapi::lib::ncurses_cbreak();
        ncursesapi::lib::ncurses_noecho();
        ncursesapi::lib::ncurses_nonl();
        ncursesapi::lib::ncurses_raw();
        ncursesapi::lib::ncurses_meta(win, true);
        ncursesapi::lib::ncurses_mousemask(
            (ncursesapi::constants::ALL_MOUSE_EVENTS as mmask_t | ncursesapi::constants::REPORT_MOUSE_POSITION as mmask_t) as mmask_t,
            None,
        );
        println!("\x1B[?1003h");
        ncursesapi::lib::ncurses_mouseinterval(0);
        ncursesapi::lib::ncurses_set_escdelay(0);

        // set color paires
        for f in 0i16..=15i16 {
            for b in 0i16..=15i16 {
                ncursesapi::lib::ncurses_init_pair(
                    b * 16 + f,
                    NcursesTerminal::color_index(Color::from_value(f as i32).unwrap()),
                    NcursesTerminal::color_index(Color::from_value(b as i32).unwrap()),
                );
            }
        }

        // get the size
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        ncursesapi::lib::ncurses_getmaxyx(win, &mut y, &mut x);

        let term = NcursesTerminal {
            size: Size::new(x as u32, y as u32),
            win,
        };

        // Start the event thread
        Input::new().start(sender);

        Ok(term)
    }

    #[inline(always)]
    #[cfg(target_family = "unix")]
    fn color_index(color: Color) -> i16 {
        match color {
            Color::Black => 0,
            Color::DarkRed => 1,
            Color::DarkGreen => 2,
            Color::Olive => 3,
            Color::DarkBlue => 4,
            Color::Magenta => 5,
            Color::Teal => 6,
            Color::Silver => 7,
            Color::Gray => 8,
            Color::Red => 9,
            Color::Green => 10,
            Color::Yellow => 11,
            Color::Blue => 12,
            Color::Pink => 13,
            Color::Aqua => 14,
            Color::White => 15,
            Color::Transparent => 0,
        }
    }
}

#[cfg(target_family = "unix")]
impl Terminal for NcursesTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        let mut x = 0;
        let mut y = 0;
        let w: i32 = surface.size.width as i32;
        let mut utf8_buf: [u8; 8] = [0; 8];
        let mut skip_chars = 0;

        for ch in surface.chars.iter() {
            if skip_chars > 0 {
                skip_chars -= 1;
            } else {
                let fc = ch.foreground as i16;
                let bc = ch.background as i16;
                let idx = fc + bc * 16;
                ncursesapi::lib::ncurses_wattron(self.win, ncursesapi::lib::ncurses_COLOR_PAIR(idx));

                if ch.flags.contains(CharFlags::Underline) {
                    ncursesapi::lib::ncurses_wattron(self.win, ncursesapi::constants::A_UNDERLINE);
                }

                if ch.flags.contains(CharFlags::Bold) {
                    ncursesapi::lib::ncurses_wattron(self.win, ncursesapi::constants::A_BOLD);
                }

                let _ = ncursesapi::lib::ncurses_mvaddstr(y, x, ch.code.encode_utf8(&mut utf8_buf));

                let cw = ncurses_wcwidth(ch.code).max(1) - 1;
                skip_chars += cw;
            }

            x += 1;
            if x >= w {
                x = 0;
                y += 1;
            }
        }

        if surface.cursor.is_visible() {
            ncursesapi::lib::ncurses_curs_set(ncursesapi::structs::CURSOR_VISIBILITY::CURSOR_VISIBLE);
            ncursesapi::lib::ncurses_wmove(self.win, surface.cursor.y as i32, surface.cursor.x as i32);
        } else {
            ncursesapi::lib::ncurses_curs_set(ncursesapi::structs::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        }

        ncursesapi::lib::ncurses_wrefresh(self.win);
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_clipboard_text(&self) -> Option<String> {
        let mut ctx: ClipboardContext = ClipboardContext::new().ok()?;
        ctx.get_contents().ok()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
        ctx.set_contents(text.to_owned()).unwrap();
    }

    fn has_clipboard_text(&self) -> bool {
        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
        ctx.get_contents().is_ok()
    }

    fn on_resize(&mut self, new_size: Size) {
        self.size = new_size;
    }

    fn is_single_threaded(&self) -> bool {
        false
    }
}
