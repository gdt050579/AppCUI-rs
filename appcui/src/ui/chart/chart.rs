use crate::prelude::*;
//use crate::ui::chart::{events::EventData, Type};

#[CustomControl(overwrite = OnPaint, internal = true)]
pub struct Chart
{

}

impl Chart{
    pub fn new(layout: Layout) -> Self
    {
        let c = Self{
            base:  ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
        };
        c
    }
}

impl OnPaint for Chart
{
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("x,red,black"));
    }
}

