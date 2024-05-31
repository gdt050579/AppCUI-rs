use std::collections::VecDeque;

use crate::{
    input::KeyModifier,
    terminals::{system_event::KeyModifierChangedEvent, SystemEvent},
};

use super::command_parser::{CommandParser, ParserError};

pub(super) struct KeyModifierCommand {
    modifier: KeyModifier,
}

impl KeyModifierCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        let pcount = parser.get_params_count();
        if pcount != 1 {
            return Err(ParserError::new(
                "KeyModifier command requires one parameter - a string that is a combination between `'Ctrl'`, `'Alt'`, `'Shift'` separated by `+` or `'None'`",
            ));
        }
        let k = parser.get_keymodifier(0);
        if let Some(modifier) = k {
            Ok(Self { modifier })
        } else {
            return Err(ParserError::new(
                "First parameter for KeyModifier is a not a valid combination (it must be a combination between `'Ctrl'`, `'Alt'`, `'Shift'` separated by `+` or `'None'`)",
            ));
        }
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>, old_state: KeyModifier) {
        sys_events.push_back(SystemEvent::KeyModifierChanged(KeyModifierChangedEvent {
            new_state: self.modifier,
            old_state,
        }));
    }
}
