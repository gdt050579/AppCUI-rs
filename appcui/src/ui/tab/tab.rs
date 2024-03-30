use crate::prelude::*;
use crate::ui::tab::Type;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct Tab {
    
}

impl OnPaint for Tab {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
