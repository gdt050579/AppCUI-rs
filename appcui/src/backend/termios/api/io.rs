use crate::{
    backend::termios::api::TermiosError,
    input::{Key, KeyModifier, MouseButton},
    prelude::KeyCode,
};

use super::common::{poll, read, PollFd, POLLIN, STDIN_FILENO};

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
    pub(crate) x: i32,
    pub(crate) y: i32,
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

const MOUSE_SHIFT_MASK: i32 = 4;
const MOUSE_META_MASK: i32 = 8;
const MOUSE_CTRL_MASK: i32 = 16;
const MOUSE_MOTION_MASK: i32 = 32;
const MOUSE_WHEEL_MASK: i32 = 64;
const MOUSE_EXTRA_BUTTON_MASK: i32 = 128;

fn mouse_modifiers(button_code: i32) -> KeyModifier {
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
    modifier
}

/// Decodes the payload of an SGR mouse report, not including the `ESC [` introducer.
///
/// `payload` is `Pb ; Px ; Py` followed by `M` or `m`. Coordinates in the payload are 1-based.
/// Button numbers follow xterm: 0 left, 1 middle, 2 right. Bit 5 (32) is motion, and a report with
/// that bit set is a move whether it ends in `M` or `m` (Windows Terminal sends hover as `m`).
/// A trailing `m` is a release only when the motion bit is clear. Bits 2, 3 and 4 are Shift, Alt and Ctrl.
pub(super) fn decode_sgr_mouse(payload: &[u8]) -> Result<AnsiKey, TermiosError> {
    let (params, final_byte) = sgr_params(payload)?;
    let button_code = params[0];
    let x = params[1].saturating_sub(1);
    let y = params[2].saturating_sub(1);
    let kind = button_code & !(MOUSE_SHIFT_MASK | MOUSE_META_MASK | MOUSE_CTRL_MASK);
    // Wheel (64) and buttons past the right button (128) are not represented as clicks. Reject
    // them after the bytes have been consumed so they are not replayed as keyboard input.
    if kind & (MOUSE_WHEEL_MASK | MOUSE_EXTRA_BUTTON_MASK) != 0 {
        return Err(TermiosError::UnknownKey);
    }

    let is_motion = kind & MOUSE_MOTION_MASK != 0;
    // A trailing `m` is a release only when the pointer is not moving. Windows Terminal ends a
    // no-button move (`35;…m`) with `m`; xterm ends the same move with `M`.
    let is_release = final_byte == b'm' && !is_motion;
    let button = if is_release {
        MouseButton::None
    } else {
        match kind & 0b11 {
            0 => MouseButton::Left,
            1 => MouseButton::Center,
            2 => MouseButton::Right,
            _ => MouseButton::None,
        }
    };
    let event = MouseButtonEvent { button, x, y };
    Ok(AnsiKey {
        bytes: [27, 91, b'<', final_byte, 0],
        code: if is_motion {
            AnsiKeyCode::MouseMove(event)
        } else {
            AnsiKeyCode::MouseButton(event)
        },
        modifier: mouse_modifiers(button_code),
    })
}

fn sgr_params(payload: &[u8]) -> Result<([i32; 3], u8), TermiosError> {
    let Some((&final_byte, fields)) = payload.split_last() else {
        return Err(TermiosError::UnknownKey);
    };
    if final_byte != b'M' && final_byte != b'm' {
        return Err(TermiosError::UnknownKey);
    }

    let mut params = [0i32; 3];
    let mut count = 0usize;
    let mut value = 0i32;
    let mut saw_digit = false;
    for &byte in fields {
        match byte {
            b'0'..=b'9' => {
                saw_digit = true;
                value = value.saturating_mul(10).saturating_add(i32::from(byte - b'0'));
            }
            b';' => {
                if !saw_digit || count >= 3 {
                    return Err(TermiosError::UnknownKey);
                }
                params[count] = value;
                count += 1;
                value = 0;
                saw_digit = false;
            }
            _ => return Err(TermiosError::UnknownKey),
        }
    }
    if !saw_digit || count != 2 {
        return Err(TermiosError::UnknownKey);
    }
    params[2] = value;
    Ok((params, final_byte))
}

