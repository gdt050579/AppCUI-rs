use std::fmt::Display;

use super::KeyCode;
use super::KeyModifier;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Key {
    pub code: KeyCode,
    pub modifier: KeyModifier,
}

impl Key {
    #[allow(non_upper_case_globals)]
    pub const None: Key = Key {
        code: KeyCode::None,
        modifier: KeyModifier::None,
    };
    pub fn new(code: KeyCode, modifier: KeyModifier) -> Key {
        Key {
            code: code,
            modifier: modifier,
        }
    }
    pub fn from_char(character: char, modifier: KeyModifier) -> Key {
        let code = match character {
            'a' | 'A' => KeyCode::A,
            'b' | 'B' => KeyCode::B,
            'c' | 'C' => KeyCode::C,
            'd' | 'D' => KeyCode::D,
            'e' | 'E' => KeyCode::E,
            'f' | 'F' => KeyCode::F,
            'g' | 'G' => KeyCode::G,
            'h' | 'H' => KeyCode::H,
            'i' | 'I' => KeyCode::I,
            'j' | 'J' => KeyCode::J,
            'k' | 'K' => KeyCode::K,
            'l' | 'L' => KeyCode::L,
            'm' | 'M' => KeyCode::M,
            'n' | 'N' => KeyCode::N,
            'o' | 'O' => KeyCode::O,
            'p' | 'P' => KeyCode::P,
            'q' | 'Q' => KeyCode::Q,
            'r' | 'R' => KeyCode::R,
            's' | 'S' => KeyCode::S,
            't' | 'T' => KeyCode::T,
            'u' | 'U' => KeyCode::U,
            'v' | 'V' => KeyCode::V,
            'w' | 'W' => KeyCode::W,
            'x' | 'X' => KeyCode::X,
            'y' | 'Y' => KeyCode::Y,
            'z' | 'Z' => KeyCode::Z,
            '0' => KeyCode::N0,
            '1' => KeyCode::N1,
            '2' => KeyCode::N2,
            '3' => KeyCode::N3,
            '4' => KeyCode::N4,
            '5' => KeyCode::N5,
            '6' => KeyCode::N6,
            '7' => KeyCode::N7,
            '8' => KeyCode::N8,
            '9' => KeyCode::N9,
            _ => KeyCode::None,
        };
        if code == KeyCode::None {
            return Key {
                code: KeyCode::None,
                modifier: KeyModifier::None,
            };
        } else {
            return Key { code, modifier };
        }
    }
    pub fn get_compact_code(&self) -> u16 {
        ((self.code as u8) as u16) | ((self.modifier.get_value() as u16) << 8)
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            code: KeyCode::None,
            modifier: KeyModifier::None,
        }
    }
}
impl From<KeyCode> for Key {
    fn from(value: KeyCode) -> Self {
        Self {
            code: value,
            modifier: KeyModifier::None,
        }
    }
}
impl From<u16> for Key {
    fn from(value: u16) -> Self {
        let k = (value & 0xFF) as u8;
        let m = (value >> 8) as u8;
        if (k >= 64) || (m >= 8) {
            Key::None
        } else {
            Self {
                code: k.into(),
                modifier: m.into(),
            }
        }
    }
}
impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.modifier.name(), self.code.name())
    }
}
