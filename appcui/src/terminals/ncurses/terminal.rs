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
use super::colors::ColorManager;
use crate::graphics::*;
use crate::system::Error;

use ncurses::chtype;
use ncurses::endwin;
use ncurses::ll::mmask_t;
use ncurses::stdscr;
use ncurses::WINDOW;

// debug
use std::fs::File;
use std::fs::Metadata;
use std::fs::OpenOptions;
use std::io::Write;
pub struct NcursesTerminal {
    window: WINDOW,
    color_manager: ColorManager,
    // mouse_button: MouseButton,
    mouse_x: i32,
    mouse_y: i32,
    // mouse_wheel: i32,
    // key_modifiers: KeyModifier,
}

impl NcursesTerminal {
    pub(crate) fn new(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        ncurses::setlocale(ncurses::LcCategory::all, "").unwrap();
        let window = ncurses::initscr();
        ncurses::clear();

        ncurses::nodelay(ncurses::stdscr(), true);
        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::cbreak();
        ncurses::noecho();
        ncurses::nonl();
        ncurses::raw();
        ncurses::meta(ncurses::stdscr(), true);
        ncurses::mousemask((ncurses::ALL_MOUSE_EVENTS as u64 | ncurses::REPORT_MOUSE_POSITION as u64) as u32, None);
        ncurses::mouseinterval(0);
        ncurses::set_escdelay(0);

        Ok(Box::new(NcursesTerminal {
            window,
            // mouse_button: MouseButton::None,
            color_manager: ColorManager::new(),
            mouse_x: 0,
            mouse_y: 0,
            // mouse_wheel: 0,
            // key_modifiers: KeyModifier::None,
        }))
    }
}

impl Terminal for NcursesTerminal {
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
        for ch in surface.chars.iter() {
            let code = match ch.code as u32 {
                9618 => ncurses::ACS_CKBOARD(),
                9552 => ncurses::ACS_HLINE(),
                9553 => ncurses::ACS_VLINE(),
                9556 => ncurses::ACS_ULCORNER(),
                9559 => ncurses::ACS_URCORNER(),
                9565 => ncurses::ACS_LRCORNER(),
                9562 => ncurses::ACS_LLCORNER(),
                _ => ch.code as u32,
            };
            if ch.foreground != Color::Transparent {
                ncurses::attron(ncurses::COLOR_PAIR(1));
            }

            ncurses::mvaddch(current_y as i32, current_x as i32, code as chtype);

            if ch.foreground != Color::Transparent {
                ncurses::attroff(ncurses::COLOR_PAIR(1));
            }
            current_x += 1;
            if current_x >= surface.size.width {
                current_x = 0;
                current_y += 1;
            }
        }
        ncurses::wrefresh(self.window);
    }

    fn get_size(&self) -> Size {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        ncurses::getmaxyx(self.window, &mut y, &mut x);
        Size::new(x as u32, y as u32)
    }

    fn get_system_event(&mut self) -> SystemEvent {
        let ch = ncurses::wgetch(stdscr());
        // let mut debugfile = OpenOptions::new().write(true).append(true).open("debug.txt").unwrap();

        if ch == ncurses::ERR {
            return SystemEvent::None;
        }

        // debugfile.write_all(format!("{}\n", ch).as_bytes()).unwrap();
        if ch == ncurses::KEY_MOUSE {
            let mut mevent = ncurses::MEVENT {
                id: 0,
                x: 0,
                y: 0,
                z: 0,
                bstate: 0,
            };
            if ncurses::getmouse(&mut mevent) == ncurses::OK {
                // debugfile.write_all(format!("{}\n", mevent.bstate).as_bytes()).unwrap();
                let x = mevent.x as i32;
                let y = mevent.y as i32;
                let button = match mevent.bstate as i32 {
                    ncurses::BUTTON1_PRESSED => MouseButton::Left,
                    ncurses::BUTTON1_RELEASED => MouseButton::Left,
                    ncurses::BUTTON1_CLICKED => MouseButton::Left,
                    ncurses::BUTTON1_DOUBLE_CLICKED => MouseButton::Left,

                    ncurses::BUTTON2_PRESSED => MouseButton::Center,
                    ncurses::BUTTON2_RELEASED => MouseButton::Center,
                    ncurses::BUTTON2_CLICKED => MouseButton::Center,
                    ncurses::BUTTON2_DOUBLE_CLICKED => MouseButton::Center,

                    ncurses::BUTTON3_PRESSED => MouseButton::Right,
                    ncurses::BUTTON3_RELEASED => MouseButton::Right,
                    ncurses::BUTTON3_CLICKED => MouseButton::Right,
                    ncurses::BUTTON3_DOUBLE_CLICKED => MouseButton::Right,
                    _ => MouseButton::None,
                };

                // debugfile.write_all(format!("MOUSE {} {} {}\n", button_type, x, y).as_bytes()).unwrap();
                
                let mut returned = SystemEvent::None;
                if mevent.bstate as i32 & ncurses::BUTTON1_PRESSED != 0 {
                    returned = SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button });
                }
                else if mevent.bstate as i32 & ncurses::BUTTON1_RELEASED != 0 {
                    returned = SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button });
                }
                else if mevent.bstate as i32 & ncurses::BUTTON1_CLICKED != 0 {
                    returned = SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button });
                }
                else if mevent.bstate as i32 & ncurses::BUTTON1_DOUBLE_CLICKED != 0 {
                    returned = SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button });
                }
                else 
                if mevent.bstate as i32 & ncurses::REPORT_MOUSE_POSITION != 0 {
                    returned = SystemEvent::MouseMove(MouseMoveEvent { x, y, button });
                }
                return returned;
            }
        }
        if ch == ncurses::KEY_RESIZE {
            let new_size = self.get_size();
            return SystemEvent::Resize(new_size);
        }

        if ch == 27 {
            endwin();
            return SystemEvent::AppClose;
        }
        SystemEvent::None
    }
}