/// Decodes one CSI key sequence, not including the `ESC [` introducer.
///
/// Parameters are decimals separated by `;`. The final byte is in `@`..=`~`. For `~` the first
/// parameter is the key code and the second is the xterm modifier. For every other final byte the
/// second parameter is the modifier (`1` + Shift + 2×Alt + 4×Ctrl). A missing modifier is `1`.
pub(super) fn decode_csi(payload: &[u8]) -> Result<AnsiKey, TermiosError> {
    let Some((&final_byte, fields)) = payload.split_last() else {
        return Err(TermiosError::UnknownKey);
    };
    if !(0x40..=0x7E).contains(&final_byte) {
        return Err(TermiosError::UnknownKey);
    }
    let (params, count) = csi_params(fields)?;
    let params = &params[..count];
    let mut modifier = csi_key_modifier(params, final_byte);
    let code = match final_byte {
        b'A' => AnsiKeyCode::_Up,
        b'B' => AnsiKeyCode::_Down,
        b'C' => AnsiKeyCode::_Right,
        b'D' => AnsiKeyCode::_Left,
        b'H' => AnsiKeyCode::_Home,
        b'F' => AnsiKeyCode::_End,
        b'P' => AnsiKeyCode::F1,
        b'Q' => AnsiKeyCode::F2,
        b'R' => AnsiKeyCode::F3,
        b'S' => AnsiKeyCode::F4,
        b'Z' => {
            modifier |= KeyModifier::Shift;
            AnsiKeyCode::_Tab
        }
        b'~' => tilde_key(*params.first().unwrap_or(&0))?,
        _ => return Err(TermiosError::UnknownKey),
    };
    Ok(AnsiKey {
        bytes: [27, 91, final_byte, 0, 0],
        code,
        modifier,
    })
}

fn csi_params(fields: &[u8]) -> Result<([i32; 4], usize), TermiosError> {
    let mut params = [0i32; 4];
    if fields.is_empty() {
        return Ok((params, 0));
    }
    let mut count = 0usize;
    let mut value = 0i32;
    let mut saw_digit = false;
    for &byte in fields {
        match byte {
            b'0'..=b'9' => {
                saw_digit = true;
                value = value.saturating_mul(10).saturating_add(i32::from(byte - b'0'));
            }
            b';' => {
                if count == params.len() {
                    return Err(TermiosError::UnknownKey);
                }
                params[count] = if saw_digit { value } else { 0 };
                count += 1;
                value = 0;
                saw_digit = false;
            }
            _ => return Err(TermiosError::UnknownKey),
        }
    }
    if count == params.len() {
        return Err(TermiosError::UnknownKey);
    }
    params[count] = if saw_digit { value } else { 0 };
    count += 1;
    Ok((params, count))
}

fn csi_key_modifier(params: &[i32], final_byte: u8) -> KeyModifier {
    let value = if final_byte == b'~' {
        params.get(1).copied().unwrap_or(1)
    } else if params.len() >= 2 {
        params[1]
    } else {
        1
    };
    let bits = value.saturating_sub(1);
    let mut modifier = KeyModifier::None;
    if bits & 1 != 0 {
        modifier |= KeyModifier::Shift;
    }
    if bits & 2 != 0 {
        modifier |= KeyModifier::Alt;
    }
    if bits & 4 != 0 {
        modifier |= KeyModifier::Ctrl;
    }
    modifier
}

fn tilde_key(code: i32) -> Result<AnsiKeyCode, TermiosError> {
    Ok(match code {
        1 => AnsiKeyCode::_Home,
        2 => AnsiKeyCode::_Insert,
        3 => AnsiKeyCode::Delete,
        4 => AnsiKeyCode::_End,
        5 => AnsiKeyCode::_PageUp,
        6 => AnsiKeyCode::_PageDown,
        11 => AnsiKeyCode::F1,
        12 => AnsiKeyCode::F2,
        13 => AnsiKeyCode::F3,
        14 => AnsiKeyCode::F4,
        15 => AnsiKeyCode::F5,
        17 => AnsiKeyCode::F6,
        18 => AnsiKeyCode::F7,
        19 => AnsiKeyCode::F8,
        20 => AnsiKeyCode::F9,
        21 => AnsiKeyCode::F10,
        23 => AnsiKeyCode::_F11,
        24 => AnsiKeyCode::F12,
        _ => return Err(TermiosError::UnknownKey),
    })
}

