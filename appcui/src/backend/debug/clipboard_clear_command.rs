use super::command_parser::{CommandParser, ParserError};

pub(super) struct ClipboardClearCommand {}

impl ClipboardClearCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 0 {
            return Err(ParserError::new("Clipboard.ClearText command does not have any parameters !"));
        }
        Ok(Self {})
    }
}
