use crate::prelude::*;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct ProgressBar {
    items_count: u64,
    items_processed: u64,
}
impl ProgressBar {
    pub fn new(items_count: u64, layout: Layout) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled),
            items_count,
            items_processed: 0 
        }
    }

}
impl OnPaint for ProgressBar {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {

    }
}
