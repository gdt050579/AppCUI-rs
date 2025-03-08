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
        Key { code, modifier }
    }
    pub fn create_hotkey(character: char, modifier: KeyModifier) -> Key {
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
            Key {
                code: KeyCode::None,
                modifier: KeyModifier::None,
            }
        } else {
            Key { code, modifier }
        }
    }
    pub fn value(&self) -> u16 {
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
impl From<char> for Key {
    fn from(value: char) -> Self {
        match value {
            // capital
            'A' => Key::new(KeyCode::A, KeyModifier::Shift),
            'B' => Key::new(KeyCode::B, KeyModifier::Shift),
            'C' => Key::new(KeyCode::C, KeyModifier::Shift),
            'D' => Key::new(KeyCode::D, KeyModifier::Shift),
            'E' => Key::new(KeyCode::E, KeyModifier::Shift),
            'F' => Key::new(KeyCode::F, KeyModifier::Shift),
            'G' => Key::new(KeyCode::G, KeyModifier::Shift),
            'H' => Key::new(KeyCode::H, KeyModifier::Shift),
            'I' => Key::new(KeyCode::I, KeyModifier::Shift),
            'J' => Key::new(KeyCode::J, KeyModifier::Shift),
            'K' => Key::new(KeyCode::K, KeyModifier::Shift),
            'L' => Key::new(KeyCode::L, KeyModifier::Shift),
            'M' => Key::new(KeyCode::M, KeyModifier::Shift),
            'N' => Key::new(KeyCode::N, KeyModifier::Shift),
            'O' => Key::new(KeyCode::O, KeyModifier::Shift),
            'P' => Key::new(KeyCode::P, KeyModifier::Shift),
            'Q' => Key::new(KeyCode::Q, KeyModifier::Shift),
            'R' => Key::new(KeyCode::R, KeyModifier::Shift),
            'S' => Key::new(KeyCode::S, KeyModifier::Shift),
            'T' => Key::new(KeyCode::T, KeyModifier::Shift),
            'U' => Key::new(KeyCode::U, KeyModifier::Shift),
            'V' => Key::new(KeyCode::V, KeyModifier::Shift),
            'W' => Key::new(KeyCode::W, KeyModifier::Shift),
            'X' => Key::new(KeyCode::X, KeyModifier::Shift),
            'Y' => Key::new(KeyCode::Y, KeyModifier::Shift),
            'Z' => Key::new(KeyCode::Z, KeyModifier::Shift),
            // lower case letters
            'a' => Key::new(KeyCode::A, KeyModifier::None),
            'b' => Key::new(KeyCode::B, KeyModifier::None),
            'c' => Key::new(KeyCode::C, KeyModifier::None),
            'd' => Key::new(KeyCode::D, KeyModifier::None),
            'e' => Key::new(KeyCode::E, KeyModifier::None),
            'f' => Key::new(KeyCode::F, KeyModifier::None),
            'g' => Key::new(KeyCode::G, KeyModifier::None),
            'h' => Key::new(KeyCode::H, KeyModifier::None),
            'i' => Key::new(KeyCode::I, KeyModifier::None),
            'j' => Key::new(KeyCode::J, KeyModifier::None),
            'k' => Key::new(KeyCode::K, KeyModifier::None),
            'l' => Key::new(KeyCode::L, KeyModifier::None),
            'm' => Key::new(KeyCode::M, KeyModifier::None),
            'n' => Key::new(KeyCode::N, KeyModifier::None),
            'o' => Key::new(KeyCode::O, KeyModifier::None),
            'p' => Key::new(KeyCode::P, KeyModifier::None),
            'q' => Key::new(KeyCode::Q, KeyModifier::None),
            'r' => Key::new(KeyCode::R, KeyModifier::None),
            's' => Key::new(KeyCode::S, KeyModifier::None),
            't' => Key::new(KeyCode::T, KeyModifier::None),
            'u' => Key::new(KeyCode::U, KeyModifier::None),
            'v' => Key::new(KeyCode::V, KeyModifier::None),
            'w' => Key::new(KeyCode::W, KeyModifier::None),
            'x' => Key::new(KeyCode::X, KeyModifier::None),
            'y' => Key::new(KeyCode::Y, KeyModifier::None),
            'z' => Key::new(KeyCode::Z, KeyModifier::None),

            // numbers
            '0' => Key::new(KeyCode::N0, KeyModifier::None),
            '1' => Key::new(KeyCode::N1, KeyModifier::None),
            '2' => Key::new(KeyCode::N2, KeyModifier::None),
            '3' => Key::new(KeyCode::N3, KeyModifier::None),
            '4' => Key::new(KeyCode::N4, KeyModifier::None),
            '5' => Key::new(KeyCode::N5, KeyModifier::None),
            '6' => Key::new(KeyCode::N6, KeyModifier::None),
            '7' => Key::new(KeyCode::N7, KeyModifier::None),
            '8' => Key::new(KeyCode::N8, KeyModifier::None),
            '9' => Key::new(KeyCode::N9, KeyModifier::None),

            // simbols over bnumbers
            ')' => Key::new(KeyCode::N0, KeyModifier::Shift),
            '!' => Key::new(KeyCode::N1, KeyModifier::Shift),
            '@' => Key::new(KeyCode::N2, KeyModifier::Shift),
            '#' => Key::new(KeyCode::N3, KeyModifier::Shift),
            '$' => Key::new(KeyCode::N4, KeyModifier::Shift),
            '%' => Key::new(KeyCode::N5, KeyModifier::Shift),
            '^' => Key::new(KeyCode::N6, KeyModifier::Shift),
            '&' => Key::new(KeyCode::N7, KeyModifier::Shift),
            '*' => Key::new(KeyCode::N8, KeyModifier::Shift),
            '(' => Key::new(KeyCode::N9, KeyModifier::Shift),

            // other combinations
            ' ' => Key::new(KeyCode::Space, KeyModifier::None),
            '\n' => Key::new(KeyCode::Enter, KeyModifier::None),
            '\t' => Key::new(KeyCode::Tab, KeyModifier::None),

            // default
            _ => {
                Key::None
            }
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.modifier.name(), self.code.name())
    }
}
