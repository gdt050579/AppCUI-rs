use std::{os::fd::AsRawFd, task::Poll};

use crate::{
    backend::termios::api::TermiosError,
    input::{Key, KeyModifier, MouseButton},
    prelude::KeyCode,
};

// Define C system binding calls
extern "C" {
    // "unistd.h" function bindings
    pub fn read(file_des: u32, buf: *mut u8, size: usize) -> isize;
}

// Default descriptor for the standard input file
pub const STDIN_FILENO: u32 = 0;

#[derive(Debug)]
pub struct TermiosReader;

#[derive(Debug)]
pub struct AnsiKey {
    bytes: [u8; 5],
    code: AnsiKeyCode,
    modifier: KeyModifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseButtonEvent {
    pub(crate) button: MouseButton, // None => release (in xterm documentation there is no None)
    pub(crate) x: u8,
    pub(crate) y: u8,
}

impl AnsiKey {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn code(&self) -> AnsiKeyCode {
        self.code
    }

    pub fn modifier(&self) -> KeyModifier {
        self.modifier
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiKeyCode {
    Letter(Letter),
    MouseButton(MouseButtonEvent),
    MouseMove(MouseButtonEvent),
    Comma,
    Dot,
    Slash,
    SemiColon,
    Quote,
    LeftBracket,
    RightBracket,
    Dash,
    Equal,
    BackSlash,
    AngleQuote,
    _PageUp,
    _PageDown,
    _Home,
    _End,
    Delete,
    _Backspace,
    Space,
    _Enter,
    _Tab,
    _Insert,
    _Up,
    _Down,
    _Left,
    _Right,
    _Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    _F11,
    F12,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub struct UnknownLetter;

impl TryFrom<u8> for Letter {
    type Error = UnknownLetter;
    fn try_from(value: u8) -> Result<Self, UnknownLetter> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            3 => Ok(Self::C),
            4 => Ok(Self::D),
            5 => Ok(Self::E),
            6 => Ok(Self::F),
            7 => Ok(Self::G),
            8 => Ok(Self::H),
            9 => Ok(Self::I),
            10 => Ok(Self::J),
            11 => Ok(Self::K),
            12 => Ok(Self::L),
            13 => Ok(Self::M),
            14 => Ok(Self::N),
            15 => Ok(Self::O),
            16 => Ok(Self::P),
            17 => Ok(Self::Q),
            18 => Ok(Self::R),
            19 => Ok(Self::S),
            20 => Ok(Self::T),
            21 => Ok(Self::U),
            22 => Ok(Self::V),
            23 => Ok(Self::W),
            24 => Ok(Self::X),
            25 => Ok(Self::Y),
            26 => Ok(Self::Z),
            _ => Err(UnknownLetter),
        }
    }
}

impl From<AnsiKey> for Key {
    fn from(value: AnsiKey) -> Self {
        let code = match value.code {
            AnsiKeyCode::Letter(letter) => match letter {
                Letter::A => KeyCode::A,
                Letter::B => KeyCode::B,
                Letter::C => KeyCode::C,
                Letter::D => KeyCode::D,
                Letter::E => KeyCode::E,
                Letter::F => KeyCode::F,
                Letter::G => KeyCode::G,
                Letter::H => KeyCode::H,
                Letter::I => KeyCode::I,
                Letter::J => KeyCode::J,
                Letter::K => KeyCode::K,
                Letter::L => KeyCode::L,
                Letter::M => KeyCode::M,
                Letter::N => KeyCode::N,
                Letter::O => KeyCode::O,
                Letter::P => KeyCode::P,
                Letter::Q => KeyCode::Q,
                Letter::R => KeyCode::R,
                Letter::S => KeyCode::S,
                Letter::T => KeyCode::T,
                Letter::U => KeyCode::U,
                Letter::V => KeyCode::V,
                Letter::W => KeyCode::W,
                Letter::X => KeyCode::X,
                Letter::Y => KeyCode::Y,
                Letter::Z => KeyCode::Z,
            },
            AnsiKeyCode::N0 => KeyCode::N0,
            AnsiKeyCode::N1 => KeyCode::N1,
            AnsiKeyCode::N2 => KeyCode::N2,
            AnsiKeyCode::N3 => KeyCode::N3,
            AnsiKeyCode::N4 => KeyCode::N4,
            AnsiKeyCode::N5 => KeyCode::N5,
            AnsiKeyCode::N6 => KeyCode::N6,
            AnsiKeyCode::N7 => KeyCode::N7,
            AnsiKeyCode::N8 => KeyCode::N8,
            AnsiKeyCode::N9 => KeyCode::N9,
            AnsiKeyCode::F1 => KeyCode::F1,
            AnsiKeyCode::F2 => KeyCode::F2,
            AnsiKeyCode::F3 => KeyCode::F3,
            AnsiKeyCode::F4 => KeyCode::F4,
            AnsiKeyCode::F5 => KeyCode::F5,
            AnsiKeyCode::F6 => KeyCode::F6,
            AnsiKeyCode::F7 => KeyCode::F7,
            AnsiKeyCode::F8 => KeyCode::F8,
            AnsiKeyCode::F9 => KeyCode::F9,
            AnsiKeyCode::F10 => KeyCode::F10,
            AnsiKeyCode::_F11 => KeyCode::F11,
            AnsiKeyCode::F12 => KeyCode::F12,
            AnsiKeyCode::_Enter => KeyCode::Enter,
            AnsiKeyCode::_Escape => KeyCode::Escape,
            AnsiKeyCode::_Insert => KeyCode::Insert,
            AnsiKeyCode::Delete => KeyCode::Delete,
            AnsiKeyCode::_Backspace => KeyCode::Backspace,
            AnsiKeyCode::_Tab => KeyCode::Tab,
            AnsiKeyCode::_Left => KeyCode::Left,
            AnsiKeyCode::_Up => KeyCode::Up,
            AnsiKeyCode::_Down => KeyCode::Down,
            AnsiKeyCode::_Right => KeyCode::Right,
            AnsiKeyCode::_PageUp => KeyCode::PageUp,
            AnsiKeyCode::_PageDown => KeyCode::PageDown,
            AnsiKeyCode::_Home => KeyCode::Home,
            AnsiKeyCode::_End => KeyCode::End,
            AnsiKeyCode::Space => KeyCode::Space,
            _ => KeyCode::None,
        };

        Key {
            code,
            modifier: value.modifier,
        }
    }
}

// Terminal assigns codes from 1 to 31 (0 being NULL) to keys pressed when `Ctrl` key is also
// pressed. A good way to think about it is that with `Ctrl` pressed, only the lower 5 bits are
// taken into condideration.
const _CTRL_KEY_MASK: u8 = 0b0001_1111;

const MOUSE_SHIFT_MASK: u8 = 4;
const MOUSE_META_MASK: u8 = 8;
const MOUSE_CTRL_MASK: u8 = 16;

impl TermiosReader {
    fn parse_mouse_event() -> Result<AnsiKey, TermiosError> {
        let (mut button_code, x, y) = (checked_stdin_read()? - 32, checked_stdin_read()? - 32, checked_stdin_read()? - 32); // all of them are "encoded" by adding 32 to the actual value

        let is_motion_event = button_code >= 32; // the way we differentiate between a simple mouse move event and simple mouse press/release event is this
        if is_motion_event {
            button_code -= 32;
        }

        let button_bits = button_code & 0b11;
        let button: MouseButton = match button_bits {
            0 => MouseButton::Left,
            1 => MouseButton::Right,
            2 => MouseButton::Center,
            _ => MouseButton::None,
        };

        let event = MouseButtonEvent { button, x: x - 1, y: y - 1 }; // coordinates start at 1 in the codes

        let mut modifier = KeyModifier::None;
        if (button_code & MOUSE_SHIFT_MASK) != 0 {
            modifier.set(KeyModifier::Shift);
        }
        if (button_code & MOUSE_META_MASK) != 0 {
            modifier.set(KeyModifier::Alt);
        }
        if (button_code & MOUSE_CTRL_MASK) != 0 {
            modifier.set(KeyModifier::Ctrl);
        }

        Ok(AnsiKey {
            bytes: [27, 91, 77, button_code, 0],
            code: if is_motion_event {
                AnsiKeyCode::MouseMove(event)
            } else {
                AnsiKeyCode::MouseButton(event)
            },
            modifier,
        })
    }

