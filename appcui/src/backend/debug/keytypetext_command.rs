use std::collections::VecDeque;

use crate::{
    input::Key,
    system::{KeyPressedEvent, SystemEvent},
};

use super::command_parser::{CommandParser, ParserError};

pub(super) struct KeyTypeTextCommand {
    text: String
}

impl KeyTypeTextCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        let pcount = parser.get_params_count();
        if pcount != 1 {
            return Err(ParserError::new(
                "Key.TypeText command requires one parameter (the text that needs to be entered)",
            ));
        }
        Ok(Self {
            text: parser.get_string(0).unwrap(),
        })
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>) {
        for ch in self.text.chars() {
            sys_events.push_back(SystemEvent::KeyPressed(KeyPressedEvent {
                key: Key::from(ch), 
                character: ch,
            }));           
        }
    }
}
