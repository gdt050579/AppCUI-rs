use super::super::SystemEvent;
use super::super::Terminal;
use super::colors::ColorManager;
use super::ncursesapi::input::get_key_struct;
use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use crate::input::MouseWheelDirection;
use crate::terminals::ncurses::ncursesapi::input::Input;
use crate::terminals::ncurses::ncursesapi::structs;
use crate::terminals::*;

use std::sync::atomic::AtomicPtr;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::graphics::*;
use crate::system::Error;
use crate::terminals::SystemEventReader;

use super::ncursesapi;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;

#[cfg(target_family = "unix")]
use ncursesapi::constants::{chtype, mmask_t};
use ncursesapi::externs::WINDOW;

#[cfg(target_family = "unix")]
use std::char;
use std::io::Write;

#[cfg(target_family = "unix")]
pub struct NcursesTerminal {
    window: WINDOW,
    color_manager: ColorManager,
}

#[cfg(target_family = "unix")]
impl NcursesTerminal {
    pub(crate) fn new(_builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        ncursesapi::lib::setlocale(ncursesapi::structs::LcCategory::all, "").unwrap();
        let window = ncursesapi::lib::ncurses_initscr();
        // ncursesapi::lib::ncurses_clear();

        // ncursesapi::lib::ncurses_nodelay(ncursesapi::lib::ncurses_stdscr(), true);
        ncursesapi::lib::ncurses_nodelay(ncursesapi::lib::ncurses_stdscr(), false);
        //ncursesapi::lib::ncurses_halfdelay(3);
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
        //println!("\x1B[?1003h");
        ncursesapi::lib::ncurses_mouseinterval(0);
        ncursesapi::lib::ncurses_set_escdelay(0);

        let term = NcursesTerminal {
            window,
            color_manager: ColorManager::new(),
        };

        // Start the event thread
        Input::new(to_atomic_ptr(window)).start(sender);

        Ok(term)
    }
}

pub fn to_atomic_ptr(ptr: *mut i8) -> AtomicPtr<i8> {
    AtomicPtr::new(ptr)
}    

