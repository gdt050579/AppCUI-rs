use crate::input::Key;
use crate::input::KeyCode;
use crate::input::KeyModifier;
use crate::input::MouseButton;
use super::super::KeyPressedEvent;
use super::super::MouseButtonDownEvent;
use super::super::MouseButtonUpEvent;
use super::super::MouseDoubleClickEvent;
use super::super::MouseMoveEvent;
use super::super::SystemEvent;
use super::super::Terminal;
use super::colors::ColorManager;

use crate::graphics::*;
use crate::system::Error;

use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use super::ncursesapi;

#[cfg(target_family = "unix")]
use ncursesapi::constants::{
    chtype, mmask_t,
};
use ncursesapi::externs::WINDOW;

#[cfg(target_family = "unix")]
use std::char;

#[cfg(target_family = "unix")]
pub struct NcursesTerminal {
    window: WINDOW,
    color_manager: ColorManager,
}

#[cfg(target_family = "unix")]
impl NcursesTerminal {
    pub(crate) fn new(_builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        ncursesapi::lib::setlocale(ncursesapi::structs::LcCategory::all, "").unwrap();
        let window = ncursesapi::lib::ncurses_initscr();
        // ncursesapi::lib::ncurses_clear();

        ncursesapi::lib::ncurses_nodelay(ncursesapi::lib::ncurses_stdscr(), true);
        ncursesapi::lib::ncurses_halfdelay(1);
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

        Ok(Box::new(NcursesTerminal {
            window,
            color_manager: ColorManager::new(),
        }))
    }
}

