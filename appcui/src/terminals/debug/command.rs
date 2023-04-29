use super::{
    command_parser::{CommandParser, ParserError},
    mouse_hold_command::MouseHoldCommand,
    mouse_release_command::MouseReleaseCommand,
};

pub(super) enum Command {
    MouseHold(MouseHoldCommand),
    MouseRelease(MouseReleaseCommand),
}
impl Command {
    pub(super) fn new(text: &str) -> Result<Command, ParserError> {
        let cp = CommandParser::new(text)?;
        match cp.get_command() {
            "Mouse.Hold" => {
                let variant = MouseHoldCommand::new(&cp)?;
                return Ok(Command::MouseHold(variant));
            },
            "Mouse.Release" => {
                let variant = MouseReleaseCommand::new(&cp)?;
                return Ok(Command::MouseRelease(variant));
            }
            _ => {
                let mut s = String::from("Invalid/Unknwon command: ");
                s += cp.get_command();
                return Err(ParserError::new(&s));
            }
        }
    }
}