    pub fn read_key() -> Result<AnsiKey, TermiosError> {
        while let Ok(c) = checked_stdin_read() {
            if c == 0 {
                continue;
            }
            let (bytes, code, modifier) = match c {
                // Entering this branch means we pressed the `Ctrl` key
                1..=31 => {
                    let modifier = KeyModifier::Ctrl;
                    // If character is 27, this means a function key was pressed and following
                    // is a sequence of characters.
                    match c {
                        8 => ([c, 0, 0, 0, 0], AnsiKeyCode::_Backspace, KeyModifier::Ctrl),
                        9 => ([c, 0, 0, 0, 0], AnsiKeyCode::_Tab, KeyModifier::None),
                        10..=13 => ([c, 0, 0, 0, 0], AnsiKeyCode::_Enter, KeyModifier::None),
                        1..=7 | 14..=26 => {
                            let key = AnsiKeyCode::Letter(Letter::try_from(c)?);
                            ([c, 0, 0, 0, 0], key, modifier)
                        }
                        // Value of `27` corresponds to the `ESC` control character and it has
                        // special handling
                        27 => {
                            let modifier = KeyModifier::None;
                            // We read the next character to see what this is about
                            //let byte_2 = checked_stdin_read()?;
                            let byte_2 = try_check_stdin_read().unwrap_or(0);
                            match byte_2 {
                                79 => {
                                    // Here we read from F1 to F4 inclusive
                                    // We need to read a 3rd byte
                                    let byte_3 = checked_stdin_read()?;
                                    // Fill in the appropriate key
                                    // F1 -> 27 79 80
                                    // F2 -> 27 79 81
                                    // F3 -> 27 79 82
                                    // F4 -> 27 79 83
                                    let key = match byte_3 {
                                        80 => AnsiKeyCode::F1,
                                        81 => AnsiKeyCode::F2,
                                        82 => AnsiKeyCode::F3,
                                        83 => AnsiKeyCode::F4,
                                        _ => return Err(TermiosError::UnknownKey),
                                    };
                                    let code = [c, byte_2, byte_3, 0, 0];
                                    (code, key, modifier)
                                }
                                91 => {
                                    // We need to read a 3rd byte
                                    let byte_3 = checked_stdin_read()?;
                                    match byte_3 {
                                        // Here we read from F5 to F8 inclusive
                                        // F5 -> 27 91 49 53 126
                                        // F6 -> 27 91 49 55 126
                                        // F7 -> 27 91 49 56 126
                                        // F8 -> 27 91 49 57 126
                                        49 => {
                                            let (byte_4, byte_5) = (checked_stdin_read()?, checked_stdin_read()?);
                                            let key = match (byte_4, byte_5) {
                                                (53, 126) => AnsiKeyCode::F5,
                                                (55, 126) => AnsiKeyCode::F6,
                                                (56, 126) => AnsiKeyCode::F7,
                                                (57, 126) => AnsiKeyCode::F8,
                                                _ => return Err(TermiosError::UnknownKey),
                                            };
                                            ([c, byte_2, byte_3, byte_4, byte_5], key, modifier)
                                        }
                                        // F9 -> 27 91 50 48 126
                                        // F10 -> 27 91 50 49 126
                                        // F11 -> Seems to be hardwired by the OS
                                        // F12 -> 27 91 50 52 126
                                        50 => {
                                            let (byte_4, byte_5) = (checked_stdin_read()?, checked_stdin_read()?);
                                            let key = match (byte_4, byte_5) {
                                                (48, 126) => AnsiKeyCode::F9,
                                                (49, 126) => AnsiKeyCode::F10,
                                                (52, 126) => AnsiKeyCode::F12,
                                                _ => return Err(TermiosError::UnknownKey),
                                            };
                                            ([c, byte_2, byte_3, byte_4, byte_5], key, modifier)
                                        }
                                        65 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::_Up, modifier),
                                        66 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::_Down, modifier),
                                        67 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::_Right, modifier),
                                        68 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::_Left, modifier),
                                        77 => return Self::parse_mouse_event(),
                                        _ => return Err(TermiosError::UnknownKey),
                                    }
                                }
                                0 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::_Escape, modifier),
                                _ => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Unknown, modifier),
                            }
                        }
                        28 => ([c, 0, 0, 0, 0], AnsiKeyCode::BackSlash, modifier),
                        29 => ([c, 0, 0, 0, 0], AnsiKeyCode::RightBracket, modifier),
                        30 => ([c, 0, 0, 0, 0], AnsiKeyCode::Unknown, modifier),
                        31 => ([c, 0, 0, 0, 0], AnsiKeyCode::Dash, modifier),
                        _ => unreachable!(),
                    }
                }
                32 => ([c, 0, 0, 0, 0], AnsiKeyCode::Space, KeyModifier::None),
                33 => ([c, 0, 0, 0, 0], AnsiKeyCode::N1, KeyModifier::Shift),
                34 => ([c, 0, 0, 0, 0], AnsiKeyCode::Quote, KeyModifier::Shift),
                35 => ([c, 0, 0, 0, 0], AnsiKeyCode::N3, KeyModifier::Shift),
                36 => ([c, 0, 0, 0, 0], AnsiKeyCode::N4, KeyModifier::Shift),
                37 => ([c, 0, 0, 0, 0], AnsiKeyCode::N5, KeyModifier::Shift),
                38 => ([c, 0, 0, 0, 0], AnsiKeyCode::N7, KeyModifier::Shift),
                39 => ([c, 0, 0, 0, 0], AnsiKeyCode::Quote, KeyModifier::None),
                40 => ([c, 0, 0, 0, 0], AnsiKeyCode::N9, KeyModifier::Shift),
                41 => ([c, 0, 0, 0, 0], AnsiKeyCode::N0, KeyModifier::Shift),
                42 => ([c, 0, 0, 0, 0], AnsiKeyCode::N8, KeyModifier::Shift),
                43 => ([c, 0, 0, 0, 0], AnsiKeyCode::Equal, KeyModifier::Shift),
                44 => ([c, 0, 0, 0, 0], AnsiKeyCode::Comma, KeyModifier::None),
                45 => ([c, 0, 0, 0, 0], AnsiKeyCode::Dash, KeyModifier::None),
                46 => ([c, 0, 0, 0, 0], AnsiKeyCode::Dot, KeyModifier::None),
                47 => ([c, 0, 0, 0, 0], AnsiKeyCode::Slash, KeyModifier::None),
                48 => ([c, 0, 0, 0, 0], AnsiKeyCode::N0, KeyModifier::None),
                49 => ([c, 0, 0, 0, 0], AnsiKeyCode::N1, KeyModifier::None),
                50 => ([c, 0, 0, 0, 0], AnsiKeyCode::N2, KeyModifier::None),
                51 => ([c, 0, 0, 0, 0], AnsiKeyCode::N3, KeyModifier::None),
                52 => ([c, 0, 0, 0, 0], AnsiKeyCode::N4, KeyModifier::None),
                53 => ([c, 0, 0, 0, 0], AnsiKeyCode::N5, KeyModifier::None),
                54 => ([c, 0, 0, 0, 0], AnsiKeyCode::N6, KeyModifier::None),
                55 => ([c, 0, 0, 0, 0], AnsiKeyCode::N7, KeyModifier::None),
                56 => ([c, 0, 0, 0, 0], AnsiKeyCode::N8, KeyModifier::None),
                57 => ([c, 0, 0, 0, 0], AnsiKeyCode::N9, KeyModifier::None),
                58 => ([c, 0, 0, 0, 0], AnsiKeyCode::SemiColon, KeyModifier::Shift),
                59 => ([c, 0, 0, 0, 0], AnsiKeyCode::SemiColon, KeyModifier::None),
                60 => ([c, 0, 0, 0, 0], AnsiKeyCode::Comma, KeyModifier::Shift),
                61 => ([c, 0, 0, 0, 0], AnsiKeyCode::Equal, KeyModifier::None),
                62 => ([c, 0, 0, 0, 0], AnsiKeyCode::Dot, KeyModifier::Shift),
                63 => ([c, 0, 0, 0, 0], AnsiKeyCode::Slash, KeyModifier::Shift),
                64 => ([c, 0, 0, 0, 0], AnsiKeyCode::N2, KeyModifier::Shift),
                // Capital Letters
                65..=90 => {
                    let key = AnsiKeyCode::Letter(Letter::try_from(c.saturating_sub(64))?);
                    ([c, 0, 0, 0, 0], key, KeyModifier::Shift)
                }
                91 => ([c, 0, 0, 0, 0], AnsiKeyCode::LeftBracket, KeyModifier::None),
                92 => ([c, 0, 0, 0, 0], AnsiKeyCode::BackSlash, KeyModifier::None),
                93 => ([c, 0, 0, 0, 0], AnsiKeyCode::RightBracket, KeyModifier::None),
                94 => ([c, 0, 0, 0, 0], AnsiKeyCode::N6, KeyModifier::Shift),
                95 => ([c, 0, 0, 0, 0], AnsiKeyCode::Dash, KeyModifier::Shift),
                96 => ([c, 0, 0, 0, 0], AnsiKeyCode::AngleQuote, KeyModifier::None),
                // Normal letters
                97..=122 => {
                    let key = AnsiKeyCode::Letter(Letter::try_from(c.saturating_sub(96))?);
                    ([c, 0, 0, 0, 0], key, KeyModifier::None)
                }
                123 => ([c, 0, 0, 0, 0], AnsiKeyCode::LeftBracket, KeyModifier::Shift),
                124 => ([c, 0, 0, 0, 0], AnsiKeyCode::BackSlash, KeyModifier::Shift),
                125 => ([c, 0, 0, 0, 0], AnsiKeyCode::RightBracket, KeyModifier::Shift),
                126 => ([c, 0, 0, 0, 0], AnsiKeyCode::AngleQuote, KeyModifier::Shift),
                127 => ([c, 0, 0, 0, 0], AnsiKeyCode::_Backspace, KeyModifier::None),
                194 => {
                    // Currently, all the character that we know with this key code, are pressed
                    // using an `Alt-Key` combination.
                    let modifier = KeyModifier::Alt;
                    // We read the next character to see what this is about
                    let byte_2 = checked_stdin_read()?;
                    match byte_2 {
                        // `Alt-Space`
                        160 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Space, modifier),
                        // `Alt-1` (Alt, one)
                        161 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::N1, modifier),
                        // `Alt-4`
                        162 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::N4, modifier),
                        // `Alt-3`
                        163 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::N3, modifier),
                        // `Alt-6`
                        167 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::N6, modifier),
                        // `Alt-u`
                        168 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::U), modifier),
                        // `Alt-g`
                        169 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::G), modifier),
                        // `Alt-8`
                        170 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::N9, modifier),
                        // `Alt-\` (Alt-backslash)
                        171 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::BackSlash, modifier),
                        // `Alt-l`
                        172 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::L), modifier),
                        // `Alt-r`
                        174 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::R), modifier),
                        // `Alt-e`
                        180 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::E), modifier),
                        // `Alt-m`
                        181 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::M), modifier),
                        // `Alt-7`
                        182 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::N7, modifier),
                        // `Alt-0`
                        186 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::N0, modifier),
                        // If we do not know the key, we log it in case we might want to have
                        // support for it.
                        _ => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Unknown, modifier),
                    }
                }
                195 => {
                    // Currently, all the character that we know with this key code, are pressed
                    // using an `Alt-Key` combination.
                    let modifier = KeyModifier::Alt;
                    // We read the next character to see what this is about
                    let byte_2 = checked_stdin_read()?;
                    match byte_2 {
                        // `Alt-s`
                        159 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::S), modifier),
                        // `Alt-a`
                        165 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::A), modifier),
                        // `Alt-'` (Alt-quote)
                        166 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Quote, modifier),
                        // `Alt-c`
                        167 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::C), modifier),
                        // `Alt-o`
                        184 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::O), modifier),
                        // If we do not know the key, we log it in case we might want to have
                        // support for it.
                        _ => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Unknown, modifier),
                    }
                }
                197 => {
                    // Currently, all the character that we know with this key code, are pressed
                    // using an `Alt-Key` combination.
                    let modifier = KeyModifier::Alt;
                    // We read the next character to see what this is about
                    let byte_2 = checked_stdin_read()?;
                    match byte_2 {
                        // `Alt-q`
                        147 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::Q), modifier),
                        // If we do not know the key, we log it in case we might want to have
                        // support for it.
                        _ => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Unknown, modifier),
                    }
                }
                198 => {
                    // Currently, all the character that we know with this key code, are pressed
                    // using an `Alt-Key` combination.
                    let modifier = KeyModifier::Alt;
                    // We read the next character to see what this is about
                    let byte_2 = checked_stdin_read()?;
                    match byte_2 {
                        // `Alt-f`
                        146 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::F), modifier),
                        // If we do not know the key, we log it in case we might want to have
                        // support for it.
                        _ => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Unknown, modifier),
                    }
                }
                203 => {
                    // Currently, all the character that we know with this key code, are pressed
                    // using an `Alt-Key` combination.
                    let modifier = KeyModifier::Alt;
                    // We read the next character to see what this is about
                    let byte_2 = checked_stdin_read()?;
                    match byte_2 {
                        // `Alt-i`
                        134 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::I), modifier),
                        // `Alt-h`
                        153 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::H), modifier),
                        // `Alt-k`
                        154 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::K), modifier),
                        // `Alt-n`
                        156 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::N), modifier),
                        // If we do not know the key, we log it in case we might want to have
                        // support for it.
                        _ => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Unknown, modifier),
                    }
                }
                206 => {
                    // Currently, all the character that we know with this key code, are pressed
                    // using an `Alt-Key` combination.
                    let modifier = KeyModifier::Alt;
                    // We read the next character to see what this is about
                    let byte_2 = checked_stdin_read()?;
                    match byte_2 {
                        // `Alt-z`
                        169 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::Z), modifier),
                        // If we do not know the key, we log it in case we might want to have
                        // support for it.
                        _ => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Unknown, modifier),
                    }
                }
                207 => {
                    // Currently, all the character that we know with this key code, are pressed
                    // using an `Alt-Key` combination.
                    let modifier = KeyModifier::Alt;
                    // We read the next character to see what this is about
                    let byte_2 = checked_stdin_read()?;
                    match byte_2 {
                        // `Alt-p`
                        128 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Letter(Letter::P), modifier),
                        // If we do not know the key, we log it in case we might want to have
                        // support for it.
                        _ => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Unknown, modifier),
                    }
                }
                226 => {
                    // We read the next character to see what this is about
                    let byte_2 = checked_stdin_read()?;
                    match byte_2 {
                        128 => {
                            // Currently, all the character that we know with this key code, are
                            // pressed using an `Alt-Key` combination.
                            let modifier = KeyModifier::Alt;
                            // At this point we know we should read a 3rd byte
                            let byte_3 = checked_stdin_read()?;
                            match byte_3 {
                                // `Alt-]` (Alt-dash)
                                147 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Dash, modifier),
                                // `Alt-]` (Alt-right bracket)
                                152 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::RightBracket, modifier),
                                // `Alt-[` (Alt-left bracket)
                                156 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::LeftBracket, modifier),
                                // `Alt-t`
                                160 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Letter(Letter::T), modifier),
                                // `Alt-8`
                                162 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::N8, modifier),
                                // `Alt-;` (Alt-semicolon)
                                166 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::SemiColon, modifier),
                                // If we do not know the key, we log it in case we might want to
                                // have support for it.
                                _ => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Unknown, KeyModifier::None),
                            }
                        }
                        132 => {
                            // Currently, all the character that we know with this key code, are
                            // pressed using an `Alt-Key` combination.
                            let modifier = KeyModifier::Alt;
                            // At this point we know we should read a 3rd byte
                            let byte_3 = checked_stdin_read()?;
                            match byte_3 {
                                // `Alt-2`
                                162 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::N2, modifier),
                                // If we do not know the key, we log it in case we might want to
                                // have support for it.
                                _ => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Unknown, KeyModifier::None),
                            }
                        }
                        136 => {
                            // Currently, all the character that we know with this key code, are
                            // pressed using an `Alt-Key` combination.
                            let modifier = KeyModifier::Alt;
                            // At this point we know we should read a 3rd byte
                            let byte_3 = checked_stdin_read()?;
                            match byte_3 {
                                // `Alt-d`
                                130 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Letter(Letter::D), modifier),
                                // `Alt-j`
                                134 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Letter(Letter::J), modifier),
                                // `Alt-w`
                                145 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Letter(Letter::W), modifier),
                                // `Alt-v`
                                154 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Letter(Letter::V), modifier),
                                // `Alt-5`
                                158 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::N5, modifier),
                                // `Alt-b`
                                171 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Letter(Letter::B), modifier),
                                // If we do not know the key, we log it in case we might want to
                                // have support for it.
                                _ => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Unknown, KeyModifier::None),
                            }
                        }
                        137 => {
                            // Currently, all the character that we know with this key code, are
                            // pressed using an `Alt-Key` combination.
                            let modifier = KeyModifier::Alt;
                            // At this point we know we should read a 3rd byte
                            let byte_3 = checked_stdin_read()?;
                            match byte_3 {
                                // `Alt-X`
                                136 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Letter(Letter::X), modifier),
                                // `Alt-=` (Alt-equal)
                                160 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Equal, modifier),
                                // `Alt-,` (Alt-comma)
                                164 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Comma, modifier),
                                // `Alt-.` (Alt-dot)
                                165 => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Dot, modifier),
                                // If we do not know the key, we log it in case we might want to
                                // have support for it.
                                _ => ([c, byte_2, byte_3, 0, 0], AnsiKeyCode::Unknown, KeyModifier::None),
                            }
                        }
                        // If we do not know the key, we log it in case we might want to have
                        // support for it.
                        _ => ([c, byte_2, 0, 0, 0], AnsiKeyCode::Unknown, KeyModifier::None),
                    }
                }
                _ => ([c, 0, 0, 0, 0], AnsiKeyCode::Unknown, KeyModifier::None),
            };
            return Ok(AnsiKey { bytes, code, modifier });
        }
        Err(TermiosError::ReadStdInFailed)
    }
}

/// Calls the system `read` and checks that no error occurs
///
/// # Erorr
///
/// In case an OS error occurs, we return it
pub fn checked_stdin_read() -> Result<u8, TermiosError> {
    let mut byte = 0;
    if unsafe { read(STDIN_FILENO, &mut byte, 1) } == -1 {
        return Err(TermiosError::ReadStdInFailed);
    }
    // println!("{}", byte);
    Ok(byte)
}

pub fn try_check_stdin_read() -> Option<u8> {
    use std::os::unix::io::AsRawFd;

    let mut byte = [0u8;1];
    let fd = std::io::stdin().as_raw_fd();
    let mut fds = libc::pollfd { fd, events: libc::POLLIN, revents: 0 };
    let ret = unsafe { libc::poll(&mut fds,1, 25)};
    if ret<=0 {
        None
    } else {
        let res = unsafe { libc::read(fd, byte.as_mut_ptr() as *mut _, 1)};
        if res == -1 {
            None
        } else {
            Some(byte[0])
        }
    }
}