const SHFIT_NUM: [i32; 10] = [41, 33, 64, 35, 36, 37, 94, 38, 42, 40];
pub fn get_key_struct(ch: u32) -> KeyPressedEvent {
    let key_code ;
    let mut key_modifier = KeyModifier::None;
    let character: char = ch as u8 as char;

    if ch >= 97 && ch <= 122 {
        key_code = KeyCode::from((ch - 69) as u8);
    } else if ch >= 65 && ch <= 90 {
        key_code = KeyCode::from((ch - 37) as u8);
        key_modifier = KeyModifier::Shift;
    } else if ch >= 48 && ch <= 57 {
        key_code = KeyCode::from((ch + 6) as u8);
    } else if SHFIT_NUM.contains(&(ch as i32)){
        let pos = SHFIT_NUM.iter().position(|&r| r == ch as i32).unwrap();
        key_code = KeyCode::from((pos + 54) as u8);
        key_modifier = KeyModifier::Shift;
    } else if ch >=1 && ch <= 26{
        key_code = KeyCode::from((ch + 27 as u32) as u8);
        key_modifier = KeyModifier::Ctrl;
    } else {
        match ch {
            32 => key_code = KeyCode::Space,
            9 => key_code = KeyCode::Tab,
            13 => key_code = KeyCode::Enter,
            27 => key_code = KeyCode::Escape,
            263 => key_code = KeyCode::Backspace,
            _ => key_code = KeyCode::None,
        }
    }
    KeyPressedEvent {
        key: Key {
            code: key_code,
            modifier: key_modifier,
        },
        character,
    }
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
        ncursesapi::lib::ncurses_wclear(self.window);

        for ch in surface.chars.iter() {
            let code = match ch.code as u32 {
                // 9618 => ncursesapi::lib::ncurses_ACS_CKBOARD(),
                // 9552 => ncursesapi::lib::ncurses_ACS_HLINE(),
                // 9553 => ncursesapi::lib::ncurses_ACS_VLINE(),
                // 9556 => ncursesapi::lib::ncurses_ACS_ULCORNER(),
                // 9559 => ncursesapi::lib::ncurses_ACS_URCORNER(),
                // 9565 => ncursesapi::lib::ncurses_ACS_LRCORNER(),
                // 9562 => ncursesapi::lib::ncurses_ACS_LLCORNER(),
                // 9604 => ncursesapi::lib::ncurses_ACS_BLOCK(),
                // 9660 => ncursesapi::lib::ncurses_ACS_DARROW(),
                // 9650 => ncursesapi::lib::ncurses_ACS_UARROW(),
                // 9472 => ncursesapi::lib::ncurses_ACS_HLINE(),
                // 9474 => ncursesapi::lib::ncurses_ACS_VLINE(),
                // 9484 => ncursesapi::lib::ncurses_ACS_ULCORNER(),
                // 9488 => ncursesapi::lib::ncurses_ACS_URCORNER(),
                // 9496 => ncursesapi::lib::ncurses_ACS_LRCORNER(),
                // 9492 => ncursesapi::lib::ncurses_ACS_LLCORNER(),
                // 9679 => ncursesapi::lib::ncurses_ACS_BULLET(),
                _ => ch.code as chtype,
            };

            if ch.foreground != Color::Transparent || ch.background != Color::Transparent {
                self.color_manager.set_color_pair(&ch.foreground, &ch.background);
                if (ch.flags & CharFlags::Underline) == CharFlags::Underline {
                    ncursesapi::lib::ncurses_wattron(self.window, ncursesapi::constants::A_UNDERLINE);
                }

                if ch.flags & CharFlags::Bold == CharFlags::Bold {
                    ncursesapi::lib::ncurses_wattron(self.window, ncursesapi::constants::A_BOLD);
                }

                ncursesapi::lib::ncurses_mvaddch(current_y as i32, current_x as i32, code as chtype);

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
            // curs_set(ncursesapi::structs::CURSOR_VISIBILITY::CURSOR_VISIBLE);
            ncursesapi::lib::ncurses_wmove(self.window, surface.cursor.y as i32, surface.cursor.x as i32);
        } else {
            // curs_set(ncursesapi::lib::ncurses_CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        }
        
        ncursesapi::lib::ncurses_wrefresh(self.window);
    }

    fn get_size(&self) -> Size {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        ncursesapi::lib::ncurses_getmaxyx(self.window, &mut y, &mut x);
        Size::new(x as u32, y as u32)
    }

    fn get_system_event(&mut self) -> SystemEvent {
        let ch = ncursesapi::lib::ncurses_wget_wch(ncursesapi::lib::ncurses_stdscr());        
        if ch.is_none() {
            return SystemEvent::None;
        }

        match ch {
            Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_MOUSE)) => {
                let mut mevent = ncursesapi::structs::MEVENT {
                    id: 0,
                    x: 0,
                    y: 0,
                    z: 0,
                    bstate: 0,
                };
                if ncursesapi::lib::ncurses_getmouse(&mut mevent) == ncursesapi::constants::OK {
                    let x = mevent.x as i32;
                    let y = mevent.y as i32;
                    let button = match mevent.bstate as i32 {
                        ncursesapi::constants::BUTTON1_PRESSED => MouseButton::Left,
                        ncursesapi::constants::BUTTON1_RELEASED => MouseButton::Left,
                        ncursesapi::constants::BUTTON1_CLICKED => MouseButton::Left,
                        ncursesapi::constants::BUTTON1_DOUBLE_CLICKED => MouseButton::Left,

                        ncursesapi::constants::BUTTON2_PRESSED => MouseButton::Center,
                        ncursesapi::constants::BUTTON2_RELEASED => MouseButton::Center,
                        ncursesapi::constants::BUTTON2_CLICKED => MouseButton::Center,
                        ncursesapi::constants::BUTTON2_DOUBLE_CLICKED => MouseButton::Center,

                        ncursesapi::constants::BUTTON3_PRESSED => MouseButton::Right,
                        ncursesapi::constants::BUTTON3_RELEASED => MouseButton::Right,
                        ncursesapi::constants::BUTTON3_CLICKED => MouseButton::Right,
                        ncursesapi::constants::BUTTON3_DOUBLE_CLICKED => MouseButton::Right,
                        _ => MouseButton::None,
                    };

                    let mut returned = SystemEvent::None;
                    if mevent.bstate as i32 & ncursesapi::constants::BUTTON1_PRESSED != 0 {
                        returned = SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button });
                    } else if mevent.bstate as i32 & ncursesapi::constants::BUTTON1_RELEASED != 0 {
                        returned = SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button });
                    } else if mevent.bstate as i32 & ncursesapi::constants::BUTTON1_CLICKED != 0 {
                        returned = SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button });
                    } else if mevent.bstate as i32 & ncursesapi::constants::BUTTON1_DOUBLE_CLICKED != 0 {
                        returned = SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button });
                    } else if mevent.bstate as i32 & ncursesapi::constants::REPORT_MOUSE_POSITION != 0 {
                        returned = SystemEvent::MouseMove(MouseMoveEvent { x, y, button });
                    }
                    return returned;
                }
            }
            Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_RESIZE)) => {
                let new_size = self.get_size();
                return SystemEvent::Resize(new_size);
            }
            // F1 - F12
            Some(ncursesapi::structs::WchResult::KeyCode(265..=276)) => {
                let key_code = match ch {
                    Some(ncursesapi::structs::WchResult::KeyCode(265)) => KeyCode::F1,
                    Some(ncursesapi::structs::WchResult::KeyCode(266)) => KeyCode::F2,
                    Some(ncursesapi::structs::WchResult::KeyCode(267)) => KeyCode::F3,
                    Some(ncursesapi::structs::WchResult::KeyCode(268)) => KeyCode::F4,
                    Some(ncursesapi::structs::WchResult::KeyCode(269)) => KeyCode::F5,
                    Some(ncursesapi::structs::WchResult::KeyCode(270)) => KeyCode::F6,
                    Some(ncursesapi::structs::WchResult::KeyCode(271)) => KeyCode::F7,
                    Some(ncursesapi::structs::WchResult::KeyCode(272)) => KeyCode::F8,
                    Some(ncursesapi::structs::WchResult::KeyCode(273)) => KeyCode::F9,
                    Some(ncursesapi::structs::WchResult::KeyCode(274)) => KeyCode::F10,
                    Some(ncursesapi::structs::WchResult::KeyCode(275)) => KeyCode::F11,
                    Some(ncursesapi::structs::WchResult::KeyCode(276)) => KeyCode::F12,
                    _ => KeyCode::None,
                };
                return SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: key_code,
                        modifier: KeyModifier::None,
                    },
                    character: '\0',
                });
            }
            // Delete
            Some(ncursesapi::structs::WchResult::KeyCode(330) ) =>{
                return SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: KeyCode::Delete,
                        modifier: KeyModifier::None,
                    },
                    character: '\0',
                });
            }
            // Arrow keys
            Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_UP | ncursesapi::constants::KEY_DOWN | ncursesapi::constants::KEY_LEFT | ncursesapi::constants::KEY_RIGHT | 263)) => {
                let key_code  = match ch {
                    Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_UP)) => KeyCode::Up,
                    Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_DOWN)) => KeyCode::Down,
                    Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_LEFT)) => KeyCode::Left,
                    Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_RIGHT)) => KeyCode::Right,
                    Some(ncursesapi::structs::WchResult::KeyCode(263)) => KeyCode::Backspace,
                    _ => KeyCode::None,
                };
                return SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: key_code,
                        modifier: KeyModifier::None,
                    },
                    character: '\0',
                });
            }

            // Shift + Arrow keys
            Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_SR | ncursesapi::constants::KEY_SF | ncursesapi::constants::KEY_SLEFT | ncursesapi::constants::KEY_SRIGHT)) => {
                let key_code  = match ch {
                    Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_SR)) => KeyCode::Up,
                    Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_SF)) => KeyCode::Down,
                    Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_SLEFT)) => KeyCode::Left,
                    Some(ncursesapi::structs::WchResult::KeyCode(ncursesapi::constants::KEY_SRIGHT)) => KeyCode::Right,
                    _ => KeyCode::None,
                };
                return SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: key_code,
                        modifier: KeyModifier::Shift,
                    },
                    character: '\0',
                });
            }

            Some(ncursesapi::structs::WchResult::Char(ch)) => {
                if ch == 27 {
                    return  SystemEvent::KeyPressed(KeyPressedEvent {
                        key: Key {
                            code: KeyCode::Escape,
                            modifier: KeyModifier::None,
                        },
                        character: '\0',
                    });
                }

                let mut key = get_key_struct(ch);
                if key.key.code == KeyCode::Backspace{
                    key.character = 8 as char;
                }
                return SystemEvent::KeyPressed(key);
            }

            _ => return SystemEvent::None,
        };

        SystemEvent::None
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
}