pub(super) fn add_alt(mut key: AnsiKey) -> AnsiKey {
    key.modifier |= KeyModifier::Alt;
    key
}

/// Decodes one non-escape byte the way the terminal delivers it in raw mode.
pub(super) fn decode_plain_key(c: u8) -> Result<AnsiKey, TermiosError> {
    let (code, modifier) = match c {
        8 => (AnsiKeyCode::_Backspace, KeyModifier::Ctrl),
        9 => (AnsiKeyCode::_Tab, KeyModifier::None),
        10..=13 => (AnsiKeyCode::_Enter, KeyModifier::None),
        1..=7 | 14..=26 => (AnsiKeyCode::Letter(Letter::try_from(c)?), KeyModifier::Ctrl),
        27 => (AnsiKeyCode::_Escape, KeyModifier::None),
        28 => (AnsiKeyCode::BackSlash, KeyModifier::Ctrl),
        29 => (AnsiKeyCode::RightBracket, KeyModifier::Ctrl),
        30 => (AnsiKeyCode::Unknown, KeyModifier::Ctrl),
        31 => (AnsiKeyCode::Dash, KeyModifier::Ctrl),
        32 => (AnsiKeyCode::Space, KeyModifier::None),
        33 => (AnsiKeyCode::N1, KeyModifier::Shift),
        34 => (AnsiKeyCode::Quote, KeyModifier::Shift),
        35 => (AnsiKeyCode::N3, KeyModifier::Shift),
        36 => (AnsiKeyCode::N4, KeyModifier::Shift),
        37 => (AnsiKeyCode::N5, KeyModifier::Shift),
        38 => (AnsiKeyCode::N7, KeyModifier::Shift),
        39 => (AnsiKeyCode::Quote, KeyModifier::None),
        40 => (AnsiKeyCode::N9, KeyModifier::Shift),
        41 => (AnsiKeyCode::N0, KeyModifier::Shift),
        42 => (AnsiKeyCode::N8, KeyModifier::Shift),
        43 => (AnsiKeyCode::Equal, KeyModifier::Shift),
        44 => (AnsiKeyCode::Comma, KeyModifier::None),
        45 => (AnsiKeyCode::Dash, KeyModifier::None),
        46 => (AnsiKeyCode::Dot, KeyModifier::None),
        47 => (AnsiKeyCode::Slash, KeyModifier::None),
        48 => (AnsiKeyCode::N0, KeyModifier::None),
        49 => (AnsiKeyCode::N1, KeyModifier::None),
        50 => (AnsiKeyCode::N2, KeyModifier::None),
        51 => (AnsiKeyCode::N3, KeyModifier::None),
        52 => (AnsiKeyCode::N4, KeyModifier::None),
        53 => (AnsiKeyCode::N5, KeyModifier::None),
        54 => (AnsiKeyCode::N6, KeyModifier::None),
        55 => (AnsiKeyCode::N7, KeyModifier::None),
        56 => (AnsiKeyCode::N8, KeyModifier::None),
        57 => (AnsiKeyCode::N9, KeyModifier::None),
        58 => (AnsiKeyCode::SemiColon, KeyModifier::Shift),
        59 => (AnsiKeyCode::SemiColon, KeyModifier::None),
        60 => (AnsiKeyCode::Comma, KeyModifier::Shift),
        61 => (AnsiKeyCode::Equal, KeyModifier::None),
        62 => (AnsiKeyCode::Dot, KeyModifier::Shift),
        63 => (AnsiKeyCode::Slash, KeyModifier::Shift),
        64 => (AnsiKeyCode::N2, KeyModifier::Shift),
        65..=90 => (AnsiKeyCode::Letter(Letter::try_from(c.saturating_sub(64))?), KeyModifier::Shift),
        91 => (AnsiKeyCode::LeftBracket, KeyModifier::None),
        92 => (AnsiKeyCode::BackSlash, KeyModifier::None),
        93 => (AnsiKeyCode::RightBracket, KeyModifier::None),
        94 => (AnsiKeyCode::N6, KeyModifier::Shift),
        95 => (AnsiKeyCode::Dash, KeyModifier::Shift),
        96 => (AnsiKeyCode::AngleQuote, KeyModifier::None),
        97..=122 => (AnsiKeyCode::Letter(Letter::try_from(c.saturating_sub(96))?), KeyModifier::None),
        123 => (AnsiKeyCode::LeftBracket, KeyModifier::Shift),
        124 => (AnsiKeyCode::BackSlash, KeyModifier::Shift),
        125 => (AnsiKeyCode::RightBracket, KeyModifier::Shift),
        126 => (AnsiKeyCode::AngleQuote, KeyModifier::Shift),
        127 => (AnsiKeyCode::_Backspace, KeyModifier::None),
        _ => (AnsiKeyCode::Unknown, KeyModifier::None),
    };
    Ok(AnsiKey {
        bytes: [c, 0, 0, 0, 0],
        code,
        modifier,
    })
}

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

        let event = MouseButtonEvent {
            button,
            x: i32::from(x) - 1, // coordinates start at 1 in the codes
            y: i32::from(y) - 1,
        };

        Ok(AnsiKey {
            bytes: [27, 91, 77, button_code, 0],
            code: if is_motion_event {
                AnsiKeyCode::MouseMove(event)
            } else {
                AnsiKeyCode::MouseButton(event)
            },
            modifier: mouse_modifiers(i32::from(button_code)),
        })
    }

    /// Reads one SGR mouse report (`CSI < Pb ; Px ; Py M/m`) from stdin.
    ///
    /// Bit 5 set means a move, for both `M` and `m`. A trailing `m` with that bit clear is a
    /// release, reported as [`MouseButton::None`] so the input layer turns it into a button-up.
    /// Wheel and extra-button reports are consumed and rejected so their digits are not parsed
    /// again as keystrokes.
    fn parse_sgr_mouse_event() -> Result<AnsiKey, TermiosError> {
        let mut payload = [0u8; 64];
        let mut len = 0usize;
        loop {
            if len == payload.len() {
                return Err(TermiosError::UnknownKey);
            }
            let byte = checked_stdin_read()?;
            match byte {
                b'0'..=b'9' | b';' | b'M' | b'm' => {
                    payload[len] = byte;
                    len += 1;
                    if byte == b'M' || byte == b'm' {
                        break;
                    }
                }
                _ => return Err(TermiosError::UnknownKey),
            }
        }
        decode_sgr_mouse(&payload[..len])
    }

    /// Bytes after `ESC [`, except SGR (`<`) and the legacy mouse introducer (`M`).
    fn parse_csi_sequence() -> Result<AnsiKey, TermiosError> {
        let mut payload = [0u8; 32];
        let mut len = 0usize;
        loop {
            if len == payload.len() {
                return Err(TermiosError::UnknownKey);
            }
            let byte = checked_stdin_read()?;
            if len == 0 {
                match byte {
                    b'<' => return Self::parse_sgr_mouse_event(),
                    b'M' => return Self::parse_mouse_event(),
                    _ => {}
                }
            }
            match byte {
                b'0'..=b'9' | b';' => {
                    payload[len] = byte;
                    len += 1;
                }
                0x40..=0x7E => {
                    payload[len] = byte;
                    len += 1;
                    break;
                }
                _ => return Err(TermiosError::UnknownKey),
            }
        }
        decode_csi(&payload[..len])
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
                                91 => return Self::parse_csi_sequence(),
                                0 => ([c, byte_2, 0, 0, 0], AnsiKeyCode::_Escape, modifier),
                                other => return Ok(add_alt(decode_plain_key(other)?)),
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
    let mut byte = 0u8;
    if unsafe { read(STDIN_FILENO, &mut byte as *mut u8 as *mut _, 1) } == -1 {
        return Err(TermiosError::ReadStdInFailed);
    }
    // println!("{}", byte);
    Ok(byte)
}

pub fn try_check_stdin_read() -> Option<u8> {
    let mut byte = 0u8;
    let mut fds = PollFd {
        fd: STDIN_FILENO,
        events: POLLIN,
        revents: 0,
    };
    let ret = unsafe { poll(&mut fds, 1, 25) };
    if ret <= 0 {
        None
    } else {
        let res = unsafe { read(STDIN_FILENO, &mut byte as *mut u8 as *mut _, 1) };
        if res == -1 {
            None
        } else {
            Some(byte)
        }
    }
}
