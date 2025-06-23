use super::command_parser::{CommandParser, ParserError};
use crate::graphics::Point;

pub(super) struct CheckCursorCommand {
    point: Point,
}
impl CheckCursorCommand {
    pub(super) fn new(parser: &CommandParser) -> Result<Self, ParserError> {
        if parser.get_params_count() == 1 {
            let param = parser.get_param(0).unwrap_or("");
            if (param != "false") && (param != "hidden") {
                return Err(ParserError::new("CheckCursor can be used in two forms: CheckCursor(hidden) or CheckCursor(false) - to check if a cursor is hidden, or CheckCursor(x,y) to check if a cursor is at a specified position. In this case, CheckCursor it is used with one parameter that is neither 'false' nor 'hidden'"));
            }
            return Ok(Self { point: Point::new(-1,-1) });
        }
        if parser.get_params_count() == 2 {
            let x = parser.get_i32(0);
            let y = parser.get_i32(1);
            if x.is_none() {
                return Err(ParserError::new("Invalid numerical value for the 'x' coordonate of the cursor !"));
            }
            if y.is_none() {
                return Err(ParserError::new("Invalid numerical value for the 'y' coordonate of the cursor !"));
            }
            return Ok(Self {
                point: Point::new(x.unwrap(), y.unwrap()),
            });
        }
        Err(ParserError::new("CheckCursor can be used in two forms: CheckCursor(hidden) or CheckCursor(false) - to check if a cursor is hidden, or CheckCursor(x,y) to check if a cursor is at a specified position."))
    }
    pub(super) fn get_point(&self) -> Point {
        self.point
    }
}
