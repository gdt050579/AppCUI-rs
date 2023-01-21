use AppCUIProcMacro::AppCUIControl;
use super::Layout;
use super::BasicControl;
use super::events::*;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;

#[AppCUIControl]
pub struct Desktop {}

impl Desktop {
    pub (crate) fn new() -> Self {
        Desktop {
            base: BasicControl::new(Layout::new("x:0,y:0,w:1,h:1")),
        }
    }
}