fn transform_to_hex_string(number: u32) -> String {
    let unicode_char = char::from_u32(number).unwrap();
    unicode_char.to_string()
}
#[cfg(target_family = "unix")]
impl Terminal for NcursesTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        if self.window.is_null() {
            self.window = ncursesapi::lib::ncurses_initscr();
            ncursesapi::lib::ncurses_raw();
            ncursesapi::lib::ncurses_keypad(ncursesapi::lib::ncurses_stdscr(), true);
            ncursesapi::lib::ncurses_noecho();
        }

        let mut current_x = 0;
        let mut current_y = 0;
        //ncursesapi::lib::ncurses_wclear(self.window);

        for ch in surface.chars.iter() {
            let code = ch.code as u32;

            if ch.foreground != Color::Transparent || ch.background != Color::Transparent {
                self.color_manager.set_color_pair(&ch.foreground, &ch.background);
                if (ch.flags & CharFlags::Underline) == CharFlags::Underline {
                    ncursesapi::lib::ncurses_wattron(self.window, ncursesapi::constants::A_UNDERLINE);
                }

                if ch.flags & CharFlags::Bold == CharFlags::Bold {
                    ncursesapi::lib::ncurses_wattron(self.window, ncursesapi::constants::A_BOLD);
                }

                ncursesapi::lib::ncurses_mvaddstr(
                    current_y as i32,
                    current_x as i32,
                    transform_to_hex_string(ch.code.clone() as u32).to_string().as_str(),
                )
                .unwrap();

                if (ch.flags & CharFlags::Underline) == CharFlags::Underline {
                    ncursesapi::lib::ncurses_wattroff(self.window, ncursesapi::constants::A_UNDERLINE);
                }

                if ch.flags & CharFlags::Bold == CharFlags::Bold {
                    ncursesapi::lib::ncurses_wattroff(self.window, ncursesapi::constants::A_BOLD);
                }
                self.color_manager.unset_color_pair(&ch.foreground, &ch.background);
            } else {
                ncursesapi::lib::ncurses_mvaddch(current_y as i32, current_x as i32, code as chtype);
            }

            current_x += 1;
            if current_x >= surface.size.width {
                current_x = 0;
                current_y += 1;
            }
        }

        if surface.cursor.is_visible() {
            ncursesapi::lib::ncurses_curs_set(ncursesapi::structs::CURSOR_VISIBILITY::CURSOR_VISIBLE);
            ncursesapi::lib::ncurses_wmove(self.window, surface.cursor.y as i32, surface.cursor.x as i32);
        } else {
            ncursesapi::lib::ncurses_curs_set(ncursesapi::structs::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        }

        ncursesapi::lib::ncurses_wrefresh(self.window);
    }

    fn get_size(&self) -> Size {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        ncursesapi::lib::ncurses_getmaxyx(self.window, &mut y, &mut x);
        Size::new(x as u32, y as u32)
    }
    
    // fn query_system_event(&mut self) -> Option<SystemEvent> {
    //     let ch = ncursesapi::lib::ncurses_wget_wch(ncursesapi::lib::ncurses_stdscr());
    //     if ch.is_none() {
    //         return None;
    //     }

    //     match ch {
    //         Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_MOUSE)) => {
    //             let mut mevent = ncursesapi::structs::MEVENT {
    //                 id: 0,
    //                 x: 0,
    //                 y: 0,
    //                 z: 0,
    //                 bstate: 0,
    //             };
    //             if ncursesapi::lib::ncurses_getmouse(&mut mevent) == ncursesapi::constants::OK {
    //                 let x = mevent.x as i32;
    //                 let y = mevent.y as i32;
    //                 let button = match mevent.bstate as i32 {
    //                     ncursesapi::constants::BUTTON1_PRESSED => MouseButton::Left,
    //                     ncursesapi::constants::BUTTON1_RELEASED => MouseButton::Left,
    //                     ncursesapi::constants::BUTTON1_CLICKED => MouseButton::Left,
    //                     ncursesapi::constants::BUTTON1_DOUBLE_CLICKED => MouseButton::Left,

    //                     ncursesapi::constants::BUTTON2_PRESSED => MouseButton::Center,
    //                     ncursesapi::constants::BUTTON2_RELEASED => MouseButton::Center,
    //                     ncursesapi::constants::BUTTON2_CLICKED => MouseButton::Center,
    //                     ncursesapi::constants::BUTTON2_DOUBLE_CLICKED => MouseButton::Center,

    //                     // ncursesapi::constants::BUTTON3_PRESSED => MouseButton::Right,
    //                     // ncursesapi::constants::BUTTON3_RELEASED => MouseButton::Right,
    //                     // ncursesapi::constants::BUTTON3_CLICKED => MouseButton::Right,
    //                     // ncursesapi::constants::BUTTON3_DOUBLE_CLICKED => MouseButton::Right,
    //                     _ => MouseButton::None,
    //                 };

    //                 if button == MouseButton::None {
    //                     let button = match mevent.bstate as i32 {
    //                         ncursesapi::constants::WHEEL_UP => MouseWheelDirection::Up,
    //                         ncursesapi::constants::WHEEL_DOWN => MouseWheelDirection::Down,
    //                         ncursesapi::constants::WHEEL_LEFT => MouseWheelDirection::Left,
    //                         ncursesapi::constants::WHEEL_RIGHT => MouseWheelDirection::Right,
    //                         _ => MouseWheelDirection::None,
    //                     };
    //                     if button != MouseWheelDirection::None {
    //                         return Some(SystemEvent::MouseWheel(MouseWheelEvent { x, y, direction: button }));
    //                     }
    //                 }
    //                 let mut returned = None;
    //                 if mevent.bstate as i32 & ncursesapi::constants::BUTTON1_PRESSED != 0 {
    //                     returned = Some(SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button }));
    //                 } else if mevent.bstate as i32 & ncursesapi::constants::BUTTON1_RELEASED != 0 {
    //                     returned = Some(SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button }));
    //                 } else if mevent.bstate as i32 & ncursesapi::constants::BUTTON1_CLICKED != 0 {
    //                     returned = Some(SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button }));
    //                 } else if mevent.bstate as i32 & ncursesapi::constants::BUTTON1_DOUBLE_CLICKED != 0 {
    //                     returned = Some(SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button }));
    //                 } else if mevent.bstate as i32 & ncursesapi::constants::REPORT_MOUSE_POSITION != 0 {
    //                     returned = Some(SystemEvent::MouseMove(MouseMoveEvent { x, y, button }));
    //                 }
    //                 return returned;
    //             }
    //         }
    //         Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_RESIZE)) => {
    //             let new_size = self.get_size();
    //             return Some(SystemEvent::Resize(new_size));
    //         }
    //         // F1 - F12
    //         Some(ncursesapi::structs::WchResult::KeyCode(265..=276)) => {
    //             let key_code = match ch {
    //                 Some(ncursesapi::structs::WchResult::KeyCode(265)) => KeyCode::F1,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(266)) => KeyCode::F2,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(267)) => KeyCode::F3,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(268)) => KeyCode::F4,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(269)) => KeyCode::F5,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(270)) => KeyCode::F6,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(271)) => KeyCode::F7,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(272)) => KeyCode::F8,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(273)) => KeyCode::F9,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(274)) => KeyCode::F10,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(275)) => KeyCode::F11,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(276)) => KeyCode::F12,
    //                 _ => KeyCode::None,
    //             };
    //             return Some(SystemEvent::KeyPressed(KeyPressedEvent {
    //                 key: Key {
    //                     code: key_code,
    //                     modifier: KeyModifier::None,
    //                 },
    //                 character: '\0',
    //             }));
    //         }
    //         // Delete
    //         Some(ncursesapi::structs::WchResult::KeyCode(330)) => {
    //             return Some(SystemEvent::KeyPressed(KeyPressedEvent {
    //                 key: Key {
    //                     code: KeyCode::Delete,
    //                     modifier: KeyModifier::None,
    //                 },
    //                 character: '\0',
    //             }));
    //         }
    //         // Arrow keys
    //         Some(ncursesapi::structs::WchResult::KeyCode(
    //             ncursesapi::constants::KEY_UP
    //             | ncursesapi::constants::KEY_DOWN
    //             | ncursesapi::constants::KEY_LEFT
    //             | ncursesapi::constants::KEY_RIGHT
    //             | 263,
    //         )) => {
    //             let key_code = match ch {
    //                 Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_UP)) => KeyCode::Up,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_DOWN)) => KeyCode::Down,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_LEFT)) => KeyCode::Left,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_RIGHT)) => KeyCode::Right,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(263)) => KeyCode::Backspace,
    //                 _ => KeyCode::None,
    //             };
    //             return Some(SystemEvent::KeyPressed(KeyPressedEvent {
    //                 key: Key {
    //                     code: key_code,
    //                     modifier: KeyModifier::None,
    //                 },
    //                 character: '\0',
    //             }));
    //         }

    //         // Shift + Arrow keys
    //         Some(ncursesapi::structs::WchResult::KeyCode(
    //             ncursesapi::constants::KEY_SR | ncursesapi::constants::KEY_SF | ncursesapi::constants::KEY_SLEFT | ncursesapi::constants::KEY_SRIGHT,
    //         )) => {
    //             let key_code = match ch {
    //                 Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_SR)) => KeyCode::Up,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_SF)) => KeyCode::Down,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_SLEFT)) => KeyCode::Left,
    //                 Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_SRIGHT)) => KeyCode::Right,
    //                 _ => KeyCode::None,
    //             };
    //             return Some(SystemEvent::KeyPressed(KeyPressedEvent {
    //                 key: Key {
    //                     code: key_code,
    //                     modifier: KeyModifier::Shift,
    //                 },
    //                 character: '\0',
    //             }));
    //         }

    //         Some(ncursesapi::structs::WchResult::Char(ch)) => {
    //             if ch == 27 {
    //                 return Some(SystemEvent::KeyPressed(KeyPressedEvent {
    //                     key: Key {
    //                         code: KeyCode::Escape,
    //                         modifier: KeyModifier::None,
    //                     },
    //                     character: '\0',
    //                 }));
    //             }

    //             let mut key = get_key_struct(ch);
    //             if key.key.code == KeyCode::Backspace {
    //                 key.character = 8 as char;
    //             }
    //             return Some(SystemEvent::KeyPressed(key));
    //         }

    //         _ => return None,
    //     };

    //     None
    // }

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
        ncursesapi::lib::ncurses_wresize(self.window, height, width);
    }

    fn is_single_threaded(&self) -> bool {
        return false;
    }
}
