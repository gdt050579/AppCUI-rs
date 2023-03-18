use AppCUIProcMacro::AppCUIControl;

use super::WindowFlags;
use crate::controls::events::*;
use crate::controls::*;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;


#[AppCUIControl(overwrite=OnPaint)]
pub struct Window {
    title: String,
    flags: WindowFlags,
}

impl Window {
    pub fn new(title: &str, layout: Layout, flags: WindowFlags) -> Self {
        Window {
            base: ControlBase::new(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            title: String::from(title),
            flags,
        }
    }
}
impl OnPaint for Window {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        
    }
}