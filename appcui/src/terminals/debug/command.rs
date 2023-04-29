use std::collections::{VecDeque};

use crate::terminals::SystemEvent;

use super::{
    command_parser::{CommandParser, ParserError},
    mouse_click_command::MouseClickCommand,
    mouse_hold_command::MouseHoldCommand,
    mouse_move_command::MouseMoveCommand,
    mouse_release_command::MouseReleaseCommand, mouse_drag_command::MouseDragCommand, mouse_wheel_command::MouseWheelCommand,
};

pub(super) enum Command {
    MouseHold(MouseHoldCommand),
    MouseRelease(MouseReleaseCommand),
    MouseClick(MouseClickCommand),
    MouseMove(MouseMoveCommand),
    MouseDrag(MouseDragCommand),
    MouseWheel(MouseWheelCommand)
}
impl Command {
    pub(super) fn new(text: &str) -> Result<Command, ParserError> {
        let cp = CommandParser::new(text)?;
        match cp.get_command() {
            "Mouse.Hold" => {
                let variant = MouseHoldCommand::new(&cp)?;
                return Ok(Command::MouseHold(variant));
            }
            "Mouse.Release" => {
                let variant = MouseReleaseCommand::new(&cp)?;
                return Ok(Command::MouseRelease(variant));
            }
            "Mouse.Click" => {
                let variant = MouseClickCommand::new(&cp)?;
                return Ok(Command::MouseClick(variant));
            }
            "Mouse.Move" => {
                let variant = MouseMoveCommand::new(&cp)?;
                return Ok(Command::MouseMove(variant));
            }
            "Mouse.Drag" => {
                let variant = MouseDragCommand::new(&cp)?;
                return Ok(Command::MouseDrag(variant));
            }
            "Mouse.Wheel" => {
                let variant = MouseWheelCommand::new(&cp)?;
                return Ok(Command::MouseWheel(variant));
            }
            _ => {
                let mut s = String::from("Invalid/Unknwon command: ");
                s += cp.get_command();
                return Err(ParserError::new(&s));
            }
        }
    }
    pub(super) fn generate_event(&self, sys_events: &mut VecDeque<SystemEvent>) {
        match self {
            Command::MouseHold(cmd) => cmd.generate_event(sys_events),
            Command::MouseRelease(cmd) => cmd.generate_event(sys_events),
            Command::MouseClick(cmd) => cmd.generate_event(sys_events),
            Command::MouseMove(cmd) => cmd.generate_event(sys_events),
            Command::MouseDrag(cmd) => cmd.generate_event(sys_events),
            Command::MouseWheel(cmd) => cmd.generate_event(sys_events),
        }
    }
}
