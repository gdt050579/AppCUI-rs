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
use ncurses::chtype;
use ncurses::curs_set;
use ncurses::endwin;
use ncurses::ll::mmask_t;
use ncurses::stdscr;
use ncurses::WINDOW;

use std::char;
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
        ncurses::halfdelay(1);
        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::cbreak();
        ncurses::noecho();
        ncurses::nonl();
        ncurses::raw();
        ncurses::meta(ncurses::stdscr(), true);
        ncurses::mousemask(
            (ncurses::ALL_MOUSE_EVENTS as mmask_t | ncurses::REPORT_MOUSE_POSITION as mmask_t) as mmask_t,
            None,
        );
        println!("\x1B[?1003h");
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
                // 9604 => ncurses::ACS_BLOCK(),
                9660 => ncurses::ACS_DARROW(),
                9650 => ncurses::ACS_UARROW(),
                9472 => ncurses::ACS_HLINE(),
                9474 => ncurses::ACS_VLINE(),
                9484 => ncurses::ACS_ULCORNER(),
                9488 => ncurses::ACS_URCORNER(),
                9496 => ncurses::ACS_LRCORNER(),
                9492 => ncurses::ACS_LLCORNER(),
                9679 => ncurses::ACS_BULLET(),
                _ => ch.code as chtype,
            };

            if ch.foreground != Color::Transparent || ch.background != Color::Transparent {
                self.color_manager.set_color_pair(&ch.foreground, &ch.background);
                if (ch.flags & CharFlags::Underline) == CharFlags::Underline {
                    ncurses::wattron(self.window, ncurses::A_UNDERLINE);
                }

                if ch.flags & CharFlags::Bold == CharFlags::Bold {
                    ncurses::wattron(self.window, ncurses::A_BOLD);
                }

                ncurses::mvaddch(current_y as i32, current_x as i32, code as chtype);

                if (ch.flags & CharFlags::Underline) == CharFlags::Underline {
                    ncurses::wattroff(self.window, ncurses::A_UNDERLINE);
                }

                if ch.flags & CharFlags::Bold == CharFlags::Bold {
                    ncurses::wattroff(self.window, ncurses::A_BOLD);
                }
                self.color_manager.unset_color_pair(&ch.foreground, &ch.background);
            } else {
                ncurses::mvaddch(current_y as i32, current_x as i32, code as chtype);
            }

            current_x += 1;
            if current_x >= surface.size.width {
                current_x = 0;
                current_y += 1;
            }
        }

        if surface.cursor.is_visible() {
            curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
            ncurses::wmove(self.window, surface.cursor.y as i32, surface.cursor.x as i32);
        } else {
            curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
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
        let ch = ncurses::wget_wch(stdscr());        
        if ch.is_none() {
            return SystemEvent::None;
        }

        match ch {
            Some(ncurses::WchResult::KeyCode(ncurses::KEY_MOUSE)) => {
                let mut mevent = ncurses::MEVENT {
                    id: 0,
                    x: 0,
                    y: 0,
                    z: 0,
                    bstate: 0,
                };
                if ncurses::getmouse(&mut mevent) == ncurses::OK {
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

                    let mut returned = SystemEvent::None;
                    if mevent.bstate as i32 & ncurses::BUTTON1_PRESSED != 0 {
                        returned = SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button });
                    } else if mevent.bstate as i32 & ncurses::BUTTON1_RELEASED != 0 {
                        returned = SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button });
                    } else if mevent.bstate as i32 & ncurses::BUTTON1_CLICKED != 0 {
                        returned = SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button });
                    } else if mevent.bstate as i32 & ncurses::BUTTON1_DOUBLE_CLICKED != 0 {
                        returned = SystemEvent::MouseDoubleClick(MouseDoubleClickEvent { x, y, button });
                    } else if mevent.bstate as i32 & ncurses::REPORT_MOUSE_POSITION != 0 {
                        returned = SystemEvent::MouseMove(MouseMoveEvent { x, y, button });
                    }
                    return returned;
                }
            }
            Some(ncurses::WchResult::KeyCode(ncurses::KEY_RESIZE)) => {
                let new_size = self.get_size();
                return SystemEvent::Resize(new_size);
            }
            // F1 - F12
            Some(ncurses::WchResult::KeyCode(265..=276)) => {
                let key_code = match ch {
                    Some(ncurses::WchResult::KeyCode(265)) => KeyCode::F1,
                    Some(ncurses::WchResult::KeyCode(266)) => KeyCode::F2,
                    Some(ncurses::WchResult::KeyCode(267)) => KeyCode::F3,
                    Some(ncurses::WchResult::KeyCode(268)) => KeyCode::F4,
                    Some(ncurses::WchResult::KeyCode(269)) => KeyCode::F5,
                    Some(ncurses::WchResult::KeyCode(270)) => KeyCode::F6,
                    Some(ncurses::WchResult::KeyCode(271)) => KeyCode::F7,
                    Some(ncurses::WchResult::KeyCode(272)) => KeyCode::F8,
                    Some(ncurses::WchResult::KeyCode(273)) => KeyCode::F9,
                    Some(ncurses::WchResult::KeyCode(274)) => KeyCode::F10,
                    Some(ncurses::WchResult::KeyCode(275)) => KeyCode::F11,
                    Some(ncurses::WchResult::KeyCode(276)) => KeyCode::F12,
                    _ => KeyCode::None,
                };
                return SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: key_code,
                        modifier: KeyModifier::None,
                    },
                    character: ' ',
                });
            }
            // Delete
            Some(ncurses::WchResult::KeyCode(330) ) =>{
                return SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: KeyCode::Delete,
                        modifier: KeyModifier::None,
                    },
                    character: ' ',
                });
            }
            // Arrow keys
            Some(ncurses::WchResult::KeyCode(ncurses::KEY_UP | ncurses::KEY_DOWN | ncurses::KEY_LEFT | ncurses::KEY_RIGHT | 263)) => {
                let key_code  = match ch {
                    Some(ncurses::WchResult::KeyCode(ncurses::KEY_UP)) => KeyCode::Up,
                    Some(ncurses::WchResult::KeyCode(ncurses::KEY_DOWN)) => KeyCode::Down,
                    Some(ncurses::WchResult::KeyCode(ncurses::KEY_LEFT)) => KeyCode::Left,
                    Some(ncurses::WchResult::KeyCode(ncurses::KEY_RIGHT)) => KeyCode::Right,
                    Some(ncurses::WchResult::KeyCode(263)) => KeyCode::Backspace,
                    _ => KeyCode::None,
                };
                return SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: key_code,
                        modifier: KeyModifier::None,
                    },
                    character: ' ',
                });
            }

            // Shift + Arrow keys
            Some(ncurses::WchResult::KeyCode(ncurses::KEY_SR | ncurses::KEY_SF | ncurses::KEY_SLEFT | ncurses::KEY_SRIGHT)) => {
                let key_code  = match ch {
                    Some(ncurses::WchResult::KeyCode(ncurses::KEY_SR)) => KeyCode::Up,
                    Some(ncurses::WchResult::KeyCode(ncurses::KEY_SF)) => KeyCode::Down,
                    Some(ncurses::WchResult::KeyCode(ncurses::KEY_SLEFT)) => KeyCode::Left,
                    Some(ncurses::WchResult::KeyCode(ncurses::KEY_SRIGHT)) => KeyCode::Right,
                    _ => KeyCode::None,
                };
                return SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key {
                        code: key_code,
                        modifier: KeyModifier::Shift,
                    },
                    character: ' ',
                });
            }

            Some(ncurses::WchResult::Char(ch)) => {
                if ch == 27 {
                    return  SystemEvent::KeyPressed(KeyPressedEvent {
                        key: Key {
                            code: KeyCode::Escape,
                            modifier: KeyModifier::None,
                        },
                        character: ' ',
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
