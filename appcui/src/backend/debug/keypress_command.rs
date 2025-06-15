use std::collections::VecDeque;

use crate::{
    input::{Key, KeyCode, KeyModifier},
    system::{KeyPressedEvent, SystemEvent},
};

use super::command_parser::{CommandParser, ParserError};

pub(super) struct KeyPressedCommand {
    key: Key,
    times: u32,
}

impl KeyPressedCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        let pcount = parser.get_params_count();
        if (pcount != 1) && (pcount != 2) {
            return Err(ParserError::new(
                "KeyPress command requires one or two parameters",
            ));
        }
        let k = parser.get_key(0);
        if k.is_none() {
            return Err(ParserError::new(
                "First parameter for KeyPress must a know key or key combination",
            ));
        }
        if pcount == 2 {
            let t = parser.get_i32(1);

            if t.is_none() {
                return Err(ParserError::new(
                "Second parameter for KeyPress (if present) is the number of times (must be a numerical value)",
            ));
            }
            if t.unwrap() < 1 {
                return Err(ParserError::new(
                    "Number of times a key should be send must be a positive (>=1) number",
                ));
            }
            Ok(Self {
                key: k.unwrap(),
                times: t.unwrap() as u32,
            })
        } else {
            Ok(Self {
                key: k.unwrap(),
                times: 1,
            })
        }
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>) {
        let mut ch = '\0';
        if self.key.modifier == KeyModifier::None {
            if ((self.key.code as u8) >= (KeyCode::A as u8))
                && ((self.key.code as u8) <= (KeyCode::Z as u8))
            {
                ch = ((('a' as u32) + (self.key.code as u32) - (KeyCode::A as u32)) as u8) as char;
            }
            if ((self.key.code as u8) >= (KeyCode::N0 as u8))
                && ((self.key.code as u8) <= (KeyCode::N9 as u8))
            {
                ch = ((('0' as u32) + (self.key.code as u32) - (KeyCode::N0 as u32)) as u8) as char;
            }
        }
        for _ in 0..self.times {
            sys_events.push_back(SystemEvent::KeyPressed(KeyPressedEvent {
                key: self.key,
                character: ch,
            }));
        }
    }
}
