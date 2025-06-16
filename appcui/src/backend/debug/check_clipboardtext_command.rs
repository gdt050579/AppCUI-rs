use super::command_parser::{CommandParser, ParserError};

pub(super) struct CheckClipboardTextCommand {
    text: String,
}

impl CheckClipboardTextCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() != 1 {
            return Err(ParserError::new("CheckClipboardText command must have one parameter !"));
        }

        Ok(Self {
            text: parser.get_string(0).unwrap(),
        })
    }
    pub(super) fn get_text(&self) -> &str {
        &self.text
    }
}
