use std::collections::VecDeque;

use crate::{graphics::Size, terminals::SystemEvent};

use super::command_parser::{CommandParser, ParserError};

pub(super) struct ResizeCommand {
    width: u32,
    height: u32,
}

impl ResizeCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 2 {
            return Err(ParserError::new("Resize command requires 32 parameters"));
        }
        let w = parser.get_i32(0);
        let h = parser.get_i32(1);
        if w.is_none() {
            return Err(ParserError::new(
                "First parameter for Resize command should the width (a numerical value)",
            ));
        }
        if h.is_none() {
            return Err(ParserError::new(
                "Second parameter for Resize command should the height (a numerical value)",
            ));
        }
        if (w.unwrap() < 5) || (w.unwrap() > 10000) {
            return Err(ParserError::new("Width must be between 5 and 10000"));
        }
        if (h.unwrap() < 5) || (h.unwrap() > 10000) {
            return Err(ParserError::new("Height must be between 5 and 10000"));
        }
        Ok(Self {
            width: w.unwrap() as u32,
            height: h.unwrap() as u32,
        })
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>) {
        sys_events.push_back(SystemEvent::Resize(Size {
            width: self.width,
            height: self.height,
        }));
    }
}
