use std::collections::VecDeque;

use crate::{input::MouseButton, system::{SystemEvent, MouseButtonUpEvent, MouseButtonDownEvent, MouseMoveEvent}};

use super::command_parser::{CommandParser, ParserError};

pub(super) struct MouseDragCommand {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl MouseDragCommand {
    pub(super) fn new(parser: &CommandParser)->Result<Self,ParserError> {
        if parser.get_params_count()!=4 {
            return Err(ParserError::new("Mouse.Drag command requires 4 parameters"));
        }
        let x1 = parser.get_i32(0);
        let y1 = parser.get_i32(1);
        let x2 = parser.get_i32(2);
        let y2 = parser.get_i32(3);
        if x1.is_none() {
            return Err(ParserError::new("First parameter for Mouse.Drag command should an integer (x value) - starting point"));
        }
        if y1.is_none() {
            return Err(ParserError::new("Second parameter for Mouse.Drag command should an integer (y value) - starting point"));
        }
        if x2.is_none() {
            return Err(ParserError::new("Third parameter for Mouse.Drag command should an integer (x value) - ending point"));
        }
        if y2.is_none() {
            return Err(ParserError::new("Fourth parameter for Mouse.Drag command should an integer (y value) - ending point"));
        }
        Ok(Self {
            x1: x1.unwrap(),
            y1: y1.unwrap(),
            x2: x2.unwrap(),
            y2: y2.unwrap(),
        })
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>) {
        sys_events.push_back(SystemEvent::MouseButtonDown(MouseButtonDownEvent {
            x: self.x1,
            y: self.y1,
            button: MouseButton::Left,
        }));
        sys_events.push_back(SystemEvent::MouseMove(MouseMoveEvent {
            x: self.x2,
            y: self.y2,
            button: MouseButton::Left,
        }));
        sys_events.push_back(SystemEvent::MouseButtonUp(MouseButtonUpEvent {
            x: self.x2,
            y: self.y2,
            button: MouseButton::None,
        }));
    }
}