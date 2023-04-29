use super::command_parser::{CommandParser, ParserError};

pub(super) struct PaintCommand {
    title: String,
}

impl PaintCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() <= 1 {
            return Err(ParserError::new("Paint command has one or no parameters !"));
        }
        if parser.get_params_count() == 1 {
            Ok(Self {
                title: String::from(parser.get_param(0).unwrap()),
            })
        } else {
            Ok(Self {
                title: String::new(),
            })
        }
    }
    pub(super) fn get_title(&self) -> &str {
        &self.title
    }
}
