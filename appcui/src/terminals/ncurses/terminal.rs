use super::super::SystemEvent;
use super::super::Terminal;
use super::colors::ColorManager;
use crate::terminals::ncurses::ncursesapi::input::Input;
use std::sync::mpsc::Sender;

use crate::graphics::*;
use crate::system::Error;
use crate::terminals::SystemEventReader;

use super::ncursesapi;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;

#[cfg(target_family = "unix")]
use ncursesapi::constants::{chtype, mmask_t};

#[cfg(target_family = "unix")]
use std::char;

#[cfg(target_family = "unix")]
pub struct NcursesTerminal {
    color_manager: ColorManager,
}

#[cfg(target_family = "unix")]
impl NcursesTerminal {
    pub(crate) fn new(_builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        ncursesapi::lib::ncurses_endwin();
        ncursesapi::lib::setlocale(ncursesapi::structs::LcCategory::all, "").unwrap();
        let _ = ncursesapi::lib::ncurses_initscr();
        
        ncursesapi::lib::ncurses_nodelay(ncursesapi::lib::ncurses_stdscr(), false);
        ncursesapi::lib::ncurses_keypad(ncursesapi::lib::ncurses_stdscr(), true);
        ncursesapi::lib::ncurses_cbreak();
        ncursesapi::lib::ncurses_noecho();
        ncursesapi::lib::ncurses_nonl();
        ncursesapi::lib::ncurses_raw();
        ncursesapi::lib::ncurses_meta(ncursesapi::lib::ncurses_stdscr(), true);
        ncursesapi::lib::ncurses_mousemask(
            (ncursesapi::constants::ALL_MOUSE_EVENTS as mmask_t | ncursesapi::constants::REPORT_MOUSE_POSITION as mmask_t) as mmask_t,
            None,
        );
        println!("\x1B[?1003h");
        ncursesapi::lib::ncurses_mouseinterval(0);
        ncursesapi::lib::ncurses_set_escdelay(0);

        let term = NcursesTerminal {
            color_manager: ColorManager::new(),
        };

        // Start the event thread
        Input::new().start(sender);

        Ok(term)
    }
}

fn transform_to_hex_string(number: u32) -> String {
    let unicode_char = char::from_u32(number).unwrap();
    unicode_char.to_string()
}
#[cfg(target_family = "unix")]
impl Terminal for NcursesTerminal {
    fn update_screen(&mut self, surface: &Surface) {

        let mut current_x = 0;
        let mut current_y = 0;

        for ch in surface.chars.iter() {
            let code = ch.code as u32;

            if ch.foreground != Color::Transparent || ch.background != Color::Transparent {
                self.color_manager.set_color_pair(&ch.foreground, &ch.background);
                if (ch.flags & CharFlags::Underline) == CharFlags::Underline {
                    ncursesapi::lib::ncurses_wattron(ncursesapi::lib::ncurses_stdscr(), ncursesapi::constants::A_UNDERLINE);
                }

                if ch.flags & CharFlags::Bold == CharFlags::Bold {
                    ncursesapi::lib::ncurses_wattron(ncursesapi::lib::ncurses_stdscr(), ncursesapi::constants::A_BOLD);
                }

                ncursesapi::lib::ncurses_mvaddstr(
                    current_y,
                    current_x,
                    transform_to_hex_string(ch.code as u32).to_string().as_str(),
                )
                .unwrap();

                if (ch.flags & CharFlags::Underline) == CharFlags::Underline {
                    ncursesapi::lib::ncurses_wattroff(ncursesapi::lib::ncurses_stdscr(), ncursesapi::constants::A_UNDERLINE);
                }

                if ch.flags & CharFlags::Bold == CharFlags::Bold {
                    ncursesapi::lib::ncurses_wattroff(ncursesapi::lib::ncurses_stdscr(), ncursesapi::constants::A_BOLD);
                }
                self.color_manager.unset_color_pair(&ch.foreground, &ch.background);
            } else {
                ncursesapi::lib::ncurses_mvaddch(current_y, current_x, code as chtype);
            }

            current_x += 1;
            if current_x >= surface.size.width.try_into().unwrap() {
                current_x = 0;
                current_y += 1;
            }
        }

        if surface.cursor.is_visible() {
            ncursesapi::lib::ncurses_curs_set(ncursesapi::structs::CURSOR_VISIBILITY::CURSOR_VISIBLE);
            ncursesapi::lib::ncurses_wmove(ncursesapi::lib::ncurses_stdscr(), surface.cursor.y as i32, surface.cursor.x as i32);
        } else {
            ncursesapi::lib::ncurses_curs_set(ncursesapi::structs::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        }

        ncursesapi::lib::ncurses_wrefresh(ncursesapi::lib::ncurses_stdscr());
    }

    fn get_size(&self) -> Size {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        ncursesapi::lib::ncurses_getmaxyx(ncursesapi::lib::ncurses_stdscr(), &mut y, &mut x);
        Size::new(x as u32, y as u32)
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
        let width = new_size.width as i32;
        let height = new_size.height as i32;
        ncursesapi::lib::ncurses_wresize(ncursesapi::lib::ncurses_stdscr(), height, width);
    }

    fn is_single_threaded(&self) -> bool {
        false
    }
}
