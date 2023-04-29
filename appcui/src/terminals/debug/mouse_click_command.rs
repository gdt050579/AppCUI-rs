use std::collections::VecDeque;

use crate::{input::MouseButton, terminals::{SystemEvent, MouseButtonDownEvent, MouseButtonUpEvent}};

use super::command_parser::{CommandParser, ParserError};

pub(super) struct MouseClickCommand {
    x: i32,
    y: i32,
    button: MouseButton
}

impl MouseClickCommand {
    pub(super) fn new(parser: &CommandParser)->Result<Self,ParserError> {
        if parser.get_params_count()!=3 {
            return Err(ParserError::new("Mouse.Click command requires 3 parameters"));
        }
        let x = parser.get_i32(0);
        let y = parser.get_i32(1);
        let b = parser.get_mouse_button(2);
        if x.is_none() {
            return Err(ParserError::new("First parameter for Mouse.Click command should an integer (x value)"));
        }
        if y.is_none() {
            return Err(ParserError::new("Second parameter for Mouse.Click command should an integer (y value)"));
        }
        if b.is_none() {
            return Err(ParserError::new("Third parameter for Mouse.Click command should an mouse button (left, right or cente)"));
        }
        Ok(Self {
            x: x.unwrap(),
            y: y.unwrap(),
            button: b.unwrap()
        })
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>) {
        sys_events.push_back(SystemEvent::MouseButtonDown(MouseButtonDownEvent {
            x: self.x,
            y: self.y,
            button: self.button,
        }));
        sys_events.push_back(SystemEvent::MouseButtonUp(MouseButtonUpEvent {
            x: self.x,
            y: self.y,
            button: MouseButton::None,
        }));
    }
}