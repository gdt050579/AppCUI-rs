use super::{
    command_parser::{CommandParser, ParserError},
    mouse_click_command::MouseClickCommand,
    mouse_hold_command::MouseHoldCommand,
    mouse_move_command::MouseMoveCommand,
    mouse_release_command::MouseReleaseCommand, mouse_drag_command::MouseDragCommand,
};

pub(super) enum Command {
    MouseHold(MouseHoldCommand),
    MouseRelease(MouseReleaseCommand),
    MouseClick(MouseClickCommand),
    MouseMove(MouseMoveCommand),
    MouseDrag(MouseDragCommand)
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
            _ => {
                let mut s = String::from("Invalid/Unknwon command: ");
                s += cp.get_command();
                return Err(ParserError::new(&s));
            }
        }
    }
}
