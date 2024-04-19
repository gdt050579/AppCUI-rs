use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::input::MouseWheelDirection;
use crate::prelude::window;
use crate::prelude::ErrorKind;
use crate::terminals::debug;

use super::super::system_event::KeyModifierChangedEvent;
use super::super::KeyPressedEvent;
use super::super::MouseButtonDownEvent;
use super::super::MouseButtonUpEvent;
use super::super::MouseDoubleClickEvent;
use super::super::MouseMoveEvent;
use super::super::MouseWheelEvent;
use super::super::SystemEvent;
use super::super::Terminal;
use crate::graphics::*;
use crate::system::Error;

use ncurses::chtype;
use ncurses::WINDOW;

// debug
use std::fs::File;
use std::io::Write;
pub struct NcursesTerminal {
    window: WINDOW,
    // mouse_button: MouseButton,
    // mouse_x: i32,
    // mouse_y: i32,
    // mouse_wheel: i32,
    // key_modifiers: KeyModifier,
}

impl NcursesTerminal {
    pub(crate) fn new(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        // let locale_conf = ncurses::LcCategory::all;
        // ncurses::setlocale(locale_conf, "en_US.UTF-8");
        let window = ncurses::initscr();
        // ncurses::raw();
        // ncurses::keypad(ncurses::stdscr(), true);
        // ncurses::noecho();

        Ok(Box::new(NcursesTerminal {
            window,
            // mouse_button: MouseButton::None,
            // mouse_x: 0,
            // mouse_y: 0,
            // mouse_wheel: 0,
            // key_modifiers: KeyModifier::None,
        }))
    }
    
}

fn char_to_unicode_int(ch: char) -> u32 {
    let mut utf8_bytes = [0; 4];
    let utf8_len = ch.encode_utf8(&mut utf8_bytes).len();
    let mut unicode_int = 0;
    for i in 0..utf8_len {
        unicode_int = (unicode_int << 8) | (utf8_bytes[i] as u32);
    }
  
    unicode_int
  }

impl Terminal for NcursesTerminal{
    fn update_screen(&mut self, surface: &Surface) {
        if self.window.is_null() {
            self.window = ncurses::initscr();
            ncurses::raw();
            ncurses::keypad(ncurses::stdscr(), true);
            ncurses::noecho();
        }

        let mut current_x = 0;
        let mut current_y = 0;
        ncurses::wclear(self.window);
        let mut debugfile = match File::create("debug.txt") {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error creating file: {}", e);
                return;
            }
        };
        for ch in surface.chars.iter() {
            ncurses::ACS_CKBOARD();
            ncurses::mvaddch(current_y as i32, current_x as i32, ch as chtype);
            debugfile.write_all(format!("{} ",ch.code as u32).as_bytes()).unwrap();
            debugfile.write_all(format!("{} ",ncurses::ACS_CKBOARD()).as_bytes()).unwrap();
            current_x += 1;
            if current_x >= surface.size.width {
                current_x = 0;
                current_y += 1;
            }

        }
        ncurses::wrefresh(self.window);

    }

    fn get_size(&self) -> Size {
        let mut x:i32 = 0;
        let mut y:i32 = 0;
        ncurses::getmaxyx(self.window,&mut y,&mut x);
        Size::new(x as u32, y as u32)
    }

    fn get_system_event(&mut self) -> SystemEvent {
        SystemEvent::None
    }
}