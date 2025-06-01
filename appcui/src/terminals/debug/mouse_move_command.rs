use std::collections::VecDeque;

use crate::{
    input::MouseButton,
    terminals::{MouseMoveEvent, SystemEvent},
};

use super::command_parser::{CommandParser, ParserError};

pub(super) struct MouseMoveCommand {
    x: i32,
    y: i32,
}

impl MouseMoveCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 2 {
            return Err(ParserError::new("Mouse.Move command requires 2 parameters"));
        }
        let x = parser.get_i32(0);
        let y = parser.get_i32(1);
        if x.is_none() {
            return Err(ParserError::new("First parameter for Mouse.Move command should an integer (x value)"));
        }
        if y.is_none() {
            return Err(ParserError::new("Second parameter for Mouse.Move command should an integer (y value)"));
        }
        Ok(Self {
            x: x.unwrap(),
            y: y.unwrap(),
        })
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>) {
        sys_events.push_back(SystemEvent::MouseMove(MouseMoveEvent {
            x: self.x,
            y: self.y,
            button: MouseButton::None,
        }));
    }
}
