use super::Flags;
use appcui_proc_macro::CustomControl;

#[CustomControl(overwrite = OnPaint, internal = true)]
pub struct UsageChart {
    flags: Flags,
}
impl UsageChart {
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::new(layout, false),
            flags,
        }
    }
}
impl OnPaint for UsageChart {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        
    }
}