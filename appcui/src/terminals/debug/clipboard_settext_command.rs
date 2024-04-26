use super::command_parser::{CommandParser, ParserError};

pub(super) struct ClipboardSetTextCommand {
    text: String,
}

impl ClipboardSetTextCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 1 {
            return Err(ParserError::new("Clipboard.SetText command must have one parameter !"));
        }

        Ok(Self {
            text: String::from(parser.get_param(0).unwrap()),
        })
    }
    pub(super) fn get_text(&self) -> &str {
        &self.text
    }
}
