use AppCUIProcMacro::AppCUIControl;
use super::Layout;
use super::BasicControl;

#[AppCUIControl]
pub struct Desktop {}

impl Desktop {
    pub (crate) fn new() -> Self {
        Desktop {
            base: BasicControl::new(Layout::new("x:0,y:0,w:1,h:1")),
        }
    }
}
